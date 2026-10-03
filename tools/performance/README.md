# Auditoria de desempenho do Arlo

Utilitário separado do workspace de produção. Executa preparação real, engine, IA, aggregators e persistência real em cópias de saves. Não acrescenta testes ao projeto nem instala instrumentação na aplicação.

Os resultados e as decisões desta rodada estão em [REPORT.md](REPORT.md). Os dados consolidados ficam em `results.json`; entradas congeladas, bancos descartáveis e relatórios completos ficam em `target/performance`, que já é ignorado pelo Git.

Depois de concluir as rodadas descritas no relatório, `python tools/performance/results.py` reconsolida suas métricas e confere novamente os hashes dos saves originais. As medições finais usam os relatórios `isolated-before`, `isolated-after` e `isolated-historical`; execuções antigas com concorrência de disco ou compilação ficam apenas como diagnóstico.

## Compilação

Na raiz do repositório:

```powershell
cargo build --manifest-path tools/performance/Cargo.toml --release --offline --target-dir target
```

Usar `--target-dir target` evita criar um diretório de compilação dentro desta ferramenta.

## Cópia protegida do save

Feche qualquer escritor do save antes da cópia. O utilitário recusa um WAL com conteúdo pendente e abre o original com `mode=ro&immutable=1`. A cópia é feita pela API de backup do SQLite, e a função registra SHA-256 do original. O destino deve ser novo e estar sob `target/performance`.

```powershell
python tools/performance/audit.py snapshot saves/save-1790959886.db target/performance/workload.db
```

A CLI Rust também recusa bancos fora de `target/performance`. Ela aplica as migrações normais da aplicação apenas na cópia.

## Preparação e entradas congeladas

```powershell
$env:ARLO_PERF_LABEL='preparation'
target/release/arlo-performance.exe capture target/performance/workload.db target/performance/workload 204 1
```

`capture` prepara os fixtures em ordem cronológica, persistindo escalações, perfis e playbooks como a aplicação. Os arquivos `.input.json` congelam seed, identificadores, formações, jogadores, condições iniciais, managers, planos, perfis, árbitros e play calls. Catálogos de faltas e lesões são carregados da mesma cópia do save.

Para comparar preparação em duas versões, faça cópias independentes do mesmo original e use diretórios de saída diferentes:

```powershell
python tools/performance/audit.py compare-captures target/performance/preparation-before target/performance/preparation-after
```

Essa comparação normaliza somente identidades geradas para partidas, escalações, perfis, layouts, planos e play calls. Jogadores, managers, formações, atributos, instruções e relações continuam sendo comparados.

## Simulação, decisões e alocações

```powershell
$env:ARLO_PERF_NO_WRITE='1'
$env:ARLO_PERF_LABEL='cpu'
target/release/arlo-performance.exe run target/performance/workload.db target/performance/workload 12 3
```

`run` faz uma primeira execução, repetições cronometradas, uma execução instrumentada por fase e uma execução para contar alocações. O relógio principal inclui construção do estado e registry, engine, decisões da IA, agregação e retenção dos eventos. Exclui leitura do JSON, construção do MatchInput, validação dos resultados e SQL. A primeira execução também é registrada separadamente.

O perfil separa dispatch em jogo aberto, stoppages e outras fases, agregação e replay de invalidações. `assessment_ms` mede sondagens adicionais das duas IAs em cada stoppage; esse valor não deve ser somado ao tempo normal da partida. Candidatos, utilidades, diagnóstico e ação selecionada dessas sondagens são comparados entre versões.

`allocation_count` inclui alloc e realloc. `allocated_bytes` é o volume cumulativo solicitado, inclusive realocações; não é consumo máximo de RAM. A contagem fica desligada nas execuções cronometradas.

Para uma temporada inteira com uma execução por partida:

```powershell
$env:ARLO_PERF_LABEL='season-cpu'
target/release/arlo-performance.exe cpu target/performance/workload.db target/performance/workload 204 1
```

