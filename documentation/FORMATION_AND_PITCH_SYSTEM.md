# Sistema Espacial, Formações e Coordenadas do Campo

## 1. Visão Geral e Filosofia Arquitetural

No universo do Arlo, o posicionamento tático e a simulação física do esporte operam sobre uma separação estrita entre **templates táticos abstratos** e a **geometria métrica real do estádio**.

Os campos de jogo possuem dimensões físicas regulamentadas que variam entre estádios:
- **Comprimento (`pitch_length_mirim`):** de 140.0 a 150.0 Mirins.
- **Largura (`pitch_width_mirim`):** de 80.0 a 90.0 Mirins.

Se as formações fossem modeladas diretamente em unidades métricas absolutas (Mirins), uma prancheta tática desenhada para um campo de 140x80 ficaria distorcida, comprimida ou descalibrada ao ser executada em um estádio de 150x90.

Para eliminar essa fragilidade, a camada de domínio desacopla a tática das medidas do campo através do conceito de **coordenadas normalizadas relativas**. O objeto `FormationSlot` define a função e a posição tática ideal de cada jogador no espaço normalizado unitário $[0.0, 1.0] \times [0.0, 1.0]$. A conversão para coordenadas métricas em Mirins ocorre dinamicamente na Engine durante a simulação da partida, baseando-se no `Venue` onde o confronto é realizado.

---

## 2. A Estrutura `FormationSlot`

### 2.1 Natureza como Value Object
O `FormationSlot` é um **Value Object** imutável. Ele não possui um identificador único (`Uuid`) próprio no banco de dados nem no domínio. Sua identidade é puramente semântica e estrutural, determinada pelo conjunto dos seus atributos:
1. **`position: Position`** — O papel funcional do jogador em campo (ex: `Goalguard`, `Centerback`, `Passer`, `WingOffense`).
2. **`pitch_length_ratio: f64`** — A coordenada longitudinal relativa no campo.
3. **`pitch_width_ratio: f64`** — A coordenada transversal relativa no campo.

### 2.2 Invariantes e Validações
Durante a instanciação através de `FormationSlot::new(...)`, o domínio garante as seguintes regras:
- `pitch_length_ratio` deve estar estritamente no intervalo contínuo $[0.0, 1.0]$.
- `pitch_width_ratio` deve estar estritamente no intervalo contínuo $[0.0, 1.0]$.
- Qualquer valor fora desse intervalo, assim como valores infinitos ou `NaN`, é rejeitado com erro de invariante de domínio (`DomainError::InvalidInvariant`).

---

## 3. Sistema de Coordenadas Normalizadas

O sistema de coordenadas táticas adota uma convenção padronizada orientada do ponto de vista do time atacante:

```
(0.0, 0.0) [Lateral Esquerda / Fundo Próprio] ------------------- (1.0, 0.0) [Lateral Esquerda / Fundo Adversário]
|                                                                                                                  |
|                                                                                                                  |
|                                         (0.5, 0.5)                                                               |
|                                      [Centro do Campo]                                                           |
|                                                                                                                  |
|                                                                                                                  |
(0.0, 1.0) [Lateral Direita / Fundo Próprio] ------------------- (1.0, 1.0) [Lateral Direita / Fundo Adversário]
```

### 3.1 Eixo Longitudinal (`pitch_length_ratio`) — Eixo X
- **$0.0$:** Linha de fundo defensiva própria (linha do próprio gol / área do Goalguard).
- **$0.5$:** Linha de meio-campo.
- **$1.0$:** Linha de fundo ofensiva adversária (linha do gol adversário).

### 3.2 Eixo Transversal (`pitch_width_ratio`) — Eixo Y
- **$0.0$:** Linha lateral esquerda (ponto de vista do time que ataca em direção a $1.0$).
- **$0.5$:** Corredor central longitudinal do campo.
- **$1.0$:** Linha lateral direita.

---

## 4. Mapeamento para Coordenadas Absolutas na Engine

Quando uma partida é carregada, a Engine recebe a formação de cada equipe e as dimensões físicas reais do `Venue` onde a partida ocorrerá (`pitch_length_mirim` e `pitch_width_mirim`).

### 4.1 Fórmulas de Transformação Linear
Para calcular a posição métrica absoluta $(X, Y)$ em Mirins a partir de um slot normalizado $(r_x, r_y)$:

$$X_{\text{mirim}} = r_x \times \text{pitch\_length\_mirim}$$
$$Y_{\text{mirim}} = r_y \times \text{pitch\_width\_mirim}$$

### 4.2 Exemplo Prático de Adaptação a Estádios Diferentes
Considere um `FormationSlot` para um `Midcenter` posicionado em $(r_x = 0.50, r_y = 0.50)$ e um `WingOffense` em $(r_x = 0.85, r_y = 0.15)$:

1. **Em um estádio compacto ($140.0 \times 80.0$ Mirins):**
   - `Midcenter`: $X = 0.50 \times 140.0 = 70.0\text{ M}$, $Y = 0.50 \times 80.0 = 40.0\text{ M}$
   - `WingOffense`: $X = 0.85 \times 140.0 = 119.0\text{ M}$, $Y = 0.15 \times 80.0 = 12.0\text{ M}$

