# IA dos managers

Escopo autorizado pelo proprietário em 2 de outubro de 2026. A análise original orienta a arquitetura; a correção posterior elimina a limitação de três planos. As regras esportivas continuam no AGENTS.md. A calibração técnica foi delegada pelo proprietário e está documentada em MANAGER_AI_AUDIT.md.

## Arquitetura

O fluxo é engine → eventos → runner → analytics → percepção → intenção → engine. A engine não depende do analytics. O runner adapta snapshots e estado para um contexto próprio; percepção, diagnóstico, candidatos, utilidade e escolha trabalham sobre esse contexto.

As avaliações acontecem nas paradas existentes, sem ticks, geometria individual ou enumeração de escalações completas. Simulação em lote e MatchSession utilizam resolve_segment_with_registry. O caminho legado resolve_segment continua disponível sem evidência analítica.

## Entregas

| Etapa | Entrega verificável |
| --- | --- |
| Fundação | Contexto próprio, snapshots vivos e replay dos agregadores |
| Substituição v2 | Qualidade estática, desempenho percebido, confiança, exposição, energia e importância |
| Escolha limitada | Aspiração, opção de manter, erro controlado e escolha determinística entre alternativas próximas |
| Reentrada | Reservas já utilizados, descanso no banco, evidência preservada e custo temporário de retorno |
| Realinhamento | Troca de slots entre ativos, intenção humana, evento próprio e atualização do analytics |
| Planos preparados | Repertório variável, instruções e formações, adaptação aos ocupantes atuais e ativação humana ou automática |
| Diagnóstico coletivo | Escolha conjunta entre substituição, realinhamento e plano; menor atribuição de culpa por baixa produção coletiva |
| Calibração | Três temporadas completas, comparação com a política anterior, cenários dirigidos e verificação dos atributos |

## Identidade e preparação

Um perfil autoral tem precedência. Na ausência dele, atributos e identificação do manager oferecem uma identidade provisória estável. A preparação considera adversário, força relativa, formações preferidas, condição do elenco, histórico de utilização e importância da partida.

Não existe quantidade universal de planos ou mistura obrigatória de orientações ofensiva e conservadora. A primeira entrada descreve a configuração inicial. A preparação examina variações de cinco eixos de instruções e formações do catálogo, admitindo apenas alternativas distintas e compatíveis. Um repertório pode conter várias alternativas ofensivas; limites da busca não definem estilos.

Formações alternativas preservam os quatorze ocupantes e as designações obrigatórias, respeitam funções limitadas e exigem proficiência suficiente nas funções alteradas. O custo e o valor de cobertura permitem preparar uma configuração útil para outro contexto, mesmo quando ela não supera a configuração inicial no confronto esperado.

A adaptação mantém encaixes compatíveis, fixa especialistas e ocupantes indisponíveis e preenche os slots restantes. Quando uma escolha antecipada impede um encaixe viável, um caminho de redistribuição resolve o conflito. A busca é limitada aos quatorze slots; não enumera permutações.

## Percepção e decisão

A habilidade estática permanece separada do desempenho da partida. A percepção utiliza o baseline do analytics e a avaliação de desempenho anterior ao ajuste final por resultado. Confiança, oportunidades e minutos reduzem o peso de pequenas amostras. Erros de percepção não alteram disponibilidade ou proficiência.

Execução, segurança da bola, defesa e disciplina sustentam o diagnóstico individual. Baixa produção isolada e poucas oportunidades não bastam para retirar um atleta. Dificuldade ofensiva coletiva reduz a atribuição individual de culpa. O diagnóstico coletivo considera atacar ou proteger o placar, circulação, carga e disciplina.

| Atributo do manager | Responsabilidade |
| --- | --- |
| JudgingAbility | Fidelidade da percepção individual |
| InGameAdjustments | Leitura das necessidades individuais e coletivas |
| TacticalKnowledge | Avaliação das alternativas e custos de complexidade |
| Adaptability | Amplitude dos candidatos e resistência à mudança |
| OffensePlanning | Resposta às necessidades ofensivas |
| DefenseOrganization | Resposta às necessidades defensivas |
| ArtroStrategy | Circulação, controle e funções de Artrine e Passer |
| LoadManagement | Fadiga e benefício físico esperado |
| Composure | Ruído da percepção sob pressão |
| Discipline | Paciência e estabilidade |

