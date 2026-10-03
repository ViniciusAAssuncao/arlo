# Auditoria final da IA dos managers

Auditoria e calibração concluídas em 2 de outubro de 2026 na branch beta-version0.1.0-unstable. Todas as etapas do roadmap foram implementadas e verificadas. As decisões de calibração foram delegadas pelo proprietário. MANAGER_AI_AUDIT.json contém os números completos e a identificação criptográfica das três fontes.

## Método e fontes

Foram auditadas as 612 partidas históricas dos três saves, com 204 partidas em cada um. A verificação incluiu integridade SQLite, referências, quatorze titulares por equipe, atletas das substituições, contadores por manager e motivo, ratings, snapshots de planos, atribuições e ativações. Nenhuma inconsistência foi encontrada nos dados históricos.

| Save | Substituições históricas | Planos persistidos | Ativações históricas |
| --- | ---: | ---: | ---: |
| save-1790959886.db | 1.284 | 0 | 0 |
| save-1790974868.db | 1.272 | 0 | 0 |
| save-1790979963.db | 1.274 | 4.298 | 38 |

As mesmas 204 fixtures de cada fonte foram repreparadas pelo controller real com o estado disponível no respectivo save. Foram executados lotes antes e depois da calibração. Cada entrada preparada foi resolvida duas vezes, comparando todos os eventos: 2.448 execuções completas nos lotes de comparação, além dos cenários dirigidos e diagnósticos.

As origens foram abertas somente para leitura. Migrações, geração de escalações e verificações de persistência aconteceram em backups consistentes. Os cenários que alteram atributos na cópia são executados após o lote e restauram os valores antes da simulação. Tamanho, data de modificação e SHA-256 dos arquivos originais foram preservados.

## Resultados finais

| Save | Partidas | Substituições | Planos ativados | Reentradas antes → depois | Jogadas invalidadas |
| --- | ---: | ---: | ---: | ---: | ---: |
| save-1790959886.db | 204 | 1.264 | 32 | 2 → 19 | 57 |
| save-1790974868.db | 204 | 1.291 | 38 | 5 → 20 | 48 |
| save-1790979963.db | 204 | 1.253 | 44 | 4 → 24 | 37 |

O lote final concluiu 612 partidas, sem preparação ou simulação ignorada e sem divergência entre os eventos das duas execuções. Foram 443.004 avaliações e 439.243 escolhas de manter a configuração, aproximadamente 99,15%. As 3.808 substituições se distribuíram em 1.728 por fadiga, 1.790 táticas, 66 disciplinares e 224 médicas.

As reentradas passaram de 11 para 63. Todas as 63 apresentaram energia superior à registrada na saída anterior. Houve quatro retornos antes de quinze minutos e quatro retiradas antes de dez minutos, somando todos os motivos, inclusive médicos. Esses indicadores registram o contexto da continuidade; não representam infrações esportivas.

Dos 114 planos ativados, 102 ocorreram após alguma substituição e oito com a equipe em inferioridade numérica. Três invalidações ocorreram após uma alteração tática anterior. Nenhum candidato utilizou um atleta indisponível e nenhuma ativação automática violou o intervalo próprio da política. Engine e analytics concordaram sobre composição e atribuições após cada segmento.

Não houve troca automática de formação ou realinhamento nos lotes com os elencos originais. A preparação encontrou poucos encaixes alternativos com a proficiência exigida. Os cenários de elenco versátil prepararam 20, 41 e 41 formações alternativas e registraram uma troca automática de formação em cada fonte. Os cenários de titulares deliberadamente mal posicionados produziram correção automática. A política conserva as restrições dos atletas e não estabelece uma frequência obrigatória de mudanças.

## Cobertura do roadmap

| Etapa | Evidência de conclusão |
| --- | --- |
| Fundação | Adapter próprio, snapshots vivos, valores finitos, verificação por segmento e replay integral |
| Substituição v2 | 3.808 decisões finais com qualidade, condição, evidência, confiança, importância e motivos registrados |
| Escolha limitada | Opção de manter em 99,15% das avaliações; alternativas próximas; percepção repetida com a mesma evidência |
| Reentrada | 63 retornos com recuperação; cenário de retirada, descanso e retorno preservando evidência |
| Realinhamento | Correção automática de funções inadequadas; duas permutações comandadas por fonte; persistência e timeline |
| Planos preparados | Repertório variável; ativações naturais; mudança de formação após substituição; snapshots exatos e timeline |
| Diagnóstico coletivo | Utilidades conjuntas e resposta à pressão de um adversário forte com uma alternativa de formação |
| Calibração | Três temporadas completas, comparação anterior/final, dez responsabilidades cognitivas e casos de continuidade |

