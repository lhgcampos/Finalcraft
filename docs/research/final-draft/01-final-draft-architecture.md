# 01 — Arquitetura estática do Final Draft 13

> Status: investigação estática histórica de 2026-11-04, complementada por interface real e FDX sintético em 2026-11-07; implementação não iniciada.
>
> Revisão: 2026-11-07. Evidências comportamentais e limites: [relatório 06](06-behavioral-investigation.md).

## Identidade e cobertura

O bundle é /Applications/Final Draft 13.app. Seu Info.plist declara versão 13.5.0, build 123, bundle id com.finaldraft.finaldraft.v13 e macOS mínimo 10.15. O executável principal é Contents/MacOS/Final Draft 13, Mach-O x86_64 com 23.330.816 bytes. O host da análise é macOS arm64; a arquitetura de um processo em execução não foi observada. SHA-256 do executável: acebfb255174b758200ba8df508b60939dd8ac4da3703fc3f12eed162656497d. O overview registra entry point 0x1004c9dfa, base 0x100000000 e UUID 5d8770d7211d35f1be951049c9f32877. (Craft Evidence: ev_07ac291dd3925caee5633355d846a1453211cb31489782d0795fa699dd9ecfce.)

O Craft Anything já estava em Deep mode (analysis_mode=deep; clean_room=false). Foram habilitados os toolsets native, apple e formats, além de core. O inventário atribui 55, 14 e 36 ferramentas a esses grupos; managed, Android, Java, browser, JavaScript e runtime permaneceram desligados. O analisador nativo embutido é versão 0.3.0.

## Formato, linguagens e dependências

O overview analisou o Mach-O com cobertura parcial: 50.000 funções descobertas (teto do analisador), 45.524 strings, 1.891 imports, 3.705 exports, 281.280 basic blocks e 1.944.936 instruções. Há 2.389 saltos indiretos não resolvidos. A ausência de um símbolo na busca não prova a ausência de um caminho. A imagem também contém 48 seções e quatro segmentos. (ev_07ac…)

Há marcadores fortes de Swift e Objective-C. O código também usa classes e símbolos C++ ligados a libc++; o executável carrega AppKit, Foundation, CoreText/CoreGraphics, WebKit, AVFoundation, SQLite, ICU, Sparkle e Sentry, entre outras 40 bibliotecas. libCon.dylib é outro Mach-O x86_64, ligado a libConfigurer64, Cocoa, AppKit, WebKit e zlib. Seu papel no modelo de roteiro não foi demonstrado. O bundle tem assinatura válida segundo codesign. O executável principal não é universal.

## Subsistemas sugeridos por símbolos

Os nomes recuperados mostram uma camada de domínio C++ chamada FDElement. Entre tipos reconhecíveis estão General, Scene Heading, Action, Character, Parenthetical, Dialogue, Transition, Shot, User Defined, Cast List, Act, New Act, End of Act, Sequence, Outline, Summary e Note. Uma rotina em 0x100051d48 inicializa rótulos de elementos; há construtores C++ separados para vários papéis. (ev_141903…; ev_896a…)

Símbolos também mostram FDSPText, FDEmbeddedDualDialogue e FDScriptPaneTabKeyHandler. Classes com nomes FDXMLNodeReader/Writer, FDXMLTextParagraphNodeReader/Writer e nós para PageLayout apontam a leitores e escritores XML dedicados. Não revelam, sozinhos, os contratos completos entre parser, modelo e editor. A função em 0x10017eef3 usa o tipo FDXMLNodeReader e duas chamadas virtuais não resolvidas; seu pseudo-C é uma reconstrução de leitura, não código original. (ev_9f741…; ev_ea820…; ev_b2af…; ev_c0f655…)

A interface usa recursos AppKit tradicionais: NIBs, storyboard de onboarding e controladores como FMPaigeTextView, FMPageLayoutWindowController e FMSmartTypeWindowController. Recursos e strings identificam Scene Navigator, Scene Numbering, revisões, relatórios, Beat Board e opções de continuação. Um nome de controlador prova presença no pacote, não a semântica integral de cada ação.

## Estrutura FDX conferida no recurso embutido

Foi lido somente /Applications/Final Draft 13.app/Contents/Resources/Script/Screenplay.fdxt, sem alteração ou cópia. Craft Anything classificou o arquivo como desconhecido (SHA-256 571f3b5dae17977f4d5b1b755a4a810afbb33c0c30c416646b4fe316c9492e22); um parser XML local verificou nomes de tags e atributos sem imprimir conteúdo do roteiro.

