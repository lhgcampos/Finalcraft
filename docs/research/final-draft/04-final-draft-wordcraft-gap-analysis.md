# 04 — Matriz técnica Final Draft × WordCraft

> Status: comparação estática de 2026-11-04 complementada por casos reais no Final Draft 13.5.0 build 123; WordCraft não implementado nesta tarefa.
>
> Revisão: 2026-11-07. Prioridades propostas; nenhum item é declarado equivalente.

## Matriz original de hipóteses estáticas (2026-11-04)

| Funcionalidade | Final Draft: evidência | WordCraft hoje | Diferença/impacto | Prioridade e estratégia Rust | Teste de equivalência futuro |
|---|---|---|---|---|---|
| Papéis de parágrafo | Tipos FDElement para Scene Heading, Action, Character, Parenthetical, Dialogue, Transition, Shot e outros; FDXT usa Paragraph Type. (ev_141903…, ev_896a…) | Paragraph genérico com texto/runs/ParaProps; sem papel tipado. [doc/para.rs](../../../crates/doc/src/para.rs) | Sem semântica para teclas, SmartType, FDX ou Scene Navigator. | P0: metadado opcional/enum de screenplay; não usar apenas nome de estilo como chave. | Parse, inserir, trocar tipo, split/join, save/reopen e unknown preservation. |
| FDX read/write | Tipos FDXMLTextParagraphNodeReader/Writer e nós PageLayout; template contém Content, Paragraph, Text, ElementSettings e metadados de cena/produção. (ev_9f741…, ev_ea820…, ev_b2af…) | formats não tem FDX; engine roteia XML para Flat OPC/Word 2003. [formats/lib.rs](../../../crates/formats/src/lib.rs), [io_ext.rs](../../../crates/engine/src/io_ext.rs) | Arquivo não abre/salva como roteiro. | P0: codec formats::fdx próprio e root detection distinto de XML Word; preservar extensões limitadamente. | Fixtures sintéticas: parse→model→write→parse semântica estável e extensões não suportadas preservadas. |
| Enter/Tab/Backspace | FDScriptPaneTabKeyHandler, InsertDualDialogue e SmartType pre/post-char aparecem no binário; regra exata não foi observada. (ev_c0ab06…, ev_4823de…, ev_583e18…) | Commands text.newParagraph, text.tab e text.backspace; lógica genérica em enter_tab.rs. | Infra existe; falta máquina de estados de screenplay. | P1: especializar handlers existentes por role após referência runtime; não duplicar dispatch. | Matriz de role/cursor/seleção; teclas, undo/redo e typing. |
| SmartType | Janela/tabela e categorias Character, Scene Heading, Transition. (ev_4823de…) | Nenhum autocomplete de screenplay encontrado. | Mais digitação e erros de personagem/elemento. | P1: provider de sugestão no engine e UI egui fina. | Sugestões por role, ordenação, confirmação, cancelamento e edição. |
| Identidade/navegação de cenas | SceneProperties, SceneNumberOptions, ScriptNavigatorPreferences, Scene Numbers Changed e Edit Scene Number. (ev_f55793…, ev_d0f74e…) | Ordem de parágrafos existe; sem entidade de cena/navigator própria. | Não modela IDs estáveis, reordenação ou cenas omitidas. | P1: anchor de scene heading, índice derivado e invalidado estruturalmente; separar ID de número mostrado. | Inserção/remoção/reorder, renumeração, edição e undo preservam IDs/ordem. |
| Dual dialogue | FDEmbeddedDualDialogue, partições, Insert/Expand/Begin/EndEdit e rotinas de desenho/offset. (ev_00b7e0…, ev_a45a623…, ev_583e18…) | Block é apenas Para/Table. [doc/block.rs](../../../crates/doc/src/block.rs) | Parágrafos em sequência não representam duas lanes acopladas. | P1/P2: avaliar grupo pareado por metadados vs bloco composto antes de espalhar variante em todo o modelo. | Inserir, editar lanes, selecionar, undo, save/reopen e alturas diferentes. |
| Paginação/continueds | Nós PageLayout, options de quebra de diálogo e recursos Mores and Continueds; algoritmo não provado. (ev_b2af88…, ev_8bf55a…) | Paginação e cache genéricos já existem. [layout/lib.rs](../../../crates/layout/src/lib.rs), [pass.rs](../../../crates/layout/src/pass.rs) | Base reutilizável; constraints de roteiro precisam ser medidas. | P1: mapear medidas para props existentes; especializar apenas agrupamento/continued confirmado. | Golden page cases por versão/fontes; edição antes da quebra e diálogos longos. |
| Revisões e página travada | Revision Mode, Clear Revision, revisão ativa e mais alta por página em strings; sem algoritmo confirmado. (ev_e479f4…) | Revisões de Word existem; sem revisão/color set e locked pages FDX dedicados. | Dados de produção podem se perder. | P2: preservar metadados primeiro; escolher escopo após observar fluxo de revisão. | Round-trip, cores/conjuntos e páginas locked/unlocked. |
| Elenco, locações, relatórios e tags | Template/controladores mencionam Actors, Locations, TagCategories, CharacterTraitData e relatórios. | Sem modelo de production breakdown FDX. | Sem relatórios ligados às cenas. | P2: modelos separados do texto; começar por consultas read-only determinísticas. | Personagens/locações repetidos, reorder, renumber e undo. |

