# 06 — Investigação comportamental do Final Draft

> Status: validação comportamental parcial na interface real; quatro documentos sintéticos salvos e reabertos; implementação não autorizada nem iniciada.
>
> Revisão: 2026-11-07. Alvo observado: Final Draft 13.5.0, build 123.

## Alcance e classificação

Foram lidos integralmente os cinco relatórios de 2026-11-04 antes dos testes. A análise anterior é histórica e estática. Nesta rodada, ações reais foram executadas pelo `cua_repl` contra o aplicativo instalado. Nenhum binário, componente, fonte ou recurso do bundle foi alterado ou extraído. Nenhum código funcional do WordCraft foi modificado.

- **COMPROVADO**: resultado efetivamente retornado pela interface, XML salvo pelo aplicativo ou PDF exportado por ele, dentro do caso indicado. Um snapshot não estabelece uma regra universal.
- **INFERIDO**: interpretação arquitetural sustentada pelos resultados, ainda dependente de outros casos.
- **DESCONHECIDO**: não executado, automação inconclusiva ou evidência insuficiente. Ausência de mudança de texto não prova ausência de efeito no caret, seleção ou sugestão.

Os identificadores `FD-*` abaixo são referências locais desta investigação; não são Evidence IDs do Craft Anything. Os `ev_*` dos relatórios anteriores continuam sendo evidência estática, não evidência desta execução.

## Ambiente e preservação

| Item | Registro |
|---|---|
| Versão na interface About | **COMPROVADO**: 13.5.0, Build 123, 2026 |
| Aplicativo | `/Applications/Final Draft 13.app`; `mdls` também informa 13.5.0 |
| Host | macOS 26.6.2, build 25G83; `uname -m`: arm64 |
| Data/fuso | 2026-11-07, America/Sao_Paulo; início do registro de ambiente 19:09:33 UTC |
| Arquitetura do processo | **DESCONHECIDO** nesta rodada. x86_64 do artefato é registro estático anterior; Rosetta não foi observada diretamente |
| Interface | Inglês; diálogos do sistema usam locale português; zoom de documento 100% |
| Documento inicial de teste | File → New, `Untitled Screenplay`; todos os textos inseridos são próprios e sintéticos |
| Papel/fonte | US Letter, 8,5 × 11 polegadas; Courier Final Draft, 12 pt, declarado em `FontSpec`; PDF confirma subset embutido `CourierFinalDraft` |
| Margens iniciais | Superior/inferior 1"; cabeçalho/rodapé 0,5"; FDX: 72/72/36/36 |
| SmartType inicial inspecionado | Character matching, Auto-guess next character name, Scene Heading matching, Transition matching e Show SmartType window ligados; transição Tab para extensão de personagem desligada; ordenação por número de falas ligada |
| Proofing | Sublinhados visíveis e `AdornmentStyle="-1"` em trechos; não foi desligado para fabricar XML mais simples |
| Fontes redistribuídas | Nenhuma. Não se leu nem copiou arquivo de fonte. A fonte embutida na exportação permanece somente na evidência local |
| Versão/hash do arquivo de fonte | DESCONHECIDO; foram registrados o nome no FDX/PDF e o tamanho, sem ler o arquivo da fonte |

O aplicativo já tinha outro documento aberto. Ele não foi usado como caso, editado, salvo, exportado ou incorporado à documentação. Os quatro documentos novos foram criados isoladamente. Metadados e conteúdos incidentais de janelas anteriores não integram os registros abaixo.

Os arquivos produzidos pelo produto ficam em `plan/final-draft/runtime-2026-11-07/`, confirmado como gitignored. Os PNGs locais são **renders de PDFs exportados**, não screenshots da GUI. Screenshots da janela sintética foram observados no histórico das ferramentas. Não há captura do desktop nos entregáveis. O [manifesto](behavior/local-evidence-manifest.json) identifica caminhos locais, tamanhos, SHA-256 e comparações de reabertura; não embute os arquivos do produto no repositório.

As [comparações FDX extraídas](behavior/fdx-comparisons.json) registram os valores e identidades efetivamente comparados nas etapas de cenas, omissão, dual, lock, formatação e Behavior. São dados de observação, não implementação de um codec WordCraft.

## Evidências e cobertura

