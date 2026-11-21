use wordcraft_doc::{Document, Paragraph, Run, ScriptParagraph, para_block};

use super::xml::{Node, parse};
use super::{Align, CharProps, LineSpacing, POINTS_PER_INCH, ParaProps, SectionProps, Underline, checked, error, kind, page_valid, para_edges};

/// Read a linear FDX 6 script. Unknown structures return an error before a document is returned.
pub fn read(bytes: &[u8]) -> Result<Document, String> {
    let root = parse(bytes)?;
    if root.name != "FinalDraft" {
        return Err(error("expected FinalDraft root"));
    }
    root.only_attrs(&["DocumentType", "Template", "Version"])?;
    if root.attr("DocumentType") != Some("Script") || root.attr("Version") != Some("6") || !matches!(root.attr("Template"), None | Some("No")) {
        return Err(error("expected a non-template Script with Version=6"));
    }
    root.container()?;
    let mut content = None;
    let mut layout = None;
    for node in root.children {
        match node.name.as_str() {
            "Content" if content.is_none() => content = Some(node),
            "PageLayout" if layout.is_none() => layout = Some(node),
            name => return Err(error(format_args!("unsupported or repeated {name} in FinalDraft"))),
        }
    }
    let content = content.ok_or_else(|| error("missing Content"))?;
    content.only_attrs(&[])?;
    content.container()?;
    let last_section = layout.as_ref().map(page).transpose()?.unwrap_or_default();
    let mut doc = Document::new();
    doc.body = content.children.into_iter().map(|node| paragraph(node, &last_section).map(para_block)).collect::<Result<_, _>>()?;
    doc.last_section = last_section;
    if doc.body.is_empty() {
        return Err(error("Content must contain at least one Paragraph"));
    }
    Ok(doc)
}

fn value(node: &Node, key: &str, scale: f64, low: f64, high: f64) -> Result<Option<f32>, String> {
    node.attr(key)
        .map(|s| {
            let number = s.parse::<f64>().map_err(|_| error(format_args!("invalid {key}")))?;
            checked(number * scale, key, low, high)
        })
        .transpose()
}

fn page(node: &Node) -> Result<SectionProps, String> {
    node.only_attrs(&["TopMargin", "BottomMargin", "HeaderMargin", "FooterMargin", "DocumentLeading"])?;
    node.container()?;
    if !matches!(node.attr("DocumentLeading"), None | Some("Normal")) {
        return Err(error("unsupported DocumentLeading"));
    }
    let mut page = SectionProps::default();
    match node.children.as_slice() {
        [] => {}
        [size] if size.name == "PageSize" => {
            size.only_attrs(&["Width", "Height"])?;
            size.container()?;
            size.leaf()?;
            page.page_w = value(size, "Width", POINTS_PER_INCH, 72.0, 2_880.0)?.ok_or_else(|| error("missing PageSize Width"))?;
            page.page_h = value(size, "Height", POINTS_PER_INCH, 72.0, 2_880.0)?.ok_or_else(|| error("missing PageSize Height"))?;
            page.landscape = page.page_w > page.page_h;
        }
        _ => return Err(error("PageLayout supports a single PageSize child")),
    }
    for (name, slot) in [
        ("TopMargin", &mut page.margin_top),
        ("BottomMargin", &mut page.margin_bottom),
        ("HeaderMargin", &mut page.header),
        ("FooterMargin", &mut page.footer),
    ] {
        if let Some(v) = value(node, name, 1.0, 0.0, page.page_h.into())? {
            *slot = v;
        }
    }
    page_valid(&page)?;
    Ok(page)
}

