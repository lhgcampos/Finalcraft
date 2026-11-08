# 02 — Editor, modelo e paginação do Final Draft

> Status: base estática de 2026-11-04 com validação comportamental parcial em 2026-11-07; teclado, cenas, paginação e FDX observados no aplicativo real.
>
> Revisão: 2026-11-07. Matriz completa e limites no [relatório 06](06-behavioral-investigation.md).

## Modelo de roteiro

**Observado no binário:** existem tipos FDElement separados para General, Scene Heading, Action, Character, Parenthetical, Dialogue, Transition, Shot e categorias de outline/produção. A rotina 0x100051d48 registra rótulos de diversos tipos; símbolos incluem construtores C++ separados para SceneHeading, Action, Character, Parenthetical, Dialogue, Transition e Shot. Isso sustenta semântica tipada no modelo interno, além de nomes de estilo. (Craft Evidence: ev_141903ef4a9ae536181c68e4226884125cfc938034c102b92c24841edbef2ee7; ev_896a5732e9e416276b60cc5c85871097912acd8b38e99b4bbb8eca24cd5b7f9b.)

O template Screenplay.fdxt confirma tipo explícito em Paragraph Type e formatação por parágrafo e trecho de texto. Também há configurações para elementos, cenas, números, SmartType, revisões, navegação, elenco, locações, transições, tags e quadros de produção. Os dados do template não foram copiados.

**Inferência:** a unidade editável visível é um parágrafo tipado e formatado; dados de produção se conectam a cenas e ao fluxo do script. **Desconhecido:** esquema completo de IDs, persistência de cena após renumeração, relações de metadados, tipos alternativos e compatibilidade de arquivos antigos.

## Edição e interação

Uma classe FDScriptPaneTabKeyHandler tem função HandleTabTransition e lambdas relacionadas; há comandos nomeados InsertDualDialogue, ExpandDualDialogue, BeginEditDualDialogue e EndEditDualDialogue. SmartType tem uma janela/tabela AppKit, categorias para Character, Scene Heading e Transition e marcadores SmartTypePreCharTyped e SmartTypePostCharTyped. Isso demonstra pontos de integração para Tab e sugestões, mas não a sequência completa de regras. (ev_c0ab06…; ev_4823de…; ev_583e18…)

O binário contém FDFinalDraftPaigeUndo com hooks WillUndo/DidUndo e chamadas StartUndoRecording/StopUndoRecording, além de controladores que expõem NSUndoManager. Isso sugere undo no editor integrado ao mecanismo do macOS; a granularidade dos grupos não foi observada. (ev_1c4f05…)

Strings e tipos apontam para FDEmbeddedDualDialogue, partições e rotinas de desenho/offset. Isso sugere uma representação composta; não é seguro modelar dual dialogue como dois parágrafos independentes sem casos de referência. (ev_a45a623…; ev_00b7e0…)

## Layout e paginação

Existem classes FDPageLayout, FDScriptPageLayoutCommand e nós XML para PageLayout, TextPageLayout e SPPageLayout. Há controladores para margens, opções e cores. O pacote contém opções de quebra de diálogo em início/fim de página e recursos “Mores and Continueds”. Esses nomes confirmam áreas funcionais, mas não o algoritmo de paginação. (ev_b2af88…; ev_8bf55a…; ev_00b7e0…)

Scene Heading, Scene Number, Edit Scene Number e notificações Scene Numbers Changed aparecem em strings e caminhos de UI. Revisões têm Revision Mode, Clear Revision e rótulos sobre revisão ativa e revisão mais alta exibida na página. Não foram inferidos algoritmos de numeração, quebra, órfãs, continuação, lock de página ou atualização incremental. (ev_d0f74e…; ev_f55793…; ev_e479f4…)

## Questões originais para observação (2026-11-04)

1. Com roteiro sintético, observar cada elemento e transições Enter, Tab, Backspace, seleção e undo/redo.
2. Inserir/remover cena antes de outra numerada e verificar IDs, números e atualização do navigator.
3. Testar diálogos longos, parentéticas, dual dialogue e quebras em páginas com/sem lock.
4. Salvar/reabrir FDX sintético com quebras manuais, revisões e variantes de template.
5. Consultar referência pública do formato para separar contrato FDX de detalhe privado do produto.

## Limitações

