# 05 — Plano de implementação Rust: screenplay e FDX no WordCraft

> Status: proposta revisada por evidência comportamental parcial; implementação não autorizada nem iniciada.
>
> Revisão: 2026-11-07. **Etapas 1–5 exigem nova autorização do proprietário.** Ver [validação e pendências](06-behavioral-investigation.md).

## Arquitetura-alvo

Preservar o pipeline existente e adicionar semântica de roteiro ao mesmo documento.

- doc: role opcional e metadados de screenplay. Separar identidade interna do WordCraft, identificadores FDX do fluxo ativo, conteúdo omitido e IDs auxiliares; não presumir unicidade global ou estabilidade de todo Paragraph/@id. Número mostrado e ordem documental são conceitos separados. Campos serde(default) mantêm compatibilidade com JSON existente; geração/clone de identidade exige contrato próprio.
- formats::fdx: codec XML Rust usando quick-xml, dispatch de .fdx/.fdxt e armazenamento bounded de elementos desconhecidos. Preservar hierarquia de OmittedScene e General/DualDialogue; root Version=6 foi observado em arquivos deste build, não deve ser confundido com versão 13.5.0 do produto.
- engine::cmd: comandos de trocar elemento, inserir/renumerar/omitir cena, navegar e aceitar sugestões, usando CommandSpec, Session::run, proteção, seleção e undo. Handler de teclado recebe configuração por elemento (ReturnKey/TabKey), posição, seleção, estado vazio e sugestão ativa; não hardcodear uma tabela universal por enum.
- layout: reutilizar SectionProps, ParaProps, estilos e métricas para coluna única. Distinguir texto do documento, repetição/decoração de continued, página física, rótulo de produção e âncora de página travada. Unidade de Position com Unicode permanece desconhecida.
- ui-egui: seletor de elemento, SmartType e Scene Navigator como consumidores dos comandos/queries do engine; seleção/edição interna de grupos precisa de contrato que não dependa do placeholder AX dd.
- render/pdf: manter display list e render atuais; estender somente se novo agrupamento exigir.

### Contratos

1. fdx::read(bytes, limits) -> Result<Document, FdxError>; XML inválido, profundo ou grande retorna erro bounded, nunca panic.
2. fdx::write(&Document, options) -> Result<Vec<u8>, FdxError>; saída determinística e unknown extensions preservadas quando possível.
3. ScriptParagraphKind representa papéis conhecidos e guarda o identificador original de tipos desconhecidos. Role não substitui configuração de Behavior.
4. ScreenplayMetadata guarda versão de formato observada, preset/configurações por elemento, continueds, locks e metadados globais, com preservação limitada e explícita.
5. Índice de cenas é derivado em **ordem documental**, com chave/anchor independente; invalidado em mudança de role, split/join, inserção, remoção, movimento e omissão/restauração. Conteúdo omitido não entra inadvertidamente no fluxo ativo, mas deve continuar consultável/preservado conforme contrato futuro.
6. Consultar sugestões como query read-only e aceitar como comando mutável/undoável continua escolha de API, não fato provado sobre o produto. Aceitação por Enter pode avançar o elemento, enquanto Tab apenas aceita; cancelar mantém o prefixo digitado no caso observado. Aprendizado persistente ainda precisa de referência.
7. Cache de layout considera as duas falas, configuração de métricas/quebras/continueds, estado de lock e revisão do grupo composto. Uma revisão isolada de um parágrafo pode não bastar.
8. Identificadores FDX têm escopo: o mesmo id apareceu no marcador ativo e no heading dentro de OmittedScene; IDs em parágrafos auxiliares de CharacterArcBeat mudaram ao reabrir. Política de importação não deve rejeitar esse corpus por uma restrição global inventada.
9. A equivalência de round-trip é definida por domínio: texto/runs/tipos, configuração, números/IDs ativos, conteúdo omitido, grupos e âncoras. Diffs de metadados auxiliares são relatados, sem afirmar byte identity.

## Etapas dependentes

### 0 — Referência comportamental antes do código

Criar roteiros sintéticos no Final Draft, observando apenas a janela e registrando versão/build, configuração, fonte, elemento, cursor, tecla, resultado, undo e FDX salvo. Cobrir transições de parágrafo, cena/numeração/omissão, SmartType, dual dialogue, revisões e quebra de página. Consultar documentação pública de FDX antes de transformar o corpus em contrato de formato. Não incorporar assets, templates, texto de roteiro de terceiros ou código do produto.

**Estado em 2026-11-07:** quatro documentos sintéticos criados/salvos/reabertos; 140 combinações de teclado registradas; comportamento de sugestões, configuração de Enter, numeração, omissão, dual e locks parcialmente comprovado. Ver [relatório 06](06-behavioral-investigation.md), [matriz](behavior/keyboard-transitions.csv) e [manifesto local](behavior/local-evidence-manifest.json). Snapshots de teclado não têm FDX individual por célula; 20 sequências de undo/redo precisam de interpretação/repetição; drag no Navigator e layout dual longo não estão qualificados. Revisões, IME, Unicode em locks e contrato público completo de FDX continuam pendentes.

**Aceitação por subsistema:** entrada própria, versão/configuração/fonte, ação real, estado após ação, seleção/offset quando relevante, FDX antes/depois, reopen e undo/redo. Usar COMPROVADO / INFERIDO / DESCONHECIDO. Não liberar um subsistema dependente de teste inconclusivo; não promover strings/símbolos estáticos a validação. A observação desta rodada não elimina as pendências nem autoriza começar código.

### 1 — Modelo com compatibilidade

Após autorização: avaliar papel e metadados opcionais em crates/doc/src/para.rs e document.rs/lib.rs; atualizar construtores/literais (81 ocorrências na busca histórica). Preservar serde antigo, igualdade, Arc sharing, Paragraph::split_off e revisão de cache. Modelar conteúdo omitido e fronteira de grupos antes de escolher a representação de identidade; não reaproveitar automaticamente um id FDX como chave interna.

**Testes futuros:** JSON antigo; split/join/clone com política explícita de identidade; repetição legítima em OmittedScene; IDs auxiliares sem estabilidade assumida; undo; igualdade/cache; limites de entrada. Esses testes não foram implementados nesta tarefa.

### 2 — Codec FDX

Após autorização: adicionar crates/formats/src/fdx/{mod,read,write,limits,tests}.rs, expor em formats/lib.rs e atualizar engine/src/io.rs, io_ext.rs, SAVE_EXTS e abertura/salvamento no CLI/UI. Identificar pelo root/extension sem confundir Flat OPC ou Word 2003 XML. Mapear texto/runs/tipos, Behavior, page setup, cenas/números, OmittedScene e grupos; guardar desconhecidos com orçamento de bytes/profundidade. Enquanto a hierarquia não puder ser representada, preservar com aviso ou retornar erro explícito; nunca achatar dual/omissão silenciosamente.

**Testes futuros:** FDX próprio mínimo por tipo, Unicode, formatação direta/por elemento, Behavior, page setup, cenas/omissões, wrapper dual e fluxo interno; XML hostil; round-trip por domínio e extensão desconhecida. Fixtures redistribuíveis devem ser autoradas com texto próprio e fonte aberta, sem copiar presets/recursos proprietários do produto.

### 3 — Comandos e digitação

Após autorização: criar engine/src/cmd/screenplay.rs; estender comandos atuais em engine/src/cmd/text/enter_tab.rs, delete.rs e autoformat.rs somente para regras qualificadas na Etapa 0. A matriz atual é evidência exploratória: repetir offsets/seleções e casos de histórico antes de usá-la como golden. Documentar mudanças em typing-parity e testar keys pelo mesmo command path. Configuração customizada e sugestão ativa precisam de casos próprios.

**Testes:** matriz role×caret×selection para Enter/Tab/Shift+Tab/Backspace; AutoCap/SmartType; undo; CLI/MCP chamam mesmo command ID.

### 4 — Cenas e produção

