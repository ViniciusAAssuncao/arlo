# Sistema Posicional, Dinâmica de Linhas e Funções Táticas do Arlo

## 1. Estrutura Geral e Dinâmica do Arlo (14 vs 14)

Uma partida de Arlo é disputada por duas equipes de **14 jogadores titulares em campo** (13 jogadores de linha + 1 Goalguard), totalizando 28 atletas ativos simultaneamente. 

```
                                  CAMPO DE JOGO DE ARLO
                     (Comprimento: 140-150 Mirins | Largura: 80-90 Mirins)
0.0                                           0.5                                           1.0
┌──────────────────────┬───────────────────────┬───────────────────────┬──────────────────────┐
│      GOALGUARD       │     DEFENSE LINE      │       BACK LINE       │    OFFENSIVE LINE    │
│    [0.00 - 0.05]     │     [0.08 - 0.35]     │     [0.35 - 0.65]     │    [0.65 - 0.98]     │
├──────────────────────┼───────────────────────┼───────────────────────┼──────────────────────┤
│ G: Goalguard         │ CB: Centerback        │ A: Artrine            │ C-O: Center-Offense  │
│                      │ DE: Defensive End     │ P: Passer             │ W-O: Wing-Offense    │
│                      │ RB: Rougieback        │ P-R: Pass-Rusher      │ MC: Midcenter        │
│                      │ D-B: Def. Blocker     │ W-E: Wide-End         │ TW: Tight Wing       │
│                      │ W-B: Wide Blocker     │ R-E: Running-End      │ CW: Center-Tight     │
│                      │ OZB: Outside Zoner    │ L: Lineback           │ C: Corridor          │
│                      │ MZB: Middle Zoner     │ F: Fullback           │                      │
└──────────────────────┴───────────────────────┴───────────────────────┴──────────────────────┘
```

### 1.1 Princípios de Funcionamento
1. **Ausência de Troca de Pelotões:** Não existem unidades separadas de ataque e defesa que entram e saem do campo como no futebol americano. Todos os 14 atletas permanecem em campo e desempenham responsabilidades contínuas.
2. **Offense e Defense como Estados de Posse:**
   - **Offense:** Estado da equipe que detém a posse do arlo após o Call-to-Action ou após recuperação contínua.
   - **Defense:** Estado da equipe sem a posse, que atua na contenção espacial, pressão e bloqueios defensivos.
3. **Turnover Contínuo vs. Out:** 
   - Se a posse for perdida em campo aberto sem que a bola saia, a transição entre Offense e Defense é imediata e fluida, sem reinício de jogada.
   - Se houver saída de bola (Out), falta ou paralisação arbitral, aciona-se o **Countdown to Size**, organizando os times na linha de escarmouche para um novo Call-to-Action.
4. **Posições Obrigatórias Rígidas:** Em qualquer formação válida, é obrigatória a designação de exatamente:
   - 1 **Goalguard (G)**
   - 1 **Passer (P)**
   - 1 **Artrine (A)**

---

## 2. As 4 Linhas Canônicas e suas 21 Posições

### 2.1 Linha Ofensiva (Offensive Line - OL)

```
                       LINHA OFENSIVA (OFFENSIVE LINE)
   
        [W-O] --------------------- [C-O] --------------------- [W-O]
     (Ponta Aberto)             (Centroavante)               (Ponta Aberto)
             \                       |                       /
              \                      |                      /
             [TW] ----------------- [CW] ----------------- [TW]
         (Ala Bloqueador)       (Bloqueador Central)   (Ala Bloqueador)
                     \               |               /
                      \              |              /
                      [MC] -------- [C] -------- [MC]
                   (Armador Curto) (Corredor) (Armador Curto)
```

#### Center-Offense (C-O)
- **Código:** `C-O`
- **Linha:** `PositionLine::OffensiveLine`
- **Envelope Espacial:** $r_x \in [0.80, 0.98]$, $r_y \in [0.35, 0.65]$
- **Função Mecânica:** Principal finalizador e artilheiro. Posiciona-se no terço final, fixa Centerbacks adversários, ataca a First e Second Zone e finaliza para Goal Point (5 pts) ou Field Point (3 pts).
- **Atributos Primários:** Finalização / Chute, Impulsão, Posicionamento, Compostura, Força.
- **Atributos Secundários:** Controle do Arlo, Técnica, Bravura, Decisões.

