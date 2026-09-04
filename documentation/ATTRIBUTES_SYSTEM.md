# Sistema de Atributos e Resolução Mecânica do Arlo

## 1. Fundamentação e Arquitetura do Sistema

O sistema de atributos da Engine de simulação do Arlo modela a capacidade técnica, psicológica, física e posicional de cada atleta em campo. O modelo adota uma escala cardinal discreta de **1 a 20** para jogadores e treinadores, em que cada incremento numérico representa um desvio significativo de rendimento estatístico.

```
[1 .. 4]   -> Amador / Severamente Deficiente
[5 .. 8]   -> Abaixo da Média Profissional / Base
[9 .. 12]  -> Médio / Padrão Competitivo
[13 .. 15] -> Bom / Alto Nível Nacional
[16 .. 18] -> Excelente / Nível Continental
[19 .. 20] -> Extraordinário / Elite Histórica
```

### 1.1 Separação Filosófica: Operários vs. Craques
Diferente de sistemas puramente utilitários de esportes de colisão, o Arlo é um esporte de alta plasticidade técnica e refinamento tático. A mecânica exige a separação clara entre:
- **Atletas Operários:** Jogadores com atributos técnicos específicos elevados para sua função direta (ex.: Bloqueio Ofensivo 18, Força 17, Trabalho em Equipe 16), mas baixa sofisticação global na execução.
- **Grandes Craques:** Jogadores dotados de alta **Técnica**, **Criatividade (Flair)** e **Visão de Jogo**, capazes de converter situações desfavoráveis em jogadas eficientes com trajetórias limpas de passe, recepção perfeita sob pressão e fintas milimétricas.

---

## 2. Catálogo Canônico de Atributos

```
                          ┌───────────────────────────┐
                          │   ATRIBUTOS DO JOGADOR    │
                          └─────────────┬─────────────┘
          ┌──────────────────┬──────────┴──────────┬──────────────────┐
          │                  │                     │                  │
┌─────────┴────────┐ ┌───────┴────────┐ ┌──────────┴────────┐ ┌───────┴────────┐
│     TÉCNICOS     │ │    MENTAIS     │ │      FÍSICOS      │ │   GOALGUARD    │
│  (12 Atributos)  │ │ (13 Atributos) │ │   (8 Atributos)   │ │ (10 Atributos) │
└──────────────────┘ └────────────────┘ └───────────────────┘ └────────────────┘
```

### 2.1 Atributos Técnicos (Jogadores de Linha)

| Atributo | Chave de Domínio | Descrição Mecânica |
| :--- | :--- | :--- |
| **Controle do Arlo** | `arlo_control` | Capacidade de amortecer, estabilizar e fixar a posse do arlo com qualquer membro (mãos, pés, tronco) dentro do limiar de 0,7s. |
| **Passe / Lançamento** | `passing_launching` | Precisão vetorial e velocidade da bola em passes curtos, médios e lançamentos em profundidade com as mãos ou pés. |
| **Drible / Condução** | `dribbling_carrying` | Habilidade de avançar com o arlo sob controle direcional direto, protegendo-o em progressão contra defensores. |
| **Finalização / Chute** | `shooting_finishing` | Eficiência e precisão na execução de chutes ao Goalpost (baliza retangular) e Fieldpost (trave em H). |
| **Cruzamento / Lateral**| `crossing_lateral` | Precisão em lançamentos em arco das zonas periféricas para o miolo da área ou Second Zone. |
| **Bloqueio Ofensivo** | `offensive_blocking`| Técnica para retardar, selar ou desviar defensores usando o corpo, seguradas e trancos permitidos. |
| **Contenção Defensiva** | `defensive_containment`| Capacidade de interromper avanços adversários com rasteiras controladas, trancos no tronco e envelopamento legal sem tackle violento. |
| **Pressão / Rush** | `pressure_rush` | Técnica de transposição de bloqueios e invasão da linha de escarmouche para acelerar ou abortar o passe inicial do Passer. |
| **Recepção / Hands** | `catching_hands` | Firmeza das mãos na apreensão aérea ou em trajetória rápida de passes e lançamentos com disputa física. |
| **Técnica de Drive** | `drive_technique` | Precisão e tempo de passada para cruzar as bordas de 35 cm de um Artro mantendo a posse válida (exclusivo para Artrine verdadeiro). |
| **Blefe / Disfarce** | `bluff_disguise` | Capacidade cênica e mecânica de reproduzir rotas, gestos e comandos do Artrine legítimo para atrair a marcação. |
| **Técnica** | `technique` | Pureza do gesto motor, facilidade para movimentos difíceis, efeitos na trajetória do arlo e controle estético sob pressão extrema. |

### 2.2 Atributos Mentais (Linha e Goalguard)