Após autorização: criar índice derivado em ordem documental e comandos para inserir, omitir/restaurar, renumerar e navegar. Distinguir número persistido, ordem exibida e identificador; a omissão conserva conteúdo e número em vez de excluir o heading. Depois, elenco, locações, tags e relatórios read-only. Integrar Scene Navigator UI após API de query estabilizar; drag continua sem referência validada.

**Testes futuros:** inserir antes, esquema 1A com/sem preservar números, cut/paste e cópia, omitir/restaurar conteúdo, hierarquia com IDs repetidos legítimos, pesquisa, references, undo/redo e relatórios determinísticos. Não generalizar estabilidade do fluxo ativo para todos os metadados auxiliares.

### 5 — Paginação, dual dialogue e revisões

Após autorização e qualificação visual: mapear margens/fontes/spacing para propriedades atuais. Escolher grupo com hierarquia vs Block composto a partir dos casos de edição interna e layout; os arquivos observados já exigem preservar o wrapper e seu conteúdo. Não deduzir regra geral de lanes a partir de apenas dois Character. Continued é decoração derivada nos casos, não texto a duplicar. Locks precisam de âncoras ajustáveis, rótulos e páginas A, separados da contagem física. Revisões/cores, Unicode em Position, relock e dual longo ainda exigem referência; evitar nova variante de Block sem inventário dos walkers/matches.

**Testes:** snapshots página a página; conteúdo assimétrico em dual dialogue; diálogo longo; quebra manual; continued; lock; edição antes da quebra e cache após undo.

## Arquivos previstos

**Primeiro corte:** crates/doc/src/para.rs, crates/doc/src/document.rs ou lib.rs, novos crates/formats/src/fdx/*, crates/formats/src/lib.rs, crates/engine/src/io.rs, io_ext.rs, cmd/screenplay.rs, cmd/text/* e testes nos crates formats/engine. Documentar FDX e typing parity.

**Fases posteriores, se necessidade for demonstrada:** crates/layout/src/para/*, flow.rs, pass.rs, cache.rs, placed.rs, Scene Navigator e seletor em ui-egui. Não abrir crate novo enquanto doc/formats/engine fornecem fronteira suficiente.

## Riscos, migração e critérios de aceite

- FDX pode conter dados não mapeados: preservar extensões desconhecidas e comunicar importação parcial; não converter silenciosamente para texto comum.
- serde: campos default e fixtures antigas; sem migração massiva.
- Split/paste exige política de geração/clonagem; distinguir duplicação ilegítima de repetição legítima em domínio aninhado. Cut/paste no caso de cena preservou IDs; isso não prova todos os fluxos de clipboard.
- Paginação divergente: declarar equivalência somente com golden pages por build de referência e fonte instalada fixa.
- Bloco composto de dual dialogue amplia mudanças por todos os consumers de Block; adiar até validar comportamento.
- Inputs XML são hostis: tamanho, profundidade e contagens limitados; Result, sem unwrap/expect/panic e sem unsafe.
- Manter Rust/wasm; revisar compatibilidade de quick-xml e parsing no alvo web.
- Snapshots/preview podem divergir do estado estabilizado: verificar controle após mudar valor, esperar conclusão pela observação real, comparar FDX e homologar o render antes de usá-lo como golden. Não reproduzir artefatos de automação como comportamento do produto.

Critério de aceite do primeiro release: FDX sintético abre e round-trippa papéis conhecidos; texto, formatação e page setup persistem; dados desconhecidos são preservados ou reportados; comandos têm undo; typing confirmado tem testes; layout tem golden cases; JSON/DOCX existentes seguem cobertos. Isso é uma meta incremental, não declaração de paridade com Final Draft.

## Histórico de revisão

| Data | Alteração |
|---|---|
| 2026-11-07 | Barreira explícita de nova autorização; Behavior e sugestão no handler; identidade com escopo; conteúdo omitido e dual hierárquicos; locks/rótulos; equivalência semântica e pendências reais |
| 2026-11-04 | Proposta inicial derivada de inspeção estática e leitura do workspace |
