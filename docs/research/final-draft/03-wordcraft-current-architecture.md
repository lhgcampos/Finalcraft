# 03 — Arquitetura atual do WordCraft relevante a roteiros

> Status: leitura do código do workspace de 2026-11-04, com implicações da observação comportamental de 2026-11-07; código funcional não modificado.
>
> Revisão: 2026-11-07. A leitura de código abaixo não foi revalidada como execução nesta rodada.

## Camadas e fluxo real

O workspace é Rust. A arquitetura documentada e confirmada divide-se em doc (modelo e primitivas de edição), fonts, layout, docx e formats (import/export), render e pdf, engine (Session, comandos, undo e dispatch), mcp, ui-egui e apps desktop/CLI/web. Ver [docs/architecture.md](../../architecture.md).

1. engine::io escolhe leitor por extensão/assinatura e produz doc::Document. DOCX passa por docx; DOC por docbin; XML é Flat OPC/Word 2003 XML; formatos simples passam por formats.
2. UI, CLI, controle e MCP invocam IDs pelo Session::run. O despachante verifica se o comando está registrado/habilitado, protege o documento e encapsula mutações; comandos falhos são revertidos e pânicos escapados viram erro.
3. Edição usa posições (story, path, off). Document::insert_text, split_paragraph, insert_block e remove_block são primitivas. Paragraph guarda String, runs, ParaProps, formatação de marca, objetos inline e metadado de seção.
4. layout::layout pagina o documento, resolve parágrafos, tabelas, cabeçalhos, notas, floats e campos e retorna páginas/display items. LayoutCache memoiza por revisão, largura e contexto.
5. render pinta o display list; pdf exporta a mesma composição.

## Modelo e edição existentes

[crates/doc/src/lib.rs](../../../crates/doc/src/lib.rs) define Document: corpo como blocos Arc, seções, partes secundárias, estilos, numbering, comentários, revisões, settings, mídia e passthrough por parte. [block.rs](../../../crates/doc/src/block.rs) define Para e Table. [para.rs](../../../crates/doc/src/para.rs) separa texto UTF-8, runs, props de parágrafo, props da marca, objetos e seção. Não há papel de roteiro tipado.

[crates/doc/src/edit.rs](../../../crates/doc/src/edit.rs) implementa inserção, divisão/junção e navegação por parágrafos. Paragraph::split_off preserva runs e transfere o metadado de seção ao segundo parágrafo. Caret e seleção usam StoryRef, Path e byte offset, com helpers para limites Unicode.

[crates/engine/src/cmd/text.rs](../../../crates/engine/src/cmd/text.rs) registra text.insert, text.newParagraph (Enter), text.lineBreak, text.tab, text.backspace e outros. [enter_tab.rs](../../../crates/engine/src/cmd/text/enter_tab.rs) implementa Enter, transições de lista e Tab contextual; autoformat.rs adiciona AutoCorrect/AutoFormat. Session::run mantém undo, proteção, redo e erro. Testes de teclado existem em crates/engine/src/tests/typing.rs.

## Layout e formatos

[crates/layout/src/lib.rs](../../../crates/layout/src/lib.rs) define LayoutOptions, DocLayout, páginas e layout. [pass.rs](../../../crates/layout/src/pass.rs) faz fluxo de seções/páginas, quebra e recomposição com limites; [cache.rs](../../../crates/layout/src/cache.rs) armazena layouts por parágrafo. [session/layout.rs](../../../crates/engine/src/session/layout.rs) conserva resultados para as opções atuais.

formats contém TXT, Markdown, HTML/MHT, RTF, ODT e LaTeX e depende de quick-xml. docx contém OOXML. engine/src/io_ext.rs roteia DOCX/DOC/XML e formatos acima. SAVE_EXTS e formats::EXTENSIONS não listam FDX/Fountain. A busca delimitada por screenplay, fountain, fdx e Final Draft em crates/docs não encontrou suporte nomeado. Isso não prova ausência de todo caminho indireto, mas as extensões não aparecem nos dispatches lidos. (Ver [io.rs](../../../crates/engine/src/io.rs), [io_ext.rs](../../../crates/engine/src/io_ext.rs), [formats/lib.rs](../../../crates/formats/src/lib.rs).)

## O que preservar e o que realmente precisa mudar

Preservar: modelo e primitivas de edição, seleção, commands, undo, layout cache, paginação, display list, render e PDF. Para roteiro de coluna única, mapear propriedades existentes antes de introduzir outro paginador. Usar command registry para CLI/MCP/atalhos compartilhados.

Adicionar: role tipada de screenplay e metadados opcionais, codec FDX próprio e comandos/queries para ações de roteiro. Paragraph não tem role de script, Block hoje tem apenas Para/Table e Document não carrega metadados FDX. Um grupo dual dialogue pode exigir representação composta e layout especializado, mas deve ser avaliado depois de fixar o comportamento. Document::passthrough é um exemplo geral de preservação de partes, mas não representa uma árvore FDX diretamente.

A busca encontrou 81 ocorrências de literais Paragraph na árvore, então qualquer campo adicionado exige atualização dos construtores e literais, serde(default), igualdade/revisão de cache e testes de compatibilidade. Esta análise é de código, não validação por execução.

## Implicações da observação real — 2026-11-07

O inventário acima continua sendo evidência de código de 2026-11-04, com suas contagens históricas. Executar o Final Draft não comprova o comportamento do WordCraft e não atualiza automaticamente esse inventário.

| Evidência no Final Draft | Implicação proposta para a infraestrutura existente | Classificação |
|---|---|---|
| ReturnKey configurável e SmartType ativo mudam a tecla | Handler deve considerar configuração/estado, além de role, posição e seleção | Observação COMPROVADA; encaixe no engine INFERIDO |
| OmittedScene conserva texto e repete ID do heading | Separar identidade interna de identificadores FDX com escopo; cena omitida não pode ser só heading removido | Observação COMPROVADA; representação Rust pendente |
| General/DualDialogue tem editor interno e AX externo dd | Seleção/path e walkers precisam de acesso ao conteúdo composto; não importar o placeholder AX como texto | Observação COMPROVADA; grupo vs variante de Block DESCONHECIDO |
| Locks guardam Position e rótulo; PDF tem páginas 1A/2A | Layout/cache precisam distinguir âncora/rótulo de produção e página física | Observação COMPROVADA; adaptação econômica INFERIDA |
| IDs auxiliares de CharacterArcBeat mudam na reabertura | Equivalência FDX deve ser definida por domínio e relatar normalização | Observação COMPROVADA; política de preservação de desconhecidos pendente |

Reutilizar Session, comandos, undo, layout e render continua uma **proposta**, não equivalência comprovada. A segurança de parsing bounded/Result, compatibilidade serde e wasm permanece requisito do projeto. Não foram adicionados campos, variantes, codecs, comandos nem testes funcionais nesta tarefa. Ver [evidências e limites](06-behavioral-investigation.md) e [plano condicionado](05-wordcraft-rust-implementation-plan.md).

## Histórico de revisão

| Data | Alteração |
|---|---|
| 2026-11-07 | Implicações dos casos reais, sem reclassificar leitura de código como teste de runtime do WordCraft |
| 2026-11-04 | Inventário de arquitetura e buscas delimitadas no workspace |
