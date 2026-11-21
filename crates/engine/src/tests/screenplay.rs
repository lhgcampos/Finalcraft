use serde_json::json;
use wordcraft_doc::{Document, Path, ScriptParagraphKind, StoryRef};

use crate::{Session, io};

const FDX: &[u8] = br#"<FinalDraft DocumentType="Script" Version="6"><Content>
  <Paragraph Type="Scene Heading" id="h" Number="2"><Text>INT. RADIO</Text></Paragraph>
  <Paragraph Type="Dialogue"><Text Font="Courier Prime" Size="12" Style="Bold">Alguem?</Text></Paragraph>
</Content><PageLayout TopMargin="96"><PageSize Width="8.5" Height="11"/></PageLayout></FinalDraft>"#;

#[test]
fn screenplay_fdx_open_edit_save_reopen() {
    let mut session = Session::new(io::open_bytes("sample.FDX", FDX).unwrap());
    session.run("caret.docEnd", &json!({})).unwrap();
    session.run("text.insert", &json!({"text": " Sim."})).unwrap();
    session.run("edit.undo", &json!({})).unwrap();
    assert_eq!(session.doc.plain_text(StoryRef::Body), "INT. RADIO\nAlguem?");
    session.run("edit.redo", &json!({})).unwrap();
    session.stamp_save();
    let reopened = io::open_bytes("edited.fdx", &io::save_bytes("edited.fdx", &session.doc).unwrap()).unwrap();
    assert_eq!(reopened.body, session.doc.body);
    assert_eq!(reopened.last_section, session.doc.last_section);
    let p = reopened.para(StoryRef::Body, &Path::top(1)).unwrap();
    assert_eq!(p.text, "Alguem? Sim.");
    assert_eq!(p.script.as_ref().unwrap().kind, ScriptParagraphKind::Dialogue);
    assert!(p.runs.iter().all(|r| r.props.bold == Some(true)));
}

#[test]
fn screenplay_json_and_docx_compatibility() {
    let doc = io::open_bytes("before.json", br#"{"body":[{"kind":"para","text":"Old text","runs":[{"len":8}]}]}"#).unwrap();
    let p = doc.para(StoryRef::Body, &Path::top(0)).unwrap();
    assert_eq!(p.text, "Old text");
    assert!(p.script.is_none());
    let serialized = io::save_bytes("after.json", &doc).unwrap();
    assert!(!std::str::from_utf8(&serialized).unwrap().contains("\"script\""));
    let docx = io::save_bytes("old.docx", &doc).unwrap();
    assert_eq!(io::open_bytes("old.docx", &docx).unwrap().plain_text(StoryRef::Body), "Old text");

    let script = io::open_bytes("script.fdx", FDX).unwrap();
    let json = io::save_bytes("script.json", &script).unwrap();
    let mut reopened: Document = serde_json::from_slice(&json).unwrap();
    assert_eq!(reopened, script);
    let heading = reopened.para_mut(StoryRef::Body, &Path::top(0)).unwrap();
    let tail = heading.split_off(4).unwrap();
    assert_eq!(heading.script.as_ref().unwrap().source_id.as_deref(), Some("h"));
    let tail = tail.script.unwrap();
    assert_eq!(tail.kind, ScriptParagraphKind::SceneHeading);
    assert_eq!(tail.source_id, None);
    assert_eq!(tail.scene_number, None);
}