Os cenários comandados ativaram os planos disponíveis, inclusive o retorno ao principal: 22 ativações na configuração original de cada fonte e 42, 63 e 63 nos elencos versáteis. Verificaram composição, reservas, especialistas, Drives, série, posse, relógio, placar, energia, tempo de entrada, atribuições, evidência, rating e confiança.

Os cenários de continuidade cobriram retorno após descanso, invalidação depois de plano e realinhamento e ativação de plano com menos de quatorze atletas. Preservaram a inferioridade numérica e a elegibilidade de Drives. As sequências que ampliaram a cobertura foram repetidas integralmente, com eventos idênticos. Os lotes dirigidos precisaram de quatro, cinco e duas seeds nas três fontes.

JudgingAbility, InGameAdjustments, TacticalKnowledge, Adaptability, OffensePlanning, DefenseOrganization, ArtroStrategy, LoadManagement, Composure e Discipline foram alterados individualmente, mantendo estado e evidência fixos. Todos afetaram a avaliação. Reavaliar a mesma entrada reproduziu a mesma percepção e conservou disponibilidade.

## Calibrações aplicadas

| Módulo | Decisão |
| --- | --- |
| engine/state/match_state/energy.rs | Reservas já utilizados recuperam uma fração exponencial da energia perdida; escala de 1.800 segundos ajustada pela capacidade física; teto na condição inicial |
| controller/services/season/matchday/prepared_plans/layouts.rs | Cobertura de outras situações entra no valor da preparação; custo de familiaridade 0,10 e de funções alteradas 0,04, modulados por flexibilidade |
| tactics/prepared_plan/adaptation.rs e matching.rs | Redistribuir encaixes quando o preenchimento guloso bloqueia uma solução viável; especialistas e ocupantes indisponíveis continuam fixos |
| match-runner/manager_ai/realignment.rs | Custo básico 0,015; resistências por adaptação e conhecimento 0,015 e 0,010; proficiência insuficiente custa 0,05; continuidade mantida |
| match-runner/manager_ai/plans.rs | Custo proporcional de funções alteradas 0,05; formação custa 0,015 mais até 0,025 por conhecimento insuficiente |
| match-runner/manager_ai/adapter.rs | Paciência usa Discipline do manager; Rigor foi excluído dessa leitura porque pertence aos árbitros no catálogo |

Os intervalos de quinze minutos para substituição, vinte para realinhamento e trinta para plano foram mantidos. Os custos de capitães, especialistas, importância, amostra incerta e retorno recente continuam participando da escolha. Nenhuma calibração estabelece três perfis, cotas de substituição ou frequência obrigatória de ações.

## Falha encontrada e corrigida

Na fixture 3ae72e14-4afe-43ad-97c1-3b21d4d0ac86 do segundo save, uma suspensão terminou durante uma jogada depois invalidada. A engine restaurava o checkpoint com o relógio transcorrido, mantendo o atleta liberado, mas descartava o evento de retorno. O analytics permanecia com um atleta a menos.

O officiating agora emite as alterações de disponibilidade reconstruídas logo após PlayInvalidated. A partida foi reproduzida isoladamente antes da correção e concluída duas vezes depois dela. O relatório de regressão registra um retorno de suspensão após invalidação, zero partidas ignoradas e eventos reproduzíveis. O lote completo do segundo save também atravessou esse caso com engine e analytics sincronizados.

## Verificações e reprodução

Passaram cargo check --offline --workspace --all-targets, compilação release do auditor, formatação dos 113 arquivos Rust trabalhados e git diff --check. O maior arquivo Rust trabalhado tem 427 linhas; o maior módulo novo tem 230. Nenhum comentário de código ou teste foi adicionado.

Para repetir um lote sem modificar a origem:

`cargo run --offline --release -p arlo-controller --example manager_ai_audit -- saves/save-1790979963.db 204`

Para auditar os registros históricos:

`python crates/arlo-controller/examples/manager_ai_audit/history.py saves/save-1790959886.db saves/save-1790974868.db saves/save-1790979963.db --output target/manager-audit/history.json`

Os relatórios finais, logs e cópias de trabalho desta execução estão em target/manager-audit. MANAGER_AI_AUDIT.json conserva os totais e hashes no repositório. As etapas previstas estão encerradas; ajustes futuros de pesos podem partir desses registros e do auditor executável.
