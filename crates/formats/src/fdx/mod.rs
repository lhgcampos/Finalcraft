//! FDX 6 script interchange for linear paragraphs and basic page setup.
//!
//! Unsupported elements and attributes fail closed. In particular a compound dialogue or an
//! omitted scene cannot be flattened into the active script by this reader.

use wordcraft_doc::props::{LineSpacing, Underline};
use wordcraft_doc::{Align, CharProps, ParaProps, ScriptParagraphKind, SectionProps};

mod read;
mod write;
mod xml;

pub use read::read;
pub use write::write;

use xml::error;

const KINDS: &[(&str, ScriptParagraphKind)] = &[
    ("General", ScriptParagraphKind::General),
    ("Scene Heading", ScriptParagraphKind::SceneHeading),
    ("Action", ScriptParagraphKind::Action),
    ("Character", ScriptParagraphKind::Character),
    ("Parenthetical", ScriptParagraphKind::Parenthetical),
    ("Dialogue", ScriptParagraphKind::Dialogue),
    ("Transition", ScriptParagraphKind::Transition),
    ("Shot", ScriptParagraphKind::Shot),
];

const POINTS_PER_INCH: f64 = 72.0;

fn kind(name: &str) -> Result<ScriptParagraphKind, String> {
    if name.trim().is_empty() || name.len() > 128 || name.chars().any(char::is_control) {
        return Err(error("invalid paragraph Type"));
    }
    Ok(KINDS.iter().find(|(n, _)| *n == name).map_or_else(|| ScriptParagraphKind::Custom(name.to_owned()), |(_, k)| k.clone()))
}

fn kind_name(kind: &ScriptParagraphKind) -> &str {
    if let ScriptParagraphKind::Custom(name) = kind {
        return name;
    }
    KINDS.iter().find(|(_, k)| k == kind).map_or("General", |(name, _)| *name)
}

fn checked(value: f64, key: &str, low: f64, high: f64) -> Result<f32, String> {
    if (low..=high).contains(&value) { Ok(value as f32) } else { Err(error(format_args!("{key} out of range"))) }
}

fn page_valid(p: &SectionProps) -> Result<(), String> {
    checked(p.page_w.into(), "Width", 72.0, 2_880.0)?;
    checked(p.page_h.into(), "Height", 72.0, 2_880.0)?;
    for (key, value, max) in [
        ("TopMargin", p.margin_top, p.page_h),
        ("BottomMargin", p.margin_bottom, p.page_h),
        ("HeaderMargin", p.header, p.page_h),
        ("FooterMargin", p.footer, p.page_h),
    ] {
        checked(value.into(), key, 0.0, max.into())?;
    }
    if p.page_h - p.margin_top - p.margin_bottom < 18.0 || p.page_w - p.margin_left - p.margin_right < 18.0 {
        return Err(error("page margins leave no usable text area"));
    }
    Ok(())
}

fn align_name(value: Align) -> Result<&'static str, String> {
    match value {
        Align::Left => Ok("Left"),
        Align::Center => Ok("Center"),
        Align::Right => Ok("Right"),
        Align::Justify => Ok("Justify"),
        Align::Distribute => Err(error("unsupported distributed alignment")),
    }
}

/// FDX stores indent edges as positions from the paper edge, in inches.
fn para_edges(p: &ParaProps, page: &SectionProps) -> Result<(Option<f32>, Option<f32>), String> {
    let left = p.indent_left.map(|indent| page.margin_left + indent);
    let right = p.indent_right.map(|indent| page.page_w - page.margin_right - indent);
    let start = left.unwrap_or(page.margin_left);
    let end = right.unwrap_or(page.page_w - page.margin_right);
    checked(start.into(), "LeftIndent", 0.0, page.page_w.into())?;
    checked(end.into(), "RightIndent", 0.0, page.page_w.into())?;
    if end - start < 18.0 {
        return Err(error("paragraph indents leave no usable text area"));
    }
    Ok((left, right))
}

fn char_subset(p: &CharProps) -> CharProps {
    CharProps {
        font: p.font.clone(),
        size: p.size,
        bold: p.bold,
        italic: p.italic,
        underline: p.underline,
        strike: p.strike,
        caps: p.caps,
        ..Default::default()
    }
}

fn para_subset(p: &ParaProps) -> ParaProps {
    ParaProps {
        align: p.align,
        indent_left: p.indent_left,
        indent_right: p.indent_right,
        indent_first: p.indent_first,
        space_before: p.space_before,
        line_spacing: p.line_spacing,
        page_break_before: p.page_break_before,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests;
