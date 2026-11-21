use std::io::{self, Write};

use quick_xml::Writer;
use quick_xml::escape::escape;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use wordcraft_doc::{Block, Document, Paragraph};

use super::xml::{MAX_BYTES, MAX_NODES, valid_xml};
use super::{
    CharProps, LineSpacing, POINTS_PER_INCH, SectionProps, Underline, align_name, char_subset, checked, error, kind, kind_name, page_valid,
    para_edges, para_subset,
};

#[derive(Default)]
struct Output(Vec<u8>);

impl Write for Output {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.0.len().saturating_add(bytes.len()) > MAX_BYTES {
            return Err(io::Error::other("file exceeds 8 MiB"));
        }
        self.0.extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

struct Xml {
    writer: Writer<Output>,
    nodes: usize,
}

impl Xml {
    fn event(&mut self, event: Event<'_>) -> Result<(), String> {
        if matches!(event, Event::Start(_) | Event::Empty(_)) {
            self.nodes += 1;
            if self.nodes > MAX_NODES {
                return Err(error("XML node limit exceeded"));
            }
        }
        self.writer.write_event(event).map_err(error)
    }

    fn end(&mut self, name: &str) -> Result<(), String> {
        self.event(Event::End(BytesEnd::new(name)))
    }
}

/// Save the supported subset. The caller receives bytes only after the entire script validates.
pub fn write(doc: &Document) -> Result<Vec<u8>, String> {
    if doc.body.is_empty() || doc.body.len() >= MAX_NODES {
        return Err(error("expected at least one Paragraph within the XML node limit"));
    }
    check_document(doc)?;
    let mut xml = Xml { writer: Writer::new_with_indent(Output::default(), b' ', 2), nodes: 0 };
    xml.event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;
    let mut root = BytesStart::new("FinalDraft");
    root.extend_attributes([("DocumentType", "Script"), ("Template", "No"), ("Version", "6")]);
    xml.event(Event::Start(root))?;
    xml.event(Event::Start(BytesStart::new("Content")))?;
    for block in &doc.body {
        let Block::Para(p) = &**block else { return Err(error("unsupported table")) };
        write_paragraph(&mut xml, p, &doc.last_section)?;
    }
    xml.end("Content")?;
    write_page(&mut xml, &doc.last_section)?;
    xml.end("FinalDraft")?;
    Ok(xml.writer.into_inner().0)
}

fn check_document(doc: &Document) -> Result<(), String> {
    let default = Document::new();
    let mut settings = doc.settings.clone();
    // The app stamps view zoom and these save-history fields even when the target has no
    // representation for them. Compare the document's actual settings and author metadata.
    settings.zoom = default.settings.zoom;
    let mut core = doc.core.clone();
    core.created.clear();
    core.modified.clear();
    core.last_modified_by.clear();
    core.revision = default.core.revision;

    if !doc.parts.is_empty()
        || !doc.comments.is_empty()
        || !doc.revisions.is_empty()
        || !doc.media.is_empty()
        || !doc.passthrough.is_empty()
        || !doc.glossary.is_empty()
        || !doc.sources.is_empty()
        || !doc.custom_props.is_empty()
        || !doc.footnote_separators.is_empty()
        || !doc.endnote_separators.is_empty()
        || doc.styles != default.styles
        || doc.numbering != default.numbering
        || settings != default.settings
        || core != default.core
    {
        return Err(error("unsupported document settings, metadata or content outside the script body"));
    }
    Ok(())
}

fn write_page(xml: &mut Xml, p: &SectionProps) -> Result<(), String> {
    page_valid(p)?;
    let supported = SectionProps {
        page_w: p.page_w,
        page_h: p.page_h,
        landscape: p.page_w > p.page_h,
        margin_top: p.margin_top,
        margin_bottom: p.margin_bottom,
        header: p.header,
        footer: p.footer,
        ..Default::default()
    };
    if *p != supported {
        return Err(error("unsupported section settings"));
    }
    let mut layout = BytesStart::new("PageLayout");
    for (name, points) in [("TopMargin", p.margin_top), ("BottomMargin", p.margin_bottom), ("HeaderMargin", p.header), ("FooterMargin", p.footer)] {
        number(&mut layout, name, points, 1.0);
    }
    xml.event(Event::Start(layout))?;
    let mut size = BytesStart::new("PageSize");
    number(&mut size, "Width", p.page_w, POINTS_PER_INCH);
    number(&mut size, "Height", p.page_h, POINTS_PER_INCH);
    xml.event(Event::Empty(size))?;
    xml.end("PageLayout")
}

fn number(tag: &mut BytesStart<'_>, key: &str, value: f32, scale: f64) {
    tag.push_attribute((key, (f64::from(value) / scale).to_string().as_str()));
}

fn attribute(tag: &mut BytesStart<'_>, key: &str, value: &str) -> Result<(), String> {
    if value.len() > MAX_BYTES || !valid_xml(value) {
        return Err(error(format_args!("invalid {key} attribute")));
    }
    // Attribute whitespace is normalized by XML parsers unless emitted as character references.
    let escaped = escape(value).replace('\t', "&#9;").replace('\n', "&#10;").replace('\r', "&#13;");
    tag.push_attribute((key.as_bytes(), escaped.as_bytes()));
    Ok(())
}

fn paragraph_tag(p: &Paragraph, page: &SectionProps) -> Result<BytesStart<'static>, String> {
    let mut tag = BytesStart::new("Paragraph");
    let name = p.script.as_ref().map_or("General", |s| kind_name(&s.kind));
    kind(name)?;
    attribute(&mut tag, "Type", name)?;
    if let Some(s) = &p.script {
        for (key, value) in [("id", &s.source_id), ("Number", &s.scene_number)] {
            if let Some(value) = value {
                attribute(&mut tag, key, value)?;
            }
        }
    }
    if p.props != para_subset(&p.props) {
        return Err(error("unsupported paragraph formatting"));
    }
    if let Some(value) = p.props.align {
        tag.push_attribute(("Alignment", align_name(value)?));
    }
    let (left, right) = para_edges(&p.props, page)?;
    let width = f64::from(page.page_w);
    for (name, value, scale, low, high) in [
        ("LeftIndent", left, POINTS_PER_INCH, 0.0, width),
        ("RightIndent", right, POINTS_PER_INCH, 0.0, width),
        ("FirstIndent", p.props.indent_first, POINTS_PER_INCH, -width, width),
        ("SpaceBefore", p.props.space_before, 1.0, 0.0, f64::from(page.page_h)),
    ] {
        if let Some(value) = value {
            checked(value.into(), name, low, high)?;
            number(&mut tag, name, value, scale);
        }
    }
    if let Some(spacing) = p.props.line_spacing {
        let LineSpacing::Multiple(n) = spacing else { return Err(error("unsupported line spacing rule")) };
        checked(n.into(), "Spacing", 0.1, 10.0)?;
        number(&mut tag, "Spacing", n, 1.0);
    }
    if let Some(before) = p.props.page_break_before {
        tag.push_attribute(("StartsNewPage", if before { "Yes" } else { "No" }));
    }
    Ok(tag)
}

fn write_paragraph(xml: &mut Xml, p: &Paragraph, page: &SectionProps) -> Result<(), String> {
    if p.section.is_some() || !p.objects.is_empty() || p.text.contains(wordcraft_doc::para::OBJ) {
        return Err(error("unsupported section break or inline object"));
    }
    xml.event(Event::Start(paragraph_tag(p, page)?))?;
    let mut start: usize = 0;
    for run in &p.runs {
        let end = start.saturating_add(run.len);
        let text = p.text.get(start..end).filter(|s| !s.is_empty()).ok_or_else(|| error("invalid text run boundaries"))?;
        write_text(xml, text, &run.props)?;
        start = end;
    }
    if start != p.text.len() {
        return Err(error("text runs do not cover the paragraph"));
    }
    if p.text.is_empty() {
        write_text(xml, "", &p.mark)?;
    }
    xml.end("Paragraph")
}

fn write_text(xml: &mut Xml, text: &str, props: &CharProps) -> Result<(), String> {
    if text.len() > MAX_BYTES || !valid_xml(text) {
        return Err(error("Text contains an unsupported character or exceeds 8 MiB"));
    }
    if *props != char_subset(props) || !matches!(props.underline, None | Some(Underline::None | Underline::Single)) {
        return Err(error("unsupported Text formatting"));
    }
    let mut tag = BytesStart::new("Text");
    if let Some(font) = &props.font {
        attribute(&mut tag, "Font", font)?;
    }
    if let Some(size) = props.size {
        checked(size.into(), "Size", 1.0, 512.0)?;
        number(&mut tag, "Size", size, 1.0);
    }
    let styles = [
        (props.bold == Some(true), "Bold"),
        (props.italic == Some(true), "Italic"),
        (props.underline == Some(Underline::Single), "Underline"),
        (props.strike == Some(true), "Strikeout"),
        (props.caps == Some(true), "AllCaps"),
    ];
    let style = styles.into_iter().filter_map(|(on, name)| on.then_some(name)).collect::<Vec<_>>().join("+");
    if !style.is_empty() {
        tag.push_attribute(("Style", style.as_str()));
    }
    if text.is_empty() {
        return xml.event(Event::Empty(tag));
    }
    xml.event(Event::Start(tag))?;
    xml.event(Event::Text(BytesText::from_escaped(escape(text).replace('\r', "&#13;"))))?;
    xml.end("Text")
}