Rigor pertence aos árbitros no catálogo atual. Não participa da paciência do manager como um valor substituto fixo.

Substituições, realinhamentos e planos disputam a mesma lista de utilidades e a mesma aspiração. Nenhuma alternativa suficiente significa manter a configuração. A escolha considera até três candidatos próximos acima da aspiração; esse limite se refere à decisão, não ao repertório preparado.

A aleatoriedade cognitiva usa uma sub-seed estável da partida, equipe, manager, jogador e sequência de decisão. Não existe cache oculto. A engine mantém disponibilidade e tempo de entrada; o analytics fornece evidência e histórico de saídas. Invalidar uma jogada reconstrói os agregadores antes da próxima avaliação.

## Continuidade e disponibilidade

Custos de continuidade protegem titulares, capitães, especialistas e atletas fortes da função. Fadiga e evidência de dificuldade individual reduzem essa proteção. O custo de retorno decai durante trinta minutos no banco. Reservas anteriormente utilizados recuperam energia por segmento, até sua condição inicial; lesão, suspensão e expulsão impedem essa recuperação como reserva disponível.

| Ação automática | Intervalo próprio | Após outra alteração |
| --- | --- | --- |
| Substituição voluntária | 15 minutos | 10 após realinhamento; 15 após plano |
| Realinhamento | 20 minutos | 10 após substituição ou perfil; 15 após plano |
| Plano preparado | 30 minutos | 10 após substituição; 15 após realinhamento ou perfil |

Esses intervalos pertencem à política automática. Intenções humanas conservam as regras da engine. Não há limite de substituições ou ativações de planos por partida. Decisões médicas obrigatórias mantêm a prioridade.

Realinhar troca dois atletas ativos entre slots, preservando as características de cada slot. Ativar um plano adapta a formação aos ocupantes atuais, inclusive reservas utilizados e ocupantes suspensos, expulsos ou retirados. Nenhuma dessas operações adiciona atletas, restaura elegibilidade de Drives, altera relógio, série, posse, placar, energia ou tempo de entrada.

O analytics atualiza atribuições sem zerar tempo em campo, oportunidades, evidência, rating ou confiança. Reentrada preserva a evidência anterior. Uma suspensão que termina durante uma jogada invalidada produz novamente o evento de retorno, pois o relógio transcorrido continua válido.

## Integração e persistência

TacticalRealignmentMade registra duas atribuições. TacticalPlanActivated registra plano, formação, perfil e todas as atribuições atuais. Atores, funções, energia, chutes, abertura de jogadas e arbitragem consultam a configuração efetiva.

As migrações 0118 e 0119 persistem realinhamentos, snapshots de planos e ativações com relógio acumulado. O controller apresenta eventos próprios na timeline. O agregador do manager conta as ações separadamente. Contratos anteriores continuam disponíveis, e novos campos serializados de decisão possuem valores default.

Snapshots JSON utilizam float_roundtrip para recuperar instruções exatamente. A auditoria compara valores completos, incluindo relógios e atribuições, e verifica a timeline após persistência.

## Auditoria reproduzível

Não adicionar testes nem comentários ao código durante esta reconstrução. A validação utiliza auditorias executáveis, saves, compilação, formatação dos arquivos alterados e limites de tamanho.

`cargo run --offline --release -p arlo-controller --example manager_ai_audit -- saves/save-1790979963.db 204`

A origem é aberta somente para leitura. Migrações, preparação e verificações de persistência acontecem em uma cópia consistente. Cada fixture é preparado pelo controller real e executado duas vezes com a mesma entrada, comparando todos os eventos. Os cenários dirigidos acontecem depois do lote.

O modo --prepared-copy aceita apenas cópias próprias com nome arlo-manager-audit-* dentro de target/manager-audit. Um quarto argumento com UUID seleciona um fixture para diagnóstico isolado, sem executar os cenários adicionais. O relatório JSON registra totais e invariantes de disponibilidade, energia, continuidade e reprodução.

history.py audita os bancos históricos em modo somente leitura: integridade SQLite, referências, elencos, contadores, ratings, snapshots e ativações. MANAGER_AI_AUDIT.md registra resultados e parâmetros calibrados.