#### Wing-Offense (W-O)
- **Código:** `W-O`
- **Linha:** `PositionLine::OffensiveLine`
- **Envelope Espacial:** $r_x \in [0.70, 0.95]$, $r_y \in [0.00, 0.20] \cup [0.80, 1.00]$
- **Função Mecânica:** Ponta agressivo de velocidade. Fornece amplitude máxima, vence defensores em duelos de drible lateral e executa cruzamentos rápidos e entregas diagonais para C-O e Runners.
- **Atributos Primários:** Velocidade (Pace), Aceleração, Drible / Condução, Cruzamento / Lateral.
- **Atributos Secundários:** Agilidade, Técnica, Resistência, Visão de Jogo.

#### Midcenter (MC)
- **Código:** `MC`
- **Linha:** `PositionLine::OffensiveLine`
- **Envelope Espacial:** $r_x \in [0.60, 0.80]$, $r_y \in [0.25, 0.75]$
- **Função Mecânica:** Meio-campista articulador auxiliar. Funciona como estação de distribuição rápida e pivô imediato do Artrine, operando entre a linha de escarmouche e a zona de finalização.
- **Atributos Primários:** Passe / Lançamento, Visão de Jogo, Decisões, Controle do Arlo.
- **Atributos Secundários:** Técnica, Compostura, Trabalho em Equipe, Posicionamento.

#### Tight Wing (TW)
- **Código:** `TW`
- **Linha:** `PositionLine::OffensiveLine`
- **Envelope Espacial:** $r_x \in [0.65, 0.85]$, $r_y \in [0.15, 0.35] \cup [0.65, 0.85]$
- **Função Mecânica:** Ala híbrido de força e técnica. Executa bloqueios na segunda linha defensiva abrindo canais laterais para o Corridor e se apresenta como opção de recepção intermediária.
- **Atributos Primários:** Bloqueio Ofensivo, Força, Resistência, Equilíbrio.
- **Atributos Secundários:** Passe / Lançamento, Recepção / Hands, Trabalho em Equipe, Aceleração.

#### Center-Tight (CW)
- **Código:** `CW`
- **Linha:** `PositionLine::OffensiveLine`
- **Envelope Espacial:** $r_x \in [0.65, 0.85]$, $r_y \in [0.40, 0.60]$
- **Função Mecânica:** Muralha e âncora de bloqueio central. Responsável pelo impacto físico primário, abrindo brechas na defesa central adversária para o avanço do Artrine e do Corridor.
- **Atributos Primários:** Bloqueio Ofensivo, Força, Equilíbrio, Agressividade Controlada.
- **Atributos Secundários:** Bravura, Determinação, Trabalho em Equipe, Posicionamento.

#### Corridor (C)
- **Código:** `C`
- **Linha:** `PositionLine::OffensiveLine`
- **Envelope Espacial:** $r_x \in [0.65, 0.90]$, $r_y \in [0.30, 0.70]$
- **Função Mecânica:** Especialista em ganho de mirins por progressão terrestre. Recebe passes curtos ou handoffs do Artrine e rompe linhas adversárias quebrando tentativas de contenção controlada.
- **Atributos Primários:** Aceleração, Velocidade (Pace), Drible / Condução, Equilíbrio, Força.
- **Atributos Secundários:** Agilidade, Bravura, Determinação, Controle do Arlo.

---

### 2.2 Linha de Conexão e Comando (Back Line - BL)

```
                          BACK LINE (LINHA DE COMANDO)

             [P-R] ----------------- [P] ----------------- [P-R]
         (Pass-Rusher)             (Passer)            (Pass-Rusher)
               \                      |                      /
                \                     |                     /
               [W-E] --------------- [A] ----------------- [W-E]
            (Wide-End)            (Artrine)             (Wide-End)
                 \                    |                    /
                  \                   |                   /
                  [R-E] ------------ [L] --------------- [F]
              (Running-End)       (Lineback)          (Fullback)
```