| ID | Ações reais / arquivos | O que comprova | Limite |
|---|---|---|---|
| FD-ENV | About; `sw_vers`; `uname`; `mdls` | Identidade e ambiente acima | Sem identificação da arquitetura do processo |
| FD-K | [140 linhas CSV](behavior/keyboard-transitions.csv) e [observações estruturadas](behavior/keyboard-observations.json) | Sete tipos × cinco estados × quatro teclas; texto AX e seletor de elemento antes/depois | Uma passagem; sem FDX e reopen por célula; caret não medido por offset em cada linha |
| FD-TYPE | `01-editor-baseline.fdx`, `01-editor-pre.fdx` | Sete tipos criados por mudanças manuais e transições; persistência de `Paragraph/@Type` | Não é especificação completa de digitação |
| FD-SMART | Lista/Options do SmartType; prefixos por teclas reais; `01-editor-smarttype-enter.fdx`; estados finais | Confirmação por Enter/Tab, Escape e algumas expansões | Ordenação, aprendizado e todas as listas não qualificados |
| FD-CONFIG | Elements → Action → Next Element; `01-editor-custom-return-confirmed.fdx` e final reaberto | Enter configurável e serializado em `Behavior` | Variação de `TabKey` não alterada nesta rodada |
| FD-SCENE | `02-scenes-pre`, `-numbered`, `-inserted`, `-1A`, `-renumbered`, `-moved` | Criação, inserção, numeração, renumeração e movimento por recortar/colar | Arrastar no Navigator não foi bem-sucedido |
| FD-OMIT | `02-scenes-omitted`, `-omitted-body`, final reaberto; undo/redo | Conteúdo omitido preservado em subárvore, com número e identidade do marcador ativo | Restaurar pelo comando de produção e casos de múltiplas cenas não executados |
| FD-PAGE | `03-pagination-baseline.fdx`, `03-pagination.pdf` | Três páginas, diálogo longo, parentética e continueds | Apenas uma fonte e um preset; algoritmo geral não descoberto |
| FD-LOCK | `03-pagination-locked.fdx`, `-locked-edited.fdx/.pdf` | Âncoras de páginas e página física 1A após inserção | Unidade de Position com Unicode e exclusão/relock não testadas |
| FD-FORMAT | `03-pagination-margin.fdx`, `-spacing.fdx`, `-margin-spacing.pdf` | Margem de 1,5", espaçamento 1,5 e Bold por trecho | PDF combinado não isola o efeito visual de cada mudança |
| FD-DUAL | `04-dual-before`, `-after`, `-edited`, final reaberto; `04-dual.pdf`; `03-pagination-dual.fdx` | Grupo composto, duas colunas, editor interno, IDs preservados, caso longo problemático | Limites de quebra do grupo longo ainda não qualificados |
| FD-REOPEN | Quatro `*-final-reopened.fdx`; comparações no manifesto | Preservação do fluxo ativo e alterações auxiliares de IDs | Não prova compatibilidade com outras versões nem preservação arbitrária de extensões |

## Editor: matriz e significado dos resultados

Antes dos ensaios, os relatórios não definiam resultados por posição/seleção. O campo **esperado** do CSV registra esse desconhecimento, em vez de inventar uma expectativa retrospectiva. O campo **observado** contém o estado efetivamente retornado. Cada estado conserva texto e tipo; `\r` representa separação retornada pela área de edição.

Preparação de cada combinação: substituir o documento de teste por `PREFIX`, aplicar Action, Enter, aplicar o tipo sob teste e inserir `ALFA BETA`. Testar início, meio antes de `BETA`, fim, seleção de `BETA` e vazio. Parenthetical insere `(...)`; os limites foram repetidos pedindo a posição em relação ao texto completo, mas o aplicativo pode ajustar o caret junto aos delimitadores. Esse limite não foi escondido.

As 140 combinações tiveram retorno da ferramenta. Quatro repetições de início de Parenthetical foram substituídas pela última observação no conjunto canônico. Um lote anterior excedeu 30 segundos e perdeu o registro; suas ações **não** contam como evidência. Os lotes seguintes foram menores e retornaram resultados. Não há simulação de teclado nem resultados de um futuro editor Rust nesta matriz.

### Transições mais claras no preset inicial

**COMPROVADO — estado após a tecla no caso indicado; SmartType pode mudar a interpretação.**

| Elemento | Enter no fim preenchido | Tab no fim preenchido | Shift+Tab no fim preenchido | Tab vazio | Shift+Tab vazio |
|---|---|---|---|---|---|
| Scene Heading | Novo Action | Acrescentou ` - `; sugestão de horário interferiu no histórico subsequente | Mesmo acréscimo observado | Action no mesmo parágrafo | Transition no mesmo parágrafo |
| Action | Novo Action | Criou novo parágrafo; seletor permaneceu Action | Mesmo resultado observado | Character | Scene Heading |
| Character | Novo Dialogue | Novo Parenthetical com `()` | Mesmo resultado observado | Transition | Action |
| Dialogue | Novo Action | Novo Parenthetical com `()` | Mesmo resultado observado | Parenthetical com `()` | Parenthetical com `()` |
| Parenthetical | Novo Dialogue | Novo Dialogue | Novo Dialogue | Removeu `()` e mudou para Dialogue | Mesmo resultado observado |
| Transition | Novo Scene Heading | Novo Scene Heading | Novo Character | Scene Heading | Character |
| Shot | Novo Action | Novo Action | Novo Action | Action | Action |