fn paragraph(node: Node, page: &SectionProps) -> Result<Paragraph, String> {
    if node.name != "Paragraph" {
        return Err(error(format_args!("unsupported {} inside Content", node.name)));
    }
    // Check the hierarchy first so the error names compound content even when its wrapper has extra attributes.
    if let Some(child) = node.children.iter().find(|n| n.name != "Text") {
        return Err(error(format_args!("unsupported {} inside Paragraph", child.name)));
    }
    node.only_attrs(&[
        "Type",
        "id",
        "Number",
        "Alignment",
        "FirstIndent",
        "LeftIndent",
        "RightIndent",
        "SpaceBefore",
        "Spacing",
        "Leading",
        "StartsNewPage",
    ])?;
    node.container()?;
    let script = ScriptParagraph {
        kind: kind(node.attr("Type").ok_or_else(|| error("missing paragraph Type"))?)?,
        source_id: node.attr("id").map(str::to_owned),
        scene_number: node.attr("Number").map(str::to_owned),
    };
    let mut p = Paragraph { props: paragraph_props(&node, page)?, script: Some(script), ..Paragraph::new() };
    let only_text = node.children.len() == 1;
    for t in node.children {
        t.leaf()?;
        let props = text_props(&t)?;
        if t.text.contains(wordcraft_doc::para::OBJ) {
            return Err(error("unsupported object replacement character"));
        }
        if !t.text.is_empty() {
            p.runs.push(Run { len: t.text.len(), props });
            p.text.push_str(&t.text);
        } else if only_text {
            p.mark = props;
        } else if !props.is_empty() {
            return Err(error("styled empty Text among multiple runs"));
        }
    }
    p.normalize();
    Ok(p)
}

fn paragraph_props(node: &Node, page: &SectionProps) -> Result<ParaProps, String> {
    let align = match node.attr("Alignment") {
        None => None,
        Some("Left") => Some(Align::Left),
        Some("Center") => Some(Align::Center),
        Some("Right") => Some(Align::Right),
        Some("Justify") => Some(Align::Justify),
        Some(_) => return Err(error("unsupported Alignment")),
    };
    let page_break_before = match node.attr("StartsNewPage") {
        None => None,
        Some("Yes") => Some(true),
        Some("No") => Some(false),
        Some(_) => return Err(error("invalid StartsNewPage")),
    };
    if !matches!(node.attr("Leading"), None | Some("Regular")) {
        return Err(error("unsupported Leading"));
    }
    let width = f64::from(page.page_w);
    let props = ParaProps {
        align,
        indent_left: value(node, "LeftIndent", POINTS_PER_INCH, 0.0, width)?.map(|left| left - page.margin_left),
        indent_right: value(node, "RightIndent", POINTS_PER_INCH, 0.0, width)?.map(|right| page.page_w - page.margin_right - right),
        indent_first: value(node, "FirstIndent", POINTS_PER_INCH, -width, width)?,
        space_before: value(node, "SpaceBefore", 1.0, 0.0, page.page_h.into())?,
        line_spacing: value(node, "Spacing", 1.0, 0.1, 10.0)?.map(LineSpacing::Multiple),
        page_break_before,
        ..Default::default()
    };
    para_edges(&props, page)?;
    Ok(props)
}

fn text_props(node: &Node) -> Result<CharProps, String> {
    node.only_attrs(&["Font", "Size", "Style", "RevisionID", "AdornmentStyle"])?;
    if !matches!(node.attr("RevisionID"), None | Some("0")) || !matches!(node.attr("AdornmentStyle"), None | Some("0")) {
        return Err(error("unsupported Text revision or adornment"));
    }
    let mut p = CharProps { font: node.attr("Font").map(str::to_owned), size: value(node, "Size", 1.0, 1.0, 512.0)?, ..Default::default() };
    if let Some(style) = node.attr("Style").filter(|s| !s.is_empty()) {
        for style in style.split('+') {
            match style {
                "Bold" => p.bold = Some(true),
                "Italic" => p.italic = Some(true),
                "Underline" => p.underline = Some(Underline::Single),
                "Strikeout" => p.strike = Some(true),
                "AllCaps" => p.caps = Some(true),
                _ => return Err(error(format_args!("unsupported Text Style {style}"))),
            }
        }
    }
    Ok(p)
}
