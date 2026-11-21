# Finalcraft 01 · Screenplay FDX

The desktop app and CLI open and save UTF-8 **FDX 6** scripts with a linear body. The codec maps each element to an ordinary model paragraph with a screenplay role. The existing editor, undo history and layout handle text edits and direct formatting. Native JSON retains the role and the optional source ID and scene number. When a paragraph splits, both pieces keep the role and the first keeps the source ID and scene number.

| Area | Supported in this delivery |
|---|---|
| Roles | General, Scene Heading, Action, Character, Parenthetical, Dialogue, Transition, Shot, and custom names |
| Text | Unicode and XML entities, font, point size, bold, italic, single underline, strikeout and all caps |
| Paragraph | Alignment, indent edges, first indent, space before, line spacing multiplier, and page break before |
| Page | Width and height in inches, top and bottom margins and header and footer distances in points |

The reader rejects structures and settings it cannot represent, including `DualDialogue`, `OmittedScene`, `ElementSettings` and production metadata. This limits the release to basic files, and a typical file saved by Final Draft may have additional sections that block opening. The writer also refuses model content it cannot represent in FDX, such as tables, inline objects and unsupported styles. A failed encode leaves the destination file intact. Appearance uses the existing layout rules, and the app has no dedicated role picker.

## Reproduce open, edit, save and reopen

Run from the repository root. The source file is a synthetic script authored for this exercise.

```sh
demo_dir=$(mktemp -d)
cargo run -q -p wordcraft-cli -- run \
  --file crates/formats/tests/fixtures/screenplay-core.fdx \
  --cmd 'select.text={"text":"Ouvem-me?"}' \
  --cmd 'text.insert={"text":"Ouvem-me? Câmbio."}' \
  --save "$demo_dir/edited.fdx"
cargo run -q -p wordcraft-cli -- text "$demo_dir/edited.fdx"
```

The reopened text includes `Ouvem-me? Câmbio.`. In the desktop app, open the same fixture, replace that line in the document, choose **Save As** with an `.fdx` name, then close and reopen the saved copy. The format test checks roles, text, run formatting and page setup independently of display.