Essas transições não devem virar uma tabela universal por enum: o ensaio FD-CONFIG mudou o destino de Enter de Action para Character. Após escolher o valor, **conferir o popup**, aplicar, digitar outro Action e pressionar Enter, o seletor passou a Character e o arquivo gravou `ReturnKey="Character"`. A primeira tentativa encadeada não aplicou a configuração e o XML continuou Action; somente a repetição verificada sustenta a conclusão.

### Posição, seleção, divisão e junção

- **COMPROVADO:** Action, Dialogue e Shot dividiram `ALFA BETA` em `ALFA ` / `BETA` com Enter no meio. Com `BETA` selecionado, Enter preservou o texto e acrescentou o próximo parágrafo, com o tipo da tabela. Não se observou substituição da seleção por uma quebra nesses casos.
- **COMPROVADO:** Enter no início acrescentou separador antes do texto em Scene Heading, Action, Character, Dialogue, Transition e Shot. Backspace no início juntou o texto com `PREFIX`; no meio removeu o espaço; no fim removeu a última letra; sobre a seleção removeu `BETA`. O seletor após a junção está no CSV, mas a herança do tipo de **todos** os parágrafos envolvidos precisa de XML por caso.
- **COMPROVADO somente como snapshot:** Enter no meio de Scene Heading, Character, Transition e Parenthetical não mudou texto/tipo no retorno. Tab/Shift+Tab no início/meio/seleção também não mudaram esses dois campos nos casos registrados. Isso não valida ausência de efeito de caret ou SmartType.
- **COMPROVADO:** Parenthetical criou delimitadores automaticamente. Backspace perto deles teve resultados distintos: no início solicitado removeu `(`; no fim solicitado removeu a letra antes de `)`. A seleção de `BETA` deixou `(ALFA)`. **DESCONHECIDO:** offsets efetivos nos limites e política geral de limpeza de espaços/delimitadores.
- **COMPROVADO somente como snapshot:** Enter em vazio não alterou texto/tipo nos sete casos. Não se deduziu uma regra de menu de elementos a partir apenas dessa ausência de mudança.
- **COMPROVADO:** atalhos Cmd+1…Cmd+7 mudaram manualmente o tipo durante a preparação. O FDX de `01-editor-pre` contém os sete papéis, além do Action final vazio. Conversão de seleções com vários tipos ainda é **DESCONHECIDA**.

### Undo/redo

Há snapshots de undo/redo em 87 células. Em 20 sequências, os snapshots não satisfazem a igualdade ingênua “undo = antes; redo = depois”. Cinco dessas sequências tentaram undo após nenhuma mudança observável, desfazendo a preparação anterior. Outras envolvem sugestões de Scene Heading/Character/Transition ou seletor/caret após junção. Isso **não** foi classificado como 20 bugs do Final Draft.

**COMPROVADO:** várias divisões/junções em Action, Dialogue e Shot, transições para Parenthetical/Dialogue, conversão dual e omissões tiveram restauração observada por undo/redo. **DESCONHECIDO:** agrupamento universal do histórico, ponto exato da seleção restaurada e transações de sugestão. As exceções permanecem visíveis no CSV; não foram convertidas em testes esperados de WordCraft.

### SmartType e autocomplete

| Caso real | Resultado | Classificação |
|---|---|---|
| Lista Characters no documento sintético | `ALFA` apareceu na lista | COMPROVADO |
| Character vazio, tecla `a` | Texto AX `A`; complemento cinza `ALFA` visível | COMPROVADO |
| Confirmar essa sugestão com Enter | `ALFA` persistido e novo Dialogue | COMPROVADO |
| Confirmar com Tab | `ALFA`, permanecendo em Character | COMPROVADO |
| Nova sugestão de `ALFA`, Escape | Ficou apenas `A`, sem complemento cinza | COMPROVADO |
| Scene Heading, tecla `i`, Enter | `INT. ` e novo Action | COMPROVADO |
| Transition, tecla `c`, Enter | `CUT TO:` e novo Scene Heading | COMPROVADO |
| Action / Dialogue / Parenthetical / Shot, tecla `a`, Enter | Prefixo literal, sem expansão equivalente; parentética ficou `(a)` | COMPROVADO somente para esses prefixos |
| Ordenação, conflitos, aprendizado, listas de locações/horários, IME e seleção cruzando parágrafos | Não qualificados | DESCONHECIDO |

Repetições de ALFA também exibiram `(CONT’D)` sem esse sufixo no texto do personagem. Não se deduziu sua política completa de ativação. Abrir uma lista sem mutação, aprendizado persistente e aceitação como uma transação única ainda precisam de casos específicos.

## Cenas, números e Navigator