| Atributo | Chave de Domínio | Descrição Mecânica |
| :--- | :--- | :--- |
| **Antecipação** | `anticipation` | Capacidade de prever o desenvolvimento da jogada antes do toque no arlo ou da quebra da linha. |
| **Decisões** | `decisions` | Avaliação estocástica correta da melhor ação (passar, avançar, reter, Drive, chutar) em tempo real. |
| **Compostura / Frieza**| `composure` | Redutor do impacto de estresse psicológico e proximidade de marcadores na precisão motora do jogador. |
| **Concentração** | `concentration` | Sustentação do foco tático e cognitivo ao longo dos 120 minutos regulamentares e prorrogações. |
| **Visão de Jogo** | `vision` | Alcance e raio de leitura de opções de passe, identificação de Artros livres e detecção de sobrecargas numéricas. |
| **Posicionamento** | `positioning` | Manutenção da posição base e ocupação geométrica inteligente do espaço em transições e fases defensivas. |
| **Trabalho em Equipe** | `teamwork` | Aderência às instruções do plano tático, sacrifício posicional em bloqueios e trocas de cobertura. |
| **Determinação** | `determination` | Resiliência mental para reverter desvantagens no placar ou converter séries críticas de 4 Call-to-Actions. |
| **Liderança** | `leadership` | Capacidade de elevar a compostura e o rendimento dos companheiros de equipe em campo. |
| **Agressividade** | `controlled_aggression`| Nível de combatividade física e ímpeto em disputas divididas sem ultrapassar o limite legal de faltas. |
| **Bravura** | `bravery` | Disposição para disputar bolas divididas, choques corporais e proteger o arlo sob impacto iminente. |
| **Criatividade / Flair**| `creativity_flair` | Propensão e habilidade para criar trajetórias não óbvias, fintas inesperadas e soluções fora da rota padrão. |
| **Taxa de Trabalho** | `work_rate` | Volume e intensidade de deslocamento ativo sem a bola nas amplas dimensões do campo (140–150m x 80–90m). |

### 2.3 Atributos Físicos (Linha e Goalguard)

| Atributo | Chave de Domínio | Descrição Mecânica |
| :--- | :--- | :--- |
| **Aceleração** | `acceleration` | Taxa temporal de variação de velocidade até atingir o vetor de velocidade máxima ($m/s^2$). |
| **Velocidade (Pace)** | `pace` | Velocidade máxima linear em linha reta atingível pelo jogador ($m/s$). |
| **Agilidade** | `agility` | Capacidade de desacelerar, mudar de direção angular e readquirir vetor cinemático sob controle. |
| **Equilíbrio** | `balance` | Resistência física contra a perda de centro de gravidade após trancos, seguradas e rasteiras controladas. |
| **Força** | `strength` | Força estática e dinâmica aplicada em disputas corpo a corpo, engajamento de bloqueios e proteção do arlo. |
| **Resistência** | `stamina` | Taxa de decaimento de energia física durante as Actions sucessivas e preservação da fadiga acumulada. |
| **Impulsão** | `aerial_reach` | Altura máxima atingida no salto vertical medida a partir da extensão vertical do tronco e membros. |
| **Forma Natural** | `natural_fitness` | Capacidade intrínseca de recuperação pós-partida, mitigação de desgaste crônico e manutenção de atributos na carreira (~40 anos). |

### 2.4 Atributos Específicos de Goalguard

| Atributo | Chave de Domínio | Descrição Mecânica |
| :--- | :--- | :--- |
| **Reflexos** | `gk_reflexes` | Tempo de reação motora instantânea a arremessos e chutes à queima-roupa no Goalpost. |
| **Handling / Mãos** | `gk_handling` | Eficiência em reter o arlo sem conceder rebote ou desviar a bola para zonas neutras fora de perigo. |
| **Comando de Área** | `gk_area_command` | Autoridade de movimentação e interceptação dentro dos 7 Mirins da First Zone. |
| **Comunicação** | `gk_communication` | Organização posicional verbal e sinérgica sobre a linha de Centerbacks e Zonadores defensivos. |
| **Alcance Aéreo GK** | `gk_aerial_reach` | Ponto mais alto em que o goleiro consegue segurar ou socar o arlo utilizando o privilégio das mãos na First Zone. |
| **Um contra Um** | `gk_one_on_one` | Fechamento de ângulos de finalização e leitura de tempo de investida contra atacantes desmarcados. |
| **Posicionamento GK**| `gk_positioning` | Alinhamento angular e profundidade entre a bissetriz do arrematante, o Goalpost e o Fieldpost. |
| **Distribuição** | `gk_distribution` | Precisão em lançamentos rápidos com as mãos ou passes rasteiros para início imediato da transição pós-defesa. |
| **Chute / Kicking** | `gk_kicking` | Potência e precisão em chutes de longa distância partindo da First Zone para além da linha de meio-campo. |
| **Saída de Gol** | `gk_rushing_out` | Tempo de tomada de decisão e velocidade de saída da meta para cortar bolas lançadas em profundidade. |