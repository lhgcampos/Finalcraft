use wordcraft_doc::props::{LineSpacing, Underline};
use wordcraft_doc::{Align, Block, Document, Paragraph, Path, ScriptParagraphKind, StoryRef};

use super::{read, write};

const SCRIPT: &[u8] = include_bytes!("../../tests/fixtures/screenplay-core.fdx");

fn para(doc: &Document, ix: usize) -> &Paragraph {
    doc.para(StoryRef::Body, &Path::top(ix)).unwrap()
}

fn with_content(body: &str) -> String {
    format!("<FinalDraft DocumentType=\"Script\" Version=\"6\"><Content>{body}</Content></FinalDraft>")
}

#[test]
fn preserves_roles_formatting_and_layout() {
    use ScriptParagraphKind::{Action, Character, Custom, Dialogue, General, Parenthetical, SceneHeading, Shot, Transition};
    let doc = read(SCRIPT).unwrap();
    let roles: Vec<_> = doc
        .body
        .iter()
        .map(|b| {
            let Block::Para(p) = &**b else { panic!("expected paragraph") };
            p.script.as_ref().unwrap().kind.clone()
        })
        .collect();
    assert_eq!(roles, [SceneHeading, Action, Character, Parenthetical, Dialogue, Shot, Transition, Custom("Nota".into()), General]);
    let first = para(&doc, 0);
    assert_eq!(first.script.as_ref().unwrap().source_id.as_deref(), Some("room"));
    assert_eq!(first.script.as_ref().unwrap().scene_number.as_deref(), Some("1A"));
    assert_eq!(first.props.indent_left, Some(36.0));
    assert_eq!(first.props.indent_right, Some(0.0));
    let action = para(&doc, 1);
    assert_eq!(action.text, "Luz & sombra. Sinal <azul>.");
    assert_eq!(action.props.line_spacing, Some(LineSpacing::Multiple(1.5)));
    assert_eq!(action.props.space_before, Some(12.0));
    let runs: Vec<_> = action.run_ranges().map(|(range, p)| (&action.text[range], p)).collect();
    assert_eq!(runs[0].0, "Luz & sombra. ");
    assert_eq!(runs[1].0, "Sinal <azul>.");
    assert_eq!(runs[1].1.bold, Some(true));
    assert_eq!(runs[1].1.italic, Some(true));
    assert_eq!(runs[1].1.font.as_deref(), Some("Courier Prime"));
    assert_eq!(runs[1].1.size, Some(12.0));
    assert_eq!(para(&doc, 6).runs[0].props.underline, Some(Underline::Single));
    assert_eq!(para(&doc, 6).props.align, Some(Align::Right));
    assert_eq!(para(&doc, 8).mark.font.as_deref(), Some("Courier Prime"));
    assert_eq!((doc.last_section.page_w, doc.last_section.page_h, doc.last_section.margin_top, doc.last_section.header), (612.0, 792.0, 90.0, 30.0));
    let bytes = write(&doc).unwrap();
    let reopened = read(&bytes).unwrap();
    assert_eq!(reopened, doc);
    assert_eq!(write(&reopened).unwrap(), bytes);
}

#[test]
fn refuses_unrepresented_content_at_each_level() {
    for (body, tag) in [
        ("<DualDialogue/>", "DualDialogue"),
        (
            "<Paragraph Type=\"General\"><DualDialogue><Paragraph Type=\"Dialogue\"><Text>salvo</Text></Paragraph></DualDialogue></Paragraph>",
            "DualDialogue",
        ),
        (
            "<Paragraph Type=\"Scene Heading\"><OmittedScene><Paragraph Type=\"Action\"><Text>salvo</Text></Paragraph></OmittedScene></Paragraph>",
            "OmittedScene",
        ),
        ("<Paragraph Type=\"Action\"><Text><HiddenText>salvo</HiddenText></Text></Paragraph>", "HiddenText"),
        ("<Paragraph Type=\"Action\"><Text Style=\"Unmapped\">salvo</Text></Paragraph>", "Unmapped"),
        ("<Paragraph Type=\"Action\" Unmapped=\"preserve\"><Text>salvo</Text></Paragraph>", "Unmapped"),
    ] {
        assert!(read(with_content(body).as_bytes()).unwrap_err().contains(tag));
    }
    let root = with_content("<Paragraph Type=\"General\"/>").replace("</FinalDraft>", "<LockedPages/></FinalDraft>");
    assert!(read(root.as_bytes()).unwrap_err().contains("LockedPages"));
    let mut doc = read(SCRIPT).unwrap();
    doc.core.title = "Would be lost".into();
    assert!(write(&doc).unwrap_err().contains("metadata"));
    let mut doc = read(SCRIPT).unwrap();
    doc.last_section.margin_left = 100.0;
    assert!(write(&doc).unwrap_err().contains("section settings"));
}

#[test]
fn strict_xml_preserves_whitespace_and_rejects_broken_or_oversized_input() {
    let input = with_content("<Paragraph Type=\"General\"><Text Font=\"A &amp; B\">  á&#13;\n\t<![CDATA[<&>]]>&#x1f4e1;  </Text></Paragraph>");
    let doc = read(input.as_bytes()).unwrap();
    assert_eq!(para(&doc, 0).text, "  á\r\n\t<&>📡  ");
    assert_eq!(read(&write(&doc).unwrap()).unwrap(), doc);
    for body in [
        "<Paragraph Type=\"General\"><Text>&#0;</Text></Paragraph>",
        "<Paragraph Type=\"General\"><Text>&unknown;</Text></Paragraph>",
        "<Paragraph Type=\"General\" Type=\"Action\"/>",
        "<Paragraph Type=\"Action\" Spacing=\"NaN\"/>",
        "<Paragraph Type=\"Action\" LeftIndent=\"8.5\" RightIndent=\"1\"/>",
        "<Paragraph Type=\"General\"><Text>broken",
        "<Paragraph Type=\"General\"><Text>\u{fffc}</Text></Paragraph>",
        "<Paragraph Type=\"<bad\"/>",
    ] {
        assert!(read(with_content(body).as_bytes()).is_err(), "{body}");
    }
    let doc = with_content("<Paragraph Type=\"General\"/>");
    assert!(read(format!("<!DOCTYPE FinalDraft>{doc}").as_bytes()).is_err());
    assert!(read(format!("{doc}<SecondRoot/>").as_bytes()).is_err());
    let body = format!("{}{}", "<a>".repeat(40), "</a>".repeat(40));
    assert!(read(with_content(&body).as_bytes()).unwrap_err().contains("depth"));
    let many = b"<a/>".repeat(super::xml::MAX_NODES);
    assert!(read(format!("<a>{}</a>", std::str::from_utf8(&many).unwrap()).as_bytes()).unwrap_err().contains("node"));
    assert!(read(&vec![b' '; super::xml::MAX_BYTES + 1]).unwrap_err().contains("8 MiB"));
}