O caso começou com BETA e GAMA, dois Scene Headings, cada um seguido por Action. Os UUIDs abaixo são abreviados somente para leitura; valores completos estão nos arquivos locais.

| Estado/ação | Resultado real | Identidade |
|---|---|---|
| Criar e salvar duas cenas | Sem `Number` inicialmente | BETA `f1d8f77c…`; GAMA `387e0057…` |
| Scene Numbers, padrão | `Number=1,2` | Mesmos IDs |
| Inserir DELTA entre elas | Nova cena sem número; GAMA continuou 2 | DELTA `05272ed8…`; IDs anteriores mantidos |
| Numerar com Keep existing numbers e esquema 1A | `1,1A,2` | Mesmos IDs |
| Renumerar sem Keep existing numbers | `1,2,3` | Mesmos IDs |
| Recortar GAMA + Action + terminador e colar antes de BETA | Ordem GAMA, BETA, DELTA; números `3,1,2` | IDs de GAMA e de seu Action mantidos |
| Navigator após movimento | Coluna Order `1,2,3`, títulos GAMA, BETA, omitida | Order é ordem, não número persistido da cena |
| Omitir DELTA, depois GAMA com conteúdo; undo/redo | Marcadores omitidos; original volta no undo e some no redo | Número/ID do marcador preservados; conteúdo guardado em subárvore |
| Salvar, fechar, reabrir e salvar | Omissões e conteúdo interno mantidos | Comparação final do Content de 02 idêntica |

**COMPROVADO:** número exibido, ordem documental e identidade do parágrafo não são o mesmo conceito nesses casos. A numeração não se recompôs automaticamente ao inserir/mover; foi aplicada pelo comando.

**COMPROVADO:** a omissão não é uma exclusão. O parágrafo ativo Scene Heading conserva `Number` e `id`, escreve texto `Omitted` e contém `OmittedScene`. Dentro dele estão o Scene Heading original e, no caso com conteúdo, o Action original. O heading interno reutiliza o ID do marcador externo. A interface retornou `OMITTED++`; o Navigator retornou `OMITTED[]`. Não copiar esses marcadores de apresentação como modelo de dados.

**Limitações de automação:** duas tentativas de arrastar a terceira linha do Navigator para o início devolveram o foco ao roteiro sem mover texto. O movimento por drag permanece **DESCONHECIDO**. O campo Starting Number recebeu uma tentativa de `10` por `setValue`, mas a numeração salva foi `1,2,3`; o valor 10 não foi confirmado antes de aplicar. Não se declarou validado começar em 10 nem se atribuiu isso a defeito do produto.

## Paginação reproduzível

### Entrada e parâmetros fixos

O `03-pagination-baseline.fdx` tem oito parágrafos:

1. Scene Heading: `INT. TEST CHAMBER - DAY`.
2. Um único Action: concatenar com espaço, para n de 001 a 040, `Action n: the blue box remains on the wooden table.` (2.159 caracteres).
3. Character: `ALFA`.
4. Um único Dialogue: concatenar com espaço, para n de 001 a 070, `Signal n remains stable while the blue lamp glows.` (3.709 caracteres).
5. Parenthetical: `(checking the panel)`.
6. Dialogue: `The final synthetic signal is complete.`
7. Character: `BETA`.
8. Dialogue: `The second synthetic voice answers briefly.`

Usar Courier Final Draft 12, US Letter e margens do ambiente. `DocumentLeading=Normal`, `Leading=Regular`, `Spacing=1`, `BreakDialogueAndActionAtSentences=Yes`. Posições horizontais de `ParagraphSpec` em polegadas, conforme UI/régua e arquivo gerado: Action/Scene Heading/Shot 1,50–7,50; Character 3,50–7,25; Dialogue 2,50–6,00; Parenthetical 3,00–5,50 com FirstIndent −0,10; Transition 5,50–7,10, alinhado à direita. SpaceBefore: Scene Heading 24; Action/Character/Transition/Shot 12; Dialogue/Parenthetical 0. Não copiar um template do bundle para reproduzir o caso.

### Resultados comprovados no caso

| Variante | Resultado de exportação real |
|---|---|
| Baseline | 3 páginas físicas; página 1 termina Signal 009; página 2 começa 010 e termina 045; página 3 começa 046 e termina os dois blocos de fala |
| Continuação padrão | `(MORE)` ao final das duas primeiras páginas; `ALFA (CONT’D)` no topo das páginas 2 e 3; o Content conserva um único Dialogue longo, sem esses textos inseridos |
| Lock sem edição | 3 `LockedPage`: PageNumber 1/2/3, Position 0/2666/4574, LockLevel 0 e LevelIndex 0/1/2 |
| Inserir 1.040 caracteres no início do Action após lock | Positions passam a 0/3706/5614; PDF tem **4 páginas físicas**, rótulos 1, 1A, 2, 3; início dos antigos trechos de 2 e 3 preservado |
| Margem 1,5" | A UI mudou em quatro incrementos de 0,125"; FDX gravou `TopMargin=108` |
| Spacing 1,5 e negrito | Action gravou `Spacing=1.5`; somente `Added 001:` recebeu `Text Style=Bold` |
| Margem + Spacing após a inserção/lock | PDF tem **5 páginas físicas**, rótulos 1, 1A, 2, 2A, 3; não é ensaio isolado do efeito de cada parâmetro |