2. **Em um estádio de grande porte ($150.0 \times 90.0$ Mirins):**
   - `Midcenter`: $X = 0.50 \times 150.0 = 75.0\text{ M}$, $Y = 0.50 \times 90.0 = 45.0\text{ M}$
   - `WingOffense`: $X = 0.85 \times 150.0 = 127.5\text{ M}$, $Y = 0.15 \times 90.0 = 13.5\text{ M}$

Em ambos os casos, a intenção tática (ocupar a meia-cancha central e a ponta avançada esquerda) permanece geometricamente fiel e proporcional às proporções reais do gramado.

---

## 5. Distribuição Espacial por Linhas Táticas e Posições

O esporte organiza seus 14 jogadores titulares (13 de linha + 1 Goalguard) em 4 linhas táticas canônicas (`PositionLine`). Os slots distribuem-se tipicamente nas seguintes faixas de proporção longitudinal:

```
+-----------------------------------------------------------------------------------------------+
|  Goalguard   |      DefenseLine       |        BackLine        |        OffensiveLine         |
| [0.00..0.05] |      [0.08..0.35]      |      [0.35..0.65]      |         [0.65..0.98]         |
+-----------------------------------------------------------------------------------------------+
0.0                                                                                           1.0
```

### 5.1 Linha do Goalguard (`PositionLine::Goalguard`)
- **Faixa longitudinal típica:** $0.00 \le r_x \le 0.05$
- **Posição:** `Goalguard`
- **Comportamento:** Ocupa a proteção imediata da meta e da First Zone.

### 5.2 Linha Defensiva (`PositionLine::DefenseLine`)
- **Faixa longitudinal típica:** $0.08 \le r_x \le 0.35$
- **Posições:** `Centerback`, `DefensiveEnd`, `Rougieback`, `DefensiveBlocker`, `WideBlocker`, `OutsideZonerback`, `MiddleZonerback`
- **Comportamento:** Formam o paredão de contenção, cobertura de espaços e proteção do terço defensivo.

### 5.3 Linha de Apoio / BackLine (`PositionLine::BackLine`)
- **Faixa longitudinal típica:** $0.35 \le r_x \le 0.65$
- **Posições:** `Artrine`, `Passer`, `PassRusher`, `WideEnd`, `RunningEnd`, `Lineback`, `Fullback`
- **Comportamento:** Centro nevrálgico da equipe, responsável pela articulação de jogadas, distribuição do Arlo, pressão ao passador adversário e controle de transição.

### 5.4 Linha Ofensiva (`PositionLine::OffensiveLine`)
- **Faixa longitudinal típica:** $0.65 \le r_x \le 0.98$
- **Posições:** `CenterOffense`, `WingOffense`, `Midcenter`, `TightWing`, `CenterTight`, `Corridor`
- **Comportamento:** Vanguarda de ataque, infiltração nos Artros finais, duelos de finalização e ocupação dos canais laterais e centrais ofensivos.

---

## 6. Malha de Artros e Zonas Regulamentares

O regulamento do esporte estabelece subdivisões espaciais sobre o campo de jogo:
- **Artros:** Marcadores espaçados a cada 3 Mirins ao longo de toda a extensão longitudinal do campo. A quantidade total de Artros em uma partida é calculada como $\lfloor \text{pitch\_length\_mirim} / 3.0 \rfloor$.
- **First Zone:** Zona de 7 Mirins imediatamente à frente de cada linha de meta.
- **Second Zone:** Zona subsequente de profundidade regulamentada de acordo com as regras de cada liga/competição.

### 6.1 Derivação em Tempo de Execução
Nenhum Artro, First Zone ou Second Zone é armazenado no banco de dados como entidade individual. Como todas essas entidades são funções determinísticas diretas de `pitch_length_mirim` e das regras da competição (`Rule`), a Engine calcula e projeta a malha de Artros em memória durante a inicialização do loop de partida.

---

## 7. Dinâmica de Simulação e Espelhamento de Equipes

Durante o ciclo de simulação da partida na Engine:

1. **Posição Base (Âncora Tática):**
   A coordenada absoluta derivada de `FormationSlot` atua como o ponto focal (âncora) para o qual o jogador tende a retornar quando sua equipe está organizada em sua estrutura padrão.

2. **Vetor de Deslocamento Dinâmico:**
   Em tempo de execução, a posição instantânea do jogador diverge da âncora base com base em tomada de decisão, movimentação com e sem a posse do Arlo, marcação individual, cobertura e atributos físicos (Aceleração, Velocidade, Agilidade).

3. **Espelhamento do Adversário:**
   Como ambas as equipes compartilham a convenção $[0.0, 1.0]$, a Engine aplica automaticamente a transformação de espelhamento espacial para a equipe visitante ou que defende o lado oposto do campo:
   $$X_{\text{mundo}} = \text{pitch\_length\_mirim} - X_{\text{absoluto}}$$
   $$Y_{\text{mundo}} = \text{pitch\_width\_mirim} - Y_{\text{absoluto}}$$
   Isso garante simetria matemática perfeita e consistência de colisão e interação física no motor de jogo.