#### Artrine (A)
- **Código:** `A`
- **Linha:** `PositionLine::BackLine`
- **Envelope Espacial:** $r_x \in [0.40, 0.60]$, $r_y \in [0.35, 0.65]$
- **Função Mecânica:** Eixo central, cérebro e capitão tático do time. É o **único jogador que valida Drives** ao atravessar Artros. Recebe o passe inicial do Passer no Call-to-Action, lê a defesa adversária e comanda as variações táticas em tempo real através do microcomunicador com o técnico.
- **Atributos Primários:** Visão de Jogo, Decisões, Técnica de Drive, Passe / Lançamento, Liderança, Compostura.
- **Atributos Secundários:** Técnica, Controle do Arlo, Agilidade, Concentração, Antecipação.

#### Passer (P)
- **Código:** `P`
- **Linha:** `PositionLine::BackLine`
- **Envelope Espacial:** $r_x \in [0.35, 0.50]$, $r_y \in [0.40, 0.60]$
- **Função Mecânica:** Iniciador oficial de cada Action no Size. Responsável pelo lançamento inicial obrigatório para o Artrine no Call-to-Action e segundo cérebro tático do elenco.
- **Atributos Primários:** Passe / Lançamento, Decisões, Compostura, Visão de Jogo.
- **Atributos Secundários:** Técnica, Concentração, Equilíbrio, Liderança.

#### Pass-Rusher (P-R)
- **Código:** `P-R`
- **Linha:** `PositionLine::BackLine`
- **Envelope Espacial:** $r_x \in [0.45, 0.65]$, $r_y \in [0.20, 0.80]$
- **Função Mecânica:** Caçador de linha de escarmouche. Em fase de Defense, sua missão é quebrar bloqueios com velocidade pura e forçar turnover ou passe imperfeito do Passer adversário antes do domínio do Artrine.
- **Atributos Primários:** Pressão / Rush, Aceleração, Força, Agressividade Controlada.
- **Atributos Secundários:** Agilidade, Antecipação, Bravura, Velocidade (Pace).

#### Wide-End (W-E)
- **Código:** `W-E`
- **Linha:** `PositionLine::BackLine`
- **Envelope Espacial:** $r_x \in [0.50, 0.75]$, $r_y \in [0.05, 0.25] \cup [0.75, 0.95]$
- **Função Mecânica:** Receptor vertical profundo. Estica a defesa adversária nas laterais criando linhas de passe longas e isolamentos aéreos contra defensores zonadores.
- **Atributos Primários:** Recepção / Hands, Velocidade (Pace), Aceleração, Impulsão.
- **Atributos Secundários:** Agilidade, Controle do Arlo, Decisões, Concentração.

#### Running-End (R-E)
- **Código:** `R-E`
- **Linha:** `PositionLine::BackLine`
- **Envelope Espacial:** $r_x \in [0.45, 0.70]$, $r_y \in [0.25, 0.75]$
- **Função Mecânica:** Conector híbrido de média distância. Executa rotas intermediárias, quebra tentativas iniciais de contenção e oferece ganho consistente de mirins terrestres após o passe do Artrine.
- **Atributos Primários:** Recepção / Hands, Força, Equilíbrio, Drible / Condução.
- **Atributos Secundários:** Aceleração, Trabalho em Equipe, Determinação, Controle do Arlo.

#### Lineback (L)
- **Código:** `L`
- **Linha:** `PositionLine::BackLine`
- **Envelope Espacial:** $r_x \in [0.35, 0.55]$, $r_y \in [0.30, 0.70]$
- **Função Mecânica:** Marechal da retaguarda e líder defensivo em campo. Lê a movimentação do Artrine adversário, comanda os ajustes da linha de contenção e fecha o miolo do campo contra corridas.
- **Atributos Primários:** Posicionamento, Antecipação, Liderança, Contenção Defensiva, Decisões.
- **Atributos Secundários:** Força, Comunicação, Trabalho em Equipe, Compostura.

#### Fullback (F)
- **Código:** `F`
- **Linha:** `PositionLine::BackLine`
- **Envelope Espacial:** $r_x \in [0.40, 0.60]$, $r_y \in [0.30, 0.70]$
- **Função Mecânica:** Operário de proteção e sacrifício tático. Atua como escudo protetor do Artrine em jogadas de passe longo e como bloqueador móvel de avanço em corridas do Corridor.
- **Atributos Primários:** Bloqueio Ofensivo, Força, Bravura, Trabalho em Equipe, Equilíbrio.
- **Atributos Secundários:** Contenção Defensiva, Resistência, Agressividade Controlada, Posicionamento.