## Fatos, inferências e desconhecidos

**Fatos observados:** binário contém tipos, strings, classes e referências para esses subsistemas; template FDXT embutido contém XML com tipos e metadados; os módulos WordCraft lidos não implementam suporte FDX/screenplay nomeado.

**Inferências:** FDElement codifica semântica; os nós FDXML dão suporte de leitura/escrita por elemento; dual dialogue é composto; o encaixe econômico é estender o pipeline atual.

**Desconhecidos na rodada estática:** detalhes das teclas, IDs, regras de numeração, algoritmo de paginação, comportamento de versões FDX, preservação de campos não suportados. A busca estática não é runtime nem cobre completamente 50 mil funções. A rodada abaixo resolve apenas partes delimitadas dessas perguntas.

## Declaração de paridade

Nenhum item é equivalente por semelhança de nomes ou XML. Cada linha exige cenário reproduzível em versão identificada do Final Draft, fixture própria, round-trip e regressão automática. Nesta fase, os itens estão ausentes, parcialmente cobertos por infraestrutura genérica ou aguardando prova.

## Reavaliação comportamental — 2026-11-07

| Área | COMPROVADO no Final Draft real | Diferença em relação à hipótese inicial / ação no plano | DESCONHECIDO |
|---|---|---|---|
| Papéis e teclas | 140 combinações retornadas; sete tipos no FDX; diferenças por posição, seleção e vazio | Não usar transição fixa só por enum; considerar configuração e sugestão | XML por célula, limites efetivos de parentética e seleção complexa |
| Configuração | Next Element de Action alterado para Character; tecla e ReturnKey persistido mudaram | Preservar Behavior, não apenas fonte/indentação | TabKey personalizado e todos os presets |
| SmartType | A→ALFA; Enter aceita e avança, Tab aceita sem avançar; Escape cancela complemento; I→INT., C→CUT TO: | Sugestão ativa é estado do handler; provider read-only ainda é escolha de API | Aprendizado, ordenação, conflitos e listas completas |
| Cenas/IDs | Número e ordem mudam sem trocar IDs ativos nos casos; cut/paste preservou heading e Action | Ordem documental separada de chave; não ordenar por UUID | Cópia entre documentos, duplicação, todos os split/join |
| Omissão | Conteúdo em OmittedScene; ID repetido entre heading ativo e original guardado | Invalidada unicidade global ingênua e modelagem de omissão como remoção | Restaurar por menu e múltiplas cenas |
| Dual | Wrapper General/DualDialogue; editor interno; IDs internos mantidos; duas colunas no caso curto | Preservar hierarquia; metadado em dois parágrafos independentes é insuficiente | Arquitetura Rust grupo vs Block e layout longo qualificado |
| Paginação | Referência de 3 páginas; continueds derivados; margem/spacing persistidos | Manter props comuns e acrescentar constraints demonstradas | Órfãs, parentéticas de fronteira, fontes e outras opções |
| Lock | Position/rótulos preservados e ajustados; exportações de 4 e 5 páginas físicas com A pages | Separar âncora/rótulo e página física | Position Unicode, exclusão, relock e revisão |
| Reabertura | Quatro documentos reabertos; texto ativo preservado; alguns IDs auxiliares regenerados | Comparar semântica por domínio e expor os diffs | Preservação arbitrária de extensões e outras versões FDX |

As decisões Rust continuam **INFERIDAS**, mesmo nas linhas que têm comportamento COMPROVADO. A implementação do WordCraft não foi executada nem comparada; não há redução de gap ou aumento de paridade declarado. Drag no Navigator e certas tentativas de controle foram inconclusivos, com procedimentos manuais no [relatório 06](06-behavioral-investigation.md). A matriz de teclado é um registro de snapshots, não uma suíte de equivalência homologada.

## Histórico de revisão

| Data | Alteração |
|---|---|
| 2026-11-07 | Reavaliação por área com evidência real, hipóteses invalidadas e escopo ainda desconhecido |
| 2026-11-04 | Matriz estática original, sem declaração de equivalência |