Para a inserção, concatenar com espaços as frases `Added n: the green box stays beside the blue box.` de 001 a 020 e acrescentar um espaço final: exatamente 1.040 caracteres ASCII. Foi usado paste de texto na interface. Não houve edição externa do XML para produzir esses resultados.

**COMPROVADO:** o status AX retornou `1 of 3` mesmo quando o PDF travado tinha quatro ou cinco páginas físicas. Contagem de páginas físicas deve vir da exportação, não dessa string isolada. **INFERIDO:** a interface está contando os rótulos-base nesse estado; isso não foi generalizado.

**COMPROVADO:** o FDX conserva as configurações de continuação em `MoresAndContinueds/DialogueBreaks`; os marcadores visuais não foram acrescentados como parágrafos ao fluxo. **INFERIDO:** são decoração/repetição de layout derivada nesse caso. **DESCONHECIDO:** comportamento ao desligar cada opção, órfãs, parentéticas exatamente na fronteira, quebras manuais, fontes alternativas, A4, exclusão de páginas travadas, relock e unidade de Position com Unicode.

### Referências visuais e ressalva de renderização

Os PDFs reais e os renders estão no diretório local do manifesto. Não foram redesenhados para se parecer com o aplicativo. A GUI da terceira página mostrou texto completo. Houve divergência entre previews/renders exibidos de algumas páginas posteriores, inclusive entre visualizações de PNGs com o mesmo SHA-256. Portanto, não atribuir essa divergência ao paginador do Final Draft, nem homologar cegamente todo PNG como golden. Os PDFs originais, a extração textual, os hashes e a observação da janela ficam preservados; a homologação visual independente das páginas afetadas permanece pendente.

Foram usados Poppler (`pdftoppm` e `pdftocairo` 26.03.0) e uma segunda renderização por MuPDF (binding informou 2.1), em dependência temporária fora do repositório. A terceira página baseline foi legível na conferência individual por MuPDF; as páginas posteriores da variante com margem/spacing continuaram com preview inconclusivo. Trocar o renderer não homologou automaticamente todo o conjunto. Arquivos com `candidate` no nome são tentativas de conferência, **não** uma indicação de aprovação: consultar o status do manifesto.

## Dual dialogue

**COMPROVADO (caso curto):** os seis parágrafos ALFA / primeira fala / parentética / segunda fala / BETA / fala foram selecionados e convertidos com Format → Dual Dialogue. O resultado visual tem duas colunas alinhadas pelo topo, incluindo a parentética na coluna esquerda. A área AX externa passa a mostrar `dd`, não o texto interno. Undo recompõe os seis parágrafos; redo recompõe o objeto. O arquivo foi fechado e reaberto.

O XML salvo tem `Content/Paragraph Type="General"/DualDialogue/Paragraph`. O wrapper ganha um ID; os seis parágrafos internos mantêm seus IDs e tipos. O grupo contém também `SpellCheckIgnoreLists`. Neste caso os parágrafos internos estão em sequência, sem atributo de lane explícito. **INFERIDO:** a separação por blocos Character participa da divisão de colunas; uma regra geral de lane não foi demonstrada.

Format → Edit Dual Dialogue abre o texto interno em sequência. Foi acrescentado ` Confirmed.` à fala de BETA; Format → Close Dual Dialogue Editor voltou à apresentação composta. O texto e o ID da fala direita persistiram dentro do grupo após salvar/reabrir. Logo, `dd` é representação AX externa; não é o conteúdo a exportar/importar.

**COMPROVADO (caso longo):** converter os dois blocos do caso de paginação, incluindo a fala de 3.709 caracteres, produziu wrapper aninhado e preservou IDs internos. O status passou de 3 para 2 páginas e a observação visual mostrou o objeto selecionado transbordando a fronteira das folhas. **DESCONHECIDO:** layout/paginação final desse caso após abrir editor interno, reabrir e exportar. Ele é um caso de stress pendente, não um golden nem uma regra a reproduzir deliberadamente.

## Persistência FDX e diferenças XML