Na rodada de 2026-11-04, a GUI não foi aberta nem recebeu teclas. A função 0x10017eef3 tem pseudo-C inferido, tipos não recuperados e duas chamadas virtuais sem alvo; não é uma implementação fiel do parser. A cobertura está limitada a 50 mil funções e saltos indiretos não resolvidos. As conclusões daquela rodada são nomes, tipos, strings e referências estáticas. A validação de 2026-11-07 está separada abaixo.

## Validação comportamental — 2026-11-07

**COMPROVADO — FD-K:** 140 combinações reais de sete elementos × início/meio/fim/seleção/vazio × Enter/Tab/Shift+Tab/Backspace tiveram snapshots retornados. A [matriz CSV](behavior/keyboard-transitions.csv) registra estado inicial, expectativa desconhecida antes do ensaio, ação, texto/tipo observados e undo/redo quando executados. Não houve FDX por célula; ausência de mudança textual não valida um no-op de cursor/SmartType. Limites de Parenthetical precisam de confirmação de offset.

| Elemento preenchido | Enter no fim, preset inicial |
|---|---|
| Scene Heading | Novo Action |
| Action | Novo Action |
| Character | Novo Dialogue |
| Dialogue | Novo Action |
| Parenthetical | Novo Dialogue |
| Transition | Novo Scene Heading |
| Shot | Novo Action |

**COMPROVADO:** Action/Dialogue/Shot dividiram texto no meio; Enter com BETA selecionado preservou o texto e criou o próximo parágrafo nesses casos. Backspace juntou texto no início e apagou caractere/seleção nas outras posições. Mudança manual de tipo e os sete papéis foram conferidos no FDX gerado. Herança geral de tipos e IDs em split/join continua **DESCONHECIDA** sem XML por célula.

**COMPROVADO — FD-SMART/FD-CONFIG:** Character com prefixo A sugeriu ALFA; Enter aceitou e criou Dialogue, Tab aceitou e ficou em Character, Escape deixou A literal. Scene Heading I→INT. e Transition C→CUT TO: foram confirmados com Enter. Action/Dialogue/Parenthetical/Shot não expandiram o prefixo a testado. A UI permite escolher Next Element, e alterar Action para Character mudou Enter e o FDX. A regra depende de configuração e sugestão ativa.

**COMPROVADO — FD-SCENE/FD-OMIT:** números 1/2 passaram a 1/1A/2 e depois 1/2/3 por comandos; IDs ativos se mantiveram. Recortar/colar moveu uma cena com seu Action, preservando IDs e números. Navigator mostrou a nova ordem e a cena omitida. OmittedScene preservou conteúdo e repetiu o ID do heading interno/externo. Drag no Navigator não funcionou nas tentativas; permanece **DESCONHECIDO**.

**COMPROVADO — FD-PAGE/FD-LOCK:** caso fixo Courier Final Draft 12/Letter resultou em três páginas; Signal 010 e 046 iniciaram páginas 2 e 3, com MORE e CONT'D derivados. Lock gravou âncoras; inserção anterior gerou página física 1A e ajustou Positions. Margem e spacing alterados persistiram; a variante combinada teve cinco páginas físicas 1/1A/2/2A/3. Não inferir contagem física do status AX, que ainda mostrava “of 3”.

**COMPROVADO — FD-DUAL:** seleção de dois blocos criou duas colunas e wrapper XML General/DualDialogue; edição interna e save/reopen preservaram grupo e IDs. O caso longo transbordou visualmente e não está homologado como golden. **DESCONHECIDO:** algoritmo geral de quebra, órfãs, fontes alternativas, opções de continueds desligadas e relock. Os renders com preview divergente também não foram homologados cegamente.

**Undo/redo:** 87 células têm snapshots; 20 não satisfazem comparação ingênua de texto/tipo, incluindo cinco undos da preparação após nenhuma mudança observável. Não são 20 defeitos demonstrados. Divisões/junções, várias transições, conversão dual e omissão tiveram reversão observada; agrupamento geral e seleção restaurada permanecem pendentes.

O [relatório 06](06-behavioral-investigation.md) contém parâmetros, fontes, arquivos locais com hashes, comparação XML, limites de automação e procedimentos manuais. **INFERIDO:** o pipeline futuro precisa de edição composta, identidade com escopo e âncoras de produção. Nenhuma implementação foi iniciada.

## Histórico de revisão

| Data | Alteração |
|---|---|
| 2026-11-07 | Matriz e casos reais de editor/cenas/paginação/FDX; distinção explícita de snapshots, persistência e pendências |
| 2026-11-04 | Hipóteses estáticas e perguntas para runtime |
