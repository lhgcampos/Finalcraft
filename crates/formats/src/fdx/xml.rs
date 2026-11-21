//! Small bounded XML tree for the supported FDX hierarchy.

use std::collections::BTreeMap;

use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, XmlVersion};

pub(super) const MAX_BYTES: usize = 8 * 1024 * 1024;
pub(super) const MAX_NODES: usize = 100_000;
const MAX_DEPTH: usize = 16;
const MAX_ATTRS: usize = 32;

#[derive(Default)]
pub(super) struct Node {
    pub name: String,
    pub text: String,
    pub attrs: BTreeMap<String, String>,
    pub children: Vec<Self>,
}

pub(super) fn error(message: impl std::fmt::Display) -> String {
    format!("FDX: {message}")
}

pub(super) fn utf8(bytes: &[u8]) -> Result<&str, String> {
    std::str::from_utf8(bytes).map_err(error)
}

pub(super) fn valid_xml(s: &str) -> bool {
    s.chars().all(|c| matches!(c, '\t' | '\n' | '\r' | '\u{20}'..='\u{D7FF}' | '\u{E000}'..='\u{FFFD}' | '\u{10000}'..='\u{10FFFF}'))
}

impl Node {
    pub fn attr(&self, key: &str) -> Option<&str> {
        self.attrs.get(key).map(String::as_str)
    }

    pub fn only_attrs(&self, allowed: &[&str]) -> Result<(), String> {
        if let Some(key) = self.attrs.keys().find(|key| !allowed.contains(&key.as_str())) {
            return Err(error(format_args!("unsupported {} attribute {key}", self.name)));
        }
        Ok(())
    }

    pub fn container(&self) -> Result<(), String> {
        if !self.text.chars().all(|c| matches!(c, ' ' | '\n' | '\r' | '\t')) {
            return Err(error(format_args!("unexpected text in {}", self.name)));
        }
        Ok(())
    }

    pub fn leaf(&self) -> Result<(), String> {
        if let Some(child) = self.children.first() {
            return Err(error(format_args!("unsupported {} inside {}", child.name, self.name)));
        }
        Ok(())
    }
}

fn element(e: &BytesStart<'_>) -> Result<Node, String> {
    let mut node = Node { name: utf8(e.name().as_ref())?.to_owned(), ..Node::default() };
    for (i, attr) in e.attributes().enumerate() {
        if i == MAX_ATTRS {
            return Err(error("too many attributes"));
        }
        let attr = attr.map_err(error)?;
        if attr.value.contains(&b'<') {
            return Err(error("invalid XML attribute value"));
        }
        let value = attr.normalized_value(XmlVersion::Implicit1_0).map_err(error)?;
        if !valid_xml(&value) {
            return Err(error("invalid XML attribute value"));
        }
        node.attrs.insert(utf8(attr.key.as_ref())?.to_owned(), value.into_owned());
    }
    Ok(node)
}

fn append_text(stack: &mut [Node], text: &str) -> Result<(), String> {
    let node = stack.last_mut().ok_or_else(|| error("invalid XML nesting"))?;
    if !valid_xml(text) {
        return Err(error("invalid XML character"));
    }
    node.text.push_str(text);
    Ok(())
}

fn reference(r: quick_xml::events::BytesRef<'_>) -> Result<char, String> {
    match r.as_ref() {
        b"amp" => Ok('&'),
        b"lt" => Ok('<'),
        b"gt" => Ok('>'),
        b"quot" => Ok('"'),
        b"apos" => Ok('\''),
        _ => r.resolve_char_ref().map_err(error)?.ok_or_else(|| error("unsupported XML entity")),
    }
}

pub(super) fn parse(bytes: &[u8]) -> Result<Node, String> {
    if bytes.len() > MAX_BYTES {
        return Err(error("file exceeds 8 MiB"));
    }
    let mut reader = Reader::from_str(utf8(bytes)?);
    reader.config_mut().check_comments = true;
    reader.config_mut().expand_empty_elements = true;
    let mut stack = vec![Node::default()];
    let mut nodes = 0;
    let mut first_event = true;
    loop {
        match reader.read_event().map_err(error)? {
            Event::Start(e) => {
                nodes += 1;
                if nodes > MAX_NODES || stack.len() > MAX_DEPTH {
                    return Err(error("XML node or depth limit exceeded"));
                }
                stack.push(element(&e)?);
            }
            Event::End(_) => {
                let node = stack.pop().ok_or_else(|| error("invalid XML nesting"))?;
                stack.last_mut().ok_or_else(|| error("unexpected closing element"))?.children.push(node);
            }
            Event::Text(t) => append_text(&mut stack, &t.xml10_content().map_err(error)?)?,
            Event::CData(t) if stack.len() > 1 => append_text(&mut stack, &t.xml10_content().map_err(error)?)?,
            Event::GeneralRef(r) if stack.len() > 1 => {
                append_text(&mut stack, reference(r)?.encode_utf8(&mut [0; 4]))?;
            }
            Event::Comment(_) => {}
            Event::Decl(d) if first_event => {
                if d.version().map_err(error)?.as_ref() != b"1.0"
                    || d.encoding().transpose().map_err(error)?.is_some_and(|enc| !enc.eq_ignore_ascii_case(b"UTF-8"))
                {
                    return Err(error("expected XML 1.0 in UTF-8"));
                }
            }
            Event::Eof => break,
            _ => return Err(error("unsupported XML construct")),
        }
        first_event = false;
    }
    let mut outer = stack.pop().ok_or_else(|| error("missing document root"))?;
    if !stack.is_empty() || outer.children.len() != 1 {
        return Err(error("expected exactly one complete document root"));
    }
    outer.container()?;
    outer.children.pop().ok_or_else(|| error("missing document root"))
}