---

### 2.3 Linha Defensiva (Defense Line - DL)

```
                         LINHA DEFENSIVA (DEFENSE LINE)

         [DE] --------------------- [CB] --------------------- [DE]
      (Ponta Defensivo)        (Zagueiro Central)        (Ponta Defensivo)
              \                      |                      /
               \                     |                     /
              [W-B] --------------- [D-B] --------------- [W-B]
          (Bloqueador Lateral)   (Bloqueador Central)   (Bloqueador Lateral)
                     \               |               /
                      \              |              /
                     [OZB] -------- [RB] -------- [MZB]
                   (Zonador Ext.) (Rougieback) (Zonador Médio)
```

#### Centerback (CB)
- **Código:** `CB`
- **Linha:** `PositionLine::DefenseLine`
- **Envelope Espacial:** $r_x \in [0.10, 0.25]$, $r_y \in [0.35, 0.65]$
- **Função Mecânica:** Último homem da linha defensiva antes da First Zone. Marca individualmente o Center-Offense adversário, rebate cruzamentos e bloqueia arremessos frontais.
- **Atributos Primários:** Posicionamento, Contenção Defensiva, Impulsão, Força, Antecipação.
- **Atributos Secundários:** Bravura, Decisões, Concentração, Equilíbrio.

#### Defensive End (DE)
- **Código:** `DE`
- **Linha:** `PositionLine::DefenseLine`
- **Envelope Espacial:** $r_x \in [0.15, 0.35]$, $r_y \in [0.00, 0.20] \cup [0.80, 1.00]$
- **Função Mecânica:** Guarda das extremidades laterais. Contém infiltrações dos Wing-Offenses e força o portador do arlo em direção ao miolo congestionado da defesa.
- **Atributos Primários:** Velocidade (Pace), Aceleração, Contenção Defensiva, Agilidade.
- **Atributos Secundários:** Força, Posicionamento, Trabalho em Equipe, Resistência.

#### Rougieback (RB)
- **Código:** `RB`
- **Linha:** `PositionLine::DefenseLine`
- **Envelope Espacial:** $r_x \in [0.20, 0.35]$, $r_y \in [0.30, 0.70]$
- **Função Mecânica:** Destruidor especializado em combate corpo a corpo contra o Corridor. Interrompe avanços terrestres com trancos no tronco e rasteiras controladas rigorosamente dentro da regra.
- **Atributos Primários:** Contenção Defensiva, Força, Bravura, Agressividade Controlada, Equilíbrio.
- **Atributos Secundários:** Decisões, Aceleração, Posicionamento, Determinação.

#### Defensive Blocker (D-B)
- **Código:** `D-B`
- **Linha:** `PositionLine::DefenseLine`
- **Envelope Espacial:** $r_x \in [0.15, 0.30]$, $r_y \in [0.35, 0.65]$
- **Função Mecânica:** Bloqueador de trincheira central. Trava o Center-Tight adversário e impede a abertura de corredores de infiltração nos Artros centrais.
- **Atributos Primários:** Força, Equilíbrio, Contenção Defensiva, Agressividade Controlada.
- **Atributos Secundários:** Bravura, Trabalho em Equipe, Posicionamento, Resistência.

#### Wide Blocker (W-B)
- **Código:** `W-B`
- **Linha:** `PositionLine::DefenseLine`
- **Envelope Espacial:** $r_x \in [0.15, 0.35]$, $r_y \in [0.10, 0.30] \cup [0.70, 0.90]$
- **Função Mecânica:** Defensor móvel de flanco intermediário. Neutraliza os Tight Wings e Wing-Ends adversários em jogadas de bloqueio e rotas cruzadas.
- **Atributos Primários:** Contenção Defensiva, Força, Agilidade, Posicionamento.
- **Atributos Secundários:** Aceleração, Equilíbrio, Trabalho em Equipe, Resistência.