O root é FinalDraft, com atributos DocumentType, Template e Version. A estrutura inclui Content, Paragraph, Text, PageLayout, ElementSettings, SceneProperties, SceneNumberOptions, SmartType, Characters, Extensions, SceneIntros, Locations, TimesOfDay, Transitions, MoresAndContinueds, DialogueBreaks, SceneBreaks, LockedPages, Revisions, ScriptNavigatorPreferences, DisplayBoards, TagCategories, CharacterTraitData e Actors. Paragraph tem atributo Type; Text tem atributos como Font, Size, Style, RevisionID e AdornmentStyle. PageLayout traz margens e opções de quebra/leading. O template comprova que o FDXT serializa mais do que texto e estilos básicos; não substitui uma especificação completa de FDX nem prova que todo FDX use todas as estruturas. (Craft Evidence da limitação do reconhecimento: ev_f0d8ad9077e8f2bb681e504fd5451d8bc26701b5ac54c1038b9f805f7a652c25.)

## Limitações

A rodada estática de 2026-11-04 não observou um processo em execução nem dirigiu a GUI, criou ou salvou documento. A descoberta de funções parou em 50 mil e 2.389 saltos indiretos não foram resolvidos. A maioria das funções tem nomes gerados e o pseudocódigo traz tipos de registrador não recuperados e branches aproximados. Strings, símbolos, endereços e pseudo-C são evidências distintas, não fonte recuperada. Essas limitações históricas não descrevem a rodada comportamental abaixo.

### Evidências principais

- ev_07ac291… — identidade, bibliotecas, contagens e limitações do analisador.
- ev_141903… e ev_896a573… — tipos e rótulos FDElement.
- ev_00b7e0… e ev_583e18… — dual dialogue e ações de inserção/edição.
- ev_9f741… e ev_ea820… — nomes de leitores/escritores FDX.
- ev_b2af88… e ev_8bf55a… — PageLayout e paginação.
- ev_c0f655… — pseudo-C inferido de função com FDXMLNodeReader.

## Complemento comportamental — 2026-11-07

**COMPROVADO — FD-ENV:** About informou Final Draft 13.5.0 Build 123. A interface real foi usada para criar quatro documentos exclusivamente sintéticos, salvar/reabrir e exportar PDFs. Host: macOS 26.6.2 (25G83), arm64. Não houve nova leitura interna do bundle; arquitetura do processo continua **DESCONHECIDA** nesta rodada.

**COMPROVADO — FD-TYPE/FD-CONFIG:** os arquivos gerados têm root `FinalDraft DocumentType="Script" Template="No" Version="6"`, tipos e IDs explícitos por parágrafo ativo, padrões em ElementSettings e comportamento em `Behavior ReturnKey/TabKey/PaginateAs`. Alterar o destino de Enter de Action para Character na UI mudou o resultado e gravou `ReturnKey="Character"`. Não basta derivar comportamento do nome do tipo.

**COMPROVADO — FD-OMIT/FD-DUAL:** OmittedScene guarda o heading e o conteúdo omitido dentro do marcador ativo; o ID do heading pode aparecer tanto fora quanto dentro da subárvore. DualDialogue fica dentro de um Paragraph General com novo ID e conserva os IDs dos parágrafos internos. Abrir/fechar seu editor interno é comportamento efetivamente observado, não dedução dos símbolos C++.

**COMPROVADO — FD-REOPEN:** IDs do fluxo ativo persistiram nos casos de numeração, recortar/colar, dual e reabertura; alguns IDs de Paragraph em SceneArcBeats/CharacterArcBeat foram regenerados ao reabrir. Logo, unicidade global e estabilidade de todo `Paragraph/@id` não são contratos observados.

**INFERIDO:** existem domínios de identidade e edição composta que um modelo econômico precisa distinguir. **DESCONHECIDO:** contratos privados entre classes, esquema completo de identidade, layout dual longo e compatibilidade entre versões. A interface não validou a arquitetura interna recuperada por nomes de símbolos. Ver [evidências, arquivos locais e procedimentos pendentes](06-behavioral-investigation.md).

## Histórico de revisão

| Data | Alteração |
|---|---|
| 2026-11-07 | Complemento de runtime separado da evidência estática; versão, estruturas geradas, configuração e domínios de IDs |
| 2026-11-04 | Inventário estático original; sem execução do aplicativo |
