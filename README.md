# 🚀 Jogo de Naves Multiplayer (Servidor em Rust)

Este é um servidor multiplayer de um jogo de naves estilo *Agar.io*, onde jogadores controlam naves que podem se mover, rotacionar e atirar. O foco principal é desempenho, multiplayer em tempo real e integração com um cliente web.
Link do jogo -> https://citeli-py.github.io/Asteroids-server/frontend/

## 🧩 Funcionalidades

- Suporte a múltiplos jogadores via WebSocket
- Simulação física simples com:
  - Velocidade e aceleração
  - Rotação (ângulo da nave)
  - Fricção
- Buffer de comandos com latência tolerante
- Atualização periódica da posição dos jogadores (tickrate fixo)
- Cliente separado em HTML/JS 

## 🏗️ Estrutura

- `src/`
  - `main.rs`: ponto de entrada do servidor
  - `game.rs`: lógica principal do jogo (comandos, estado do jogo)
  - `types.rs`: tipos auxiliares (ex: `ClientId`, `TICK_RATE`)
  - `player.rs`: Lógica dos jogadores
- `frontend`
  - `index.html`: Canvas principal
  - `main.js`: Captura de inputs e lógica principal
  - `network.js`: Conexão com websockets
  - `player.js`: Lógica do desenho dos jogadores
- 
- Comunicação via WebSocket (com `tokio` e `tokio_tungstenite`)
- Gerenciamento de estado com `Arc<Mutex<_>>`

## 📦 Tecnologias

- Linguagem: [Rust](https://www.rust-lang.org/)
- Concorrência assíncrona: `tokio`
- WebSocket: `tokio-tungstenite`

## 📚 Como Rodar

### Pré-requisitos

- Rust instalado (use [rustup.rs](https://rustup.rs))
- Cargo (vem com o Rust)
- Node.js (caso queira testar com o cliente web)

### 1. Clone o projeto

```bash
git clone https://github.com/seu-usuario/nome-do-repositorio
cd nome-do-repositorio
```
2. Compile e execute o servidor

```bash
cargo run
```
O servidor WebSocket iniciará na porta 8080 por padrão.

4. Teste com um cliente
Você pode usar um cliente web com WebSocket que envie comandos como:

```json
"UP"
"LEFT|UP"
"UP|RIGHT"
"SHOT|UP"
```
A resposta do servidor será o estado atual do jogo, por exemplo:

```json
{
  "Players": [
    {
      "id": "d24eae9f-9367-4717-b1a8-42388fe1b63d",
      "x": 249.04756,
      "y": 126.10016,
      "angle": 0.875,
      "is_destroyed": false
    }
  ],
  "Bullets": [
    {
      "id": "a98cf687-e6aa-4c61-a806-dd58e39f08a4",
      "player_id": "d24eae9f-9367-4717-b1a8-42388fe1b63d",
      "x": 539.82574,
      "y": 608.3442
    },
    {
      "id": "094507ca-8111-4929-bfbb-fd4566cead60",
      "player_id": "d24eae9f-9367-4717-b1a8-42388fe1b63d",
      "x": 489.82257,
      "y": 523.1333
    }
  ]
}
```

🔧 Comandos aceitos
- UP: acelera a nave na direção atual
- LEFT: rotaciona para a esquerda
- RIGHT: rotaciona para a direita
- SHOT: Atira um projetil

📏 Taxa de atualização (Tick Rate)
O servidor atualiza o estado do jogo 60 vezes por segundo (TICK_RATE = 32), de forma síncrona para todos os jogadores conectados.

⚠️ Avisos

- Este projeto é voltado para estudo, com foco em jogos multiplayer simples em tempo real usando Rust.

## TODOs

- [X] Coleções de jogadores para diminuir o acoplamento em game
- [X] Camera para acompanhar o jogador
- [X] Mapa bem definido
- [X] Deploy do servidor e frontend
- [X] Asteroides
- [X] Migrar para o Axum
- [X] Health Check
- [ ] Logs
- [ ] Organização do projeto em pastas
- [ ] Entity Component System Design
- [ ] Sprites
- [ ] Mensagens no protocolo para trocar informações como: tamanho do mundo, sessão, ping e etc
- [ ] Tokens de sessão para jogadores
- [ ] QuadTree para lidar com colisões de forma mais eficiente
- [ ] Bots
- [ ] Seleção de servidor



📜 Licença
MIT © Matheus Citeli

Feito com 💻 em Rust para explorar jogos multiplayer de forma simples e performática.

---