#### Outside Zonerback (OZB)
- **Código:** `OZB`
- **Linha:** `PositionLine::DefenseLine`
- **Envelope Espacial:** $r_x \in [0.20, 0.35]$, $r_y \in [0.05, 0.25] \cup [0.75, 0.95]$
- **Função Mecânica:** Defensor espacial periférico. Cobre zonas laterais intermediárias, intercepta passes em curva e fecha rotas oblíquas de avanço.
- **Atributos Primários:** Posicionamento, Antecipação, Agilidade, Velocidade (Pace).
- **Atributos Secundários:** Concentração, Decisões, Contenção Defensiva, Visão de Jogo.

#### Middle Zonerback (MZB)
- **Código:** `MZB`
- **Linha:** `PositionLine::DefenseLine`
- **Envelope Espacial:** $r_x \in [0.20, 0.35]$, $r_y \in [0.35, 0.65]$
- **Função Mecânica:** Sentinela do espaço central entrelinhas. Intercepta passes médios destinados aos Midcenters e bloqueia investidas frontais do Artrine verdadeiro.
- **Atributos Primários:** Antecipação, Posicionamento, Visão de Jogo, Contenção Defensiva, Concentração.
- **Atributos Secundários:** Decisões, Agilidade, Força, Trabalho em Equipe.

---

### 2.4 Linha do Goalguard (Meta)

```
                            LINHA DO GOALGUARD
                       (Proteção da First Zone)
   
                 [Fieldpost - Trave H (3 pts / 1 pt FG)]
                                   │
                                   │
               [Goalpost - Baliza Principal (5 pts / 2 pts FG)]
                                   │
                                   │
                            ┌──────────────┐
                            │ [G] Goalguard│
                            └──────────────┘
```

#### Goalguard (G)
- **Código:** `G`
- **Linha:** `PositionLine::Goalguard`
- **Envelope Espacial:** $r_x \in [0.00, 0.05]$, $r_y \in [0.40, 0.60]$
- **Função Mecânica:** Único jogador autorizado a usar as mãos dentro da First Zone (7 Mirins da meta). Protege a baliza retangular (Goalpost) e a trave superior em H (Fieldpost), organiza a linha defensiva e efetua recomeços de contra-ataque.
- **Atributos Primários:** Reflexos, Handling / Mãos, Comando de Área, Posicionamento GK, Um contra Um.
- **Atributos Secundários:** Alcance Aéreo GK, Comunicação, Distribuição, Agilidade, Concentração.

---

## 3. Funções Especiais e Sobreposição de Papéis

### 3.1 Funções Especiais (Special Teams)

| Função Especial | Posições Típicas de Origem | Comportamento no Motor de Jogo |
| :--- | :--- | :--- |
| **Falso Artrine** | Corridor, Wide-End, Midcenter | Executa rotas idênticas às do Artrine legítimo e carrega microcomunicador réplica inativo para atrair marcadores. Não valida Drives. |
| **Launcher** | Passer, Midcenter, Artrine | Assume a posse em jogadas ensaiadas para disparar lançamentos com alcance $\ge 40$ Mirins. |
| **Safeguard** | Fullback, Center-Tight, Center-Offense | Alinha-se adjacente ao Artrine exclusivamente para absorver o impacto do Pass-Rusher. |
| **Blocker** | Center-Tight, Tight Wing, Fullback | Atua como obstáculo cinemático dinâmico para abrir rotas livres de marcação para o portador. |
| **Kicker** | Center-Offense, Artrine, Goalguard | Assume a cobrança de chutes de média e longa distância visando o Goalpost ou Fieldpost. |

### 3.2 Matriz de Sobreposição e Plasticidade Funcional
Em virtude da regra de que todos os atletas permanecem em campo durante as transições de posse, as seguintes sobreposições dinâmicas ocorrem automaticamente:

```
Center-Offense (C-O) ──[Pressão Defensiva]──► Safeguard / Blocker
Lineback (L)         ──[Transição Ofensiva]──► Corridor / Midcenter
Tight Wing (TW)      ──[Fase sem Posse]────► Wide Blocker (W-B)
Midcenter (MC)       ──[Jogada Ensaiada]───► Falso Artrine / Launcher
```

---

## 4. Arquiteturas Táticas e Formações Clássicas