O primeiro resultado cria `.expected.json`; versões seguintes devem reutilizar o mesmo diretório. O hash SHA-256 de toda a sequência de eventos precisa coincidir exatamente. Estatísticas, analytics, estado final e avaliações são comparados recursivamente. Apenas floats admitem diferença absoluta de até `1e-10`, necessária para reduções sobre HashMap que já variavam na versão anterior. Inteiros, IDs, ordem, ações e eventos não recebem tolerância.

## Persistência

```powershell
Remove-Item Env:ARLO_PERF_NO_WRITE -ErrorAction SilentlyContinue
$env:ARLO_PERF_LABEL='persistence'
target/release/arlo-performance.exe verify target/performance/workload.db target/performance/workload 204 1
```

`verify` simula, confere o resultado e persiste uma vez, separando tempo dos statements e do commit. Use uma cópia nova dos bancos capturados para cada versão: o MatchPersister não deve receber duas inserções da mesma partida.

Sem `ARLO_PERF_NO_WRITE`, `run` também mede persistência após cada repetição: faz rollback nas primeiras e commit na última. Rollback aquece páginas e statements; não confunda essas amostras com a primeira gravação em banco frio.

O utilitário usa uma transação por partida. A aplicação reúne as partidas do dia em uma transação. Portanto, seus commits e checkpoints não representam diretamente a latência total de um dia simulado.

```powershell
python tools/performance/audit.py compare-databases target/performance/before.db target/performance/after.db target/performance/workload
python tools/performance/audit.py summarize target/performance/workload/persistence.report.json
```

São comparadas todas as tabelas com `match_id`, além de `matches`, para os IDs congelados. Somente `id` de linhas geradas e `created_at_unix_seconds` são excluídos. O ID da própria partida, identidades de entidades, contagens e referências são mantidos. JSON é comparado estruturalmente, e a mesma tolerância numérica é aplicada. A cópia de destino também passa por `integrity_check` e `foreign_key_check`.

## Contagem de SQL

```powershell
$env:ARLO_PERF_SQL_TRACE='1'
$env:ARLO_PERF_LABEL='sql-count'
target/release/arlo-performance.exe capture target/performance/sql-count.db target/performance/sql-count 12 1
Remove-Item Env:ARLO_PERF_SQL_TRACE
```

Um Subscriber registra os eventos `sqlx::query`, sem imprimir SQL ou parâmetros. As contagens são exatas para as operações registradas pelo driver; `query_elapsed_ms` inclui o tempo observado pelo SQLx, inclusive agendamento e entrega das linhas. Essa rodada serve para diagnóstico, separada do benchmark com tracing desligado.

## Comparação histórica

Os commits usados nesta auditoria foram `834fb4d`, antes desta otimização, e `394bc88`, anterior ao épico de managers. Foram extraídos com `git archive` sob `target/performance`, sem modificar checkout ou histórico.

Copie esta pasta para `tools/performance` dentro do archive. A feature `pre-epic` adapta a ferramenta às APIs históricas:

```powershell
cargo build --manifest-path target/performance/pre-epic/tools/performance/Cargo.toml --features pre-epic --release --offline --target-dir target
```

Preserve cada executável com nome diferente, porque as compilações usam o mesmo diretório de saída.

Git archive pode extrair migrações com LF quando o checkout usado para produzir o save tinha CRLF. Isso altera o checksum, sem alterar SQL. Apenas na cópia descartável destinada ao código histórico:

```powershell
python tools/performance/audit.py normalize-migrations target/performance/historical.db target/performance/pre-epic
```

Esse comando modifica somente checksums de migrações conhecidas no banco descartável. Não deve ser usado para acomodar uma migração SQL realmente alterada.

A execução histórica possui seu próprio diretório de entradas e resultados. Seu comportamento é anterior ao épico; não se espera igualdade de eventos com a IA atual.