**COMPROVADO em arquivos gerados:** root `FinalDraft`, `DocumentType=Script`, `Template=No`, `Version=6`. A versão de root não é 13.5.0. `Paragraph Type` e `id` existem no fluxo ativo; `Number` fica no heading numerado; formatação de trecho fica em `Text`; propriedades diretas podem ficar no próprio Paragraph; padrões e comportamento ficam em `ElementSettings`. Há também metadados de janela, seleção, SmartType, revisão e produção.

A verificação de reabertura comparou separadamente Content, PageLayout, LockedPages, MoresAndContinueds, ElementSettings e SmartType, sem exigir igualdade de bytes de todo o arquivo:

| Documento final | Comparação após fechar/reabrir/salvar |
|---|---|
| 01-editor | Fluxo ativo, configurações e texto preservados; dois IDs de parágrafos auxiliares em SceneArcBeats/CharacterArcBeat regenerados |
| 02-scenes | Subárvores comparadas iguais, incluindo OmittedScene e números |
| 03-pagination | Fluxo ativo, Bold/Spacing, margem e locks preservados; dois IDs auxiliares em SceneArcBeats/CharacterArcBeat regenerados |
| 04-dual | Subárvores comparadas iguais, incluindo wrapper, fala editada e IDs internos |

O manifesto mantém tanto o resultado **Content desigual** em 01/03 quanto os caminhos e valores que mudaram. A igualdade após retirar apenas esses IDs auxiliares foi verificada separadamente; a normalização não é usada para ocultar diferenças. **COMPROVADO:** “todo Paragraph/@id é único e persistente em toda a árvore” seria falso para este corpus. **DESCONHECIDO:** domínios completos de identidade e política de importação/cópia entre documentos.

## Mudanças necessárias no plano Rust

Estas são revisões documentais de uma proposta; não autorizam desenvolvimento.

| Hipótese / proposta inicial | Evidência nova | Consequência |
|---|---|---|
| Máquina de teclas principalmente por role | Posição, vazio, seleção, sugestão e `ReturnKey` configurável mudam o resultado | Acrescentar configuração por elemento e estado de sugestão ao contrato do handler; não hardcodear a tabela como regra universal |
| ID opcional genérico por parágrafo | IDs ativos estáveis nos casos; IDs repetidos em OmittedScene e regenerados em metadados auxiliares | Separar identidade interna, domínio ativo, conteúdo omitido e IDs auxiliares; não impor unicidade global no XML |
| Cena como índice derivado de headings ativos | Número, ordem, omissão e conteúdo guardado são distintos | Índice em ordem documental; política explícita de cenas omitidas; preservar subárvore e número independentemente |
| Dual talvez grupo de metadados em parágrafos lineares | Wrapper General, objeto AX externo e editor interno comprovados | O modelo/codec precisa preservar hierarquia e seleção interna; representação Rust grupo vs Block continua decisão pendente |
| Paginação mapeável só com propriedades comuns | Continuação derivada e âncoras travadas com páginas 1A/2A | Reaproveitar infraestrutura, mas definir constraints, rótulos e âncoras de produção separados da página física |
| Round-trip como igualdade simples | Arquivo carrega configurações comportamentais e IDs auxiliares normalizados pelo produto | Definir equivalência semântica por domínio e relatar diffs; não prometer byte identity nem estabilidade global de UUIDs |
| Preservar desconhecidos genericamente | OmittedScene/dual têm conteúdo semântico importante | Não rebaixar essas estruturas a texto ou ignorá-las; feature incompleta deve preservar ou recusar com erro explícito |

O [plano revisado](05-wordcraft-rust-implementation-plan.md) mantém a Etapa 0 como barreira por subsistema e exige **nova autorização** antes de qualquer implementação.

## Pendências e reprodução manual

| Pendência | Procedimento reproduzível | Evidência necessária para sair de DESCONHECIDO |
|---|---|---|
| Repetição da matriz e histórico | Novo roteiro sintético por caso; configurar SmartType ligado/desligado; confirmar offset/seleção, executar uma tecla, salvar FDX antes/depois, undo e redo em ações separadas | Tipo de cada parágrafo, texto, seleção, grupo do undo e FDX, incluindo os 20 casos de snapshots não equivalentes |
| Limites de Parenthetical | Posicionar por teclado antes de `(`, depois de `(`, antes/depois de `)`; salvar TextState e conferir a janela | Offset efetivo; não usar apenas a posição solicitada pelo AX |
| Seleção complexa | Selecionar vários parágrafos de tipos diferentes; mudar tipo, Enter/Tab/Backspace; undo/redo | XML de todos os sobreviventes e política de IDs |
| Navigator por drag | Abrir Navigator, fixá-lo se necessário, arrastar uma linha por seu controle visível, conferir ordem e salvar | Movimento efetivo do texto, IDs, números e undo; tentativas desta rodada não validam drag |
| Numeração inicial diferente | Desmarcar Keep existing numbers; preencher 10 por digitação, sair do campo e confirmar o valor antes de aplicar | UI e FDX começando em 10; não reutilizar a tentativa inconclusiva |
| IDs e offsets Unicode | Repetir inserção/lock/cópia/split com `ação`, caractere combinante e emoji; contar bytes, escalares e UTF-16 separadamente | Unidade observada de Position/TextState e política de identidade por operação |
| Continueds e parentéticas na quebra | Recriar baseline; desligar uma opção de continuação por vez; variar uma linha perto da borda; repetir com parentética longa | PDF página a página e XML antes/depois; distinguir texto de decoração |
| Dual longo | Abrir `03-pagination-dual.fdx`, editar e fechar o editor interno, salvar/reabrir/exportar; repetir com alturas assimétricas menores | Referência visual independente, contagem física, fragmentação e comportamento de seleção |
| Locked pages | Remover texto antes de lock, inserir em várias páginas, Unlock A/All, Relock, editar número e usar revisão | Rótulos/âncoras e PDFs após cada comando; nenhum desses fluxos foi inferido como validado |
| Configuração/fontes | Recriar caso com A4 e fonte aberta instalada, registrando nome/versão/hash permitido; variar uma medida por ensaio | Métricas e quebras por configuração; nenhuma equivalência entre fontes foi demonstrada |
| Compatibilidade FDX | Versões distintas, tipos desconhecidos e extensões em arquivos próprios delimitados | Aceitação/rejeição real e preservação semântica, com orçamento de entrada; não foi testado nesta rodada |
| Homologação visual | Abrir PDFs em leitor independente e conferir as páginas cujos previews divergem | Golden visual confiável; manter originais e hashes até esclarecer o pipeline de preview |

Uma decisão de layout ou identidade continua bloqueada **apenas no subsistema que depende dessa pendência**. Não se declarou a automação inteira inviável: criação, teclado, menus, save/reopen e exportação funcionaram; drag, alguns controles e a homologação visual tiveram limites concretos registrados. A tarefa termina na investigação e documentação.

## Verificação dos entregáveis

JSON/CSV e referências locais foram verificados; a matriz contém 140 chaves únicas e cobre o produto cartesiano indicado. As quatro comparações de reabertura conservam texto, propriedades e estrutura, com os diffs auxiliares explicitados. Não há arquivo funcional Rust, workflow ou asset do produto entre as alterações.

Na primeira entrega, `cargo xtask ci` retornou `command not found: cargo` (código 127). Cargo não existia em `~/.cargo/bin`, `/opt/homebrew/bin` ou `/usr/local/bin`; não se instalou toolchain naquela etapa e os documentos ficaram sem commit. Log histórico: `/tmp/finalcraft-fd-ci-20261107.log`. A preparação posterior do ambiente foi autorizada separadamente e está registrada abaixo; o erro inicial não é aprovação do gate.

## Finalização documental e ambiente de integração

Esta etapa trata exclusivamente da validação e integração dos documentos existentes. Não houve nova engenharia reversa, interação com o Final Draft ou implementação de funcionalidades.

| Diagnóstico / requisito | Verificação e configuração utilizada |
|---|---|
| Ambiente do agente | `exec_command` em zsh, macOS 26.6.2 (25G83), arm64, UID 501 (`luizcampos`), HOME `/Users/luizcampos` |
| PATH e shells | O PATH herdado não tinha `~/.cargo/bin`; zsh sem login e zsh interativo de login não encontravam cargo/rustc/rustup. Nenhuma instalação foi encontrada nos diretórios usuais de Rustup, Homebrew, mise ou asdf verificados. Isso descreve esses processos; não presume o estado de uma sessão separada de Terminal.app |
| Causa do bloqueio inicial | Toolchain ausente nos locais verificados, além da ausência do diretório Cargo no PATH; não bastava acrescentar um executável já instalado ao PATH |
| Requisito do projeto | `Cargo.toml`: Rust mínimo 1.95, edition 2024; não há rust-toolchain.toml no checkout; `.cargo/config.toml` define somente o alias xtask |
| Ferramentas nativas | Apple Command Line Tools em `/Library/Developer/CommandLineTools`, clang 21.0.0, alvo arm64 |
| Toolchain instalado | Rustup oficial, perfil minimal, Rust/Cargo 1.95.0, componentes rustfmt e clippy, alvos aarch64-apple-darwin e wasm32-unknown-unknown |
| PATH de validação | `. /Users/luizcampos/.cargo/env` no processo do gate; instalação com `--no-modify-path`, sem editar perfis do shell ou configuração do workspace |
| Idiomas | Checkout externo de craft-i18n em `9d6d54d6d9ff603cc41e12e5f6ba51af3554ab66`, exatamente `craft-i18n.rev` |
| Fontes abertas do projeto | Checkout externo de craft-fonts em `b076417b5880bd4154bd22f00695b1d28c6bea83`, pin do workflow CI existente; nenhuma fonte adicionada a este repositório |
| Diretórios externos | `/Users/luizcampos/.cache/finalcraft-validation/{craft-i18n,craft-fonts,target}`; Cargo/Rustup nos diretórios padrão do usuário |
| Ambiente do gate | CRAFT_I18N_DIR e CRAFT_FONTS_DIR apontam para esses checkouts; ambos REQUIRED=1; CARGO_BUILD_JOBS=2 limita concorrência; CARGO_TARGET_DIR externo |
| Dependências e configuração | Cargo.lock, Cargo.toml, `.cargo/config.toml`, craft-i18n.rev e workflows preservados |

O instalador veio de `https://sh.rustup.rs`, com TLS/HTTPS, e foi executado com `--no-modify-path --profile minimal --default-toolchain 1.95.0 --component rustfmt --component clippy --target wasm32-unknown-unknown`. Essas opções seguem a [documentação oficial do Rustup](https://rust-lang.github.io/rustup/installation/other.html). Não houve instalação privilegiada nem intervenção manual necessária.

A revisão documental corrigiu quatro registros de diferenças de ID no manifesto: os dicionários de atributos precisavam ser copiados antes da normalização das árvores, para não apagar o próprio valor que se pretendia registrar. A correção foi conferida nos FDX já produzidos, sem novos ensaios. Também foram removidos espaços finais de Markdown e normalizados os terminadores do CSV para LF, verificando igualdade de todas as células.

**Gate de integração: REPROVADO.** `cargo xtask ci` foi executado com o ambiente acima e retornou código 1. `fmt --check` e Clippy com `-D warnings` passaram. A etapa `test`, pelo fallback existente para cargo test, executou 2.209 testes antes de parar: 2.208 passaram e um falhou; no binário do engine foram 1.193 aprovados e um reprovado. `i18n-report`, `language`, `doctest`, `assets`, `layers` e `wasm` ficaram sem execução, conforme a parada no primeiro erro do xtask. A instalação do alvo WASM não equivale a aprovação de seu gate.

O teste existente `tests::review_language_validates_tags_and_marks_text_unchecked` falhou em `crates/engine/src/tests.rs:9383`. Ele usa fr-FR como exemplo de idioma sem ferramentas de proofing e espera uma lista vazia. Neste Mac existe o par `fr.aff`/`fr.dic` em `/Applications/LibreOffice.app/Contents/Resources/extensions/dict-fr`, um dos caminhos de sistema consultados pela dependência craft-proof; a execução retornou um erro ortográfico francês para o texto sintético `wrold`. Nenhum recurso desse aplicativo foi copiado, alterado ou incluído no Git.

A falha também foi reproduzida executando **apenas esse teste** no binário já construído pelo gate (código 101), sem alterar entradas, bibliotecas ou fontes. Portanto, essa reprodução não dependeu da execução concorrente dos outros testes. Uma tentativa de recompilação diagnóstica por pacote foi interrompida e substituída pelo binário existente para evitar trabalho redundante.

Não se ocultou o dicionário de sistema, desabilitou proofing, removeu teste, alterou dependência ou modificou código para obter aprovação. A correção exige tornar esse teste independente dos dicionários instalados ou fornecer um ambiente de CI isolado. Docker, Podman e Colima não foram encontrados no PATH verificado; não se instalou infraestrutura adicional para contornar a falha. Corrigir o teste de proofing ultrapassa o escopo documental autorizado.

Logs e ambiente ficaram preservados fora do worktree em `/Users/luizcampos/.cache/finalcraft-validation/logs/2026-11-07/`: `ci.log`, `isolated-language-test-binary.log`, logs de instalação/checkouts e `ci-environment.sh`. O manifesto registra resultados, pins, SHA-256 do log e o commit-base validado. A auditoria dos documentos passou, incluindo equivalência CSV/JSON, links, whitespace, hashes e exclusão dos 74 artefatos do produto do índice Git.

**Integração bloqueada:** nenhum commit foi criado. Os dez arquivos documentais ficaram preparados na branch `codex/final-draft-research-docs`; código funcional, Cargo.lock, manifests e workflows não foram alterados. A falha do gate não foi tratada como aprovação documental nem ignorada para criar o commit.

## Histórico de revisão

| Data | Alteração |
|---|---|
| 2026-11-07 | Finalização documental autorizada: Rust 1.95.0 e insumos fixados, manifesto/whitespace corrigidos; fmt e Clippy aprovados, gate reprovado por teste de proofing existente também reproduzido isoladamente; commit bloqueado |
| 2026-11-07 | Primeira rodada comportamental real: matriz de 140 combinações, quatro documentos reabertos, numeração/omissão, dual, paginação/lock e revisão condicionada do plano; pendências mantidas explícitas |