### 4.1 Formação 4-3-3-4 (Ofensiva / Agressiva)
- **Estrutura:** 4 Defensores (CB, CB, DE, DE), 3 Back Line (A, P, L), 3 Ofensivos Suporte (CW, MC, TW), 4 Atacantes (C-O, W-O, W-O, C).
- **Finalidade:** Pressão alta na saída de posse adversária, sobrecarga nas pontas e velocidade de finalização em Goal Point.

### 4.2 Formação 5-2-4-3 (Defensiva / Compacta)
- **Estrutura:** 5 Defensores (CB, CB, D-B, W-B, W-B), 2 Back Line (A, P), 4 Linha Ofensiva/Bloqueio (CW, TW, MC, MC), 3 Atacantes (C-O, C, W-O).
- **Finalidade:** Proteção densa da Second Zone, contenção de Corridors adversários e transição por passes longos para o C-O.

### 4.3 Formação 3-4-3-4 (Controle de Meio-Campo e Alta Rotatividade)
- **Estrutura:** 3 Defensores (CB, DE, DE), 4 Back Line (A, P, P-R, L), 3 Linha Ofensiva (CW, MC, MC), 4 Conectores/Atacantes (C-O, W-O, C, W-E).
- **Finalidade:** Domínio absoluto da circulação de bola entre as linhas de 40 e 60 Mirins e múltiplos caminhos para Drives válidos do Artrine.

---

## 5. Implementação no Domínio

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PositionLine {
    OffensiveLine,
    BackLine,
    DefenseLine,
    Goalguard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Position {
    CenterOffense,
    WingOffense,
    Midcenter,
    TightWing,
    CenterTight,
    Corridor,
    Artrine,
    Passer,
    PassRusher,
    WideEnd,
    RunningEnd,
    Lineback,
    Fullback,
    Centerback,
    DefensiveEnd,
    Rougieback,
    DefensiveBlocker,
    WideBlocker,
    OutsideZonerback,
    MiddleZonerback,
    Goalguard,
}

impl Position {
    pub fn line(&self) -> PositionLine {
        match self {
            Position::CenterOffense
            | Position::WingOffense
            | Position::Midcenter
            | Position::TightWing
            | Position::CenterTight
            | Position::Corridor => PositionLine::OffensiveLine,
            Position::Artrine
            | Position::Passer
            | Position::PassRusher
            | Position::WideEnd
            | Position::RunningEnd
            | Position::Lineback
            | Position::Fullback => PositionLine::BackLine,
            Position::Centerback
            | Position::DefensiveEnd
            | Position::Rougieback
            | Position::DefensiveBlocker
            | Position::WideBlocker
            | Position::OutsideZonerback
            | Position::MiddleZonerback => PositionLine::DefenseLine,
            Position::Goalguard => PositionLine::Goalguard,
        }
    }

    pub fn is_mandatory(&self) -> bool {
        matches!(
            self,
            Position::Goalguard | Position::Passer | Position::Artrine
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FormationSlot {
    position: Position,
    pitch_length_ratio: f64,
    pitch_width_ratio: f64,
}

impl FormationSlot {
    pub fn new(
        position: Position,
        pitch_length_ratio: f64,
        pitch_width_ratio: f64,
    ) -> Result<Self, arlo_domain::DomainError> {
        if !pitch_length_ratio.is_finite() || !(0.0..=1.0).contains(&pitch_length_ratio) {
            return Err(arlo_domain::DomainError::InvalidInvariant {
                field: "pitch_length_ratio".to_string(),
                reason: "must be between 0.0 and 1.0".to_string(),
            });
        }
        if !pitch_width_ratio.is_finite() || !(0.0..=1.0).contains(&pitch_width_ratio) {
            return Err(arlo_domain::DomainError::InvalidInvariant {
                field: "pitch_width_ratio".to_string(),
                reason: "must be between 0.0 and 1.0".to_string(),
            });
        }

        Ok(Self {
            position,
            pitch_length_ratio,
            pitch_width_ratio,
        })
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn pitch_length_ratio(&self) -> f64 {
        self.pitch_length_ratio
    }

    pub fn pitch_width_ratio(&self) -> f64 {
        self.pitch_width_ratio
    }
}