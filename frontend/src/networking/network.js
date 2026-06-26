export class Network {
  constructor() {
    this.gameState = {};
    this.gameInfo = null;
    this.sessionId = null;
    this.clientId = null;
    this.lastPing = null;
    this.socket = null;

    this.url = "localhost:8080";
    //this.url = "asteroids-server-ampj.onrender.com";
  }

  isSocketOpen() {
    return this.socket && this.socket.readyState === WebSocket.OPEN;
  }

  openSocket() {
    if (this.socket) {
      this.socket.onmessage = null;
      this.socket.onopen = null;
      this.socket.onerror = null;
      this.socket.close();
      this.socket = null;
    }

    this.sessionId = null;
    this.clientId = null;
    this.gameState = {};

    const wsProtocol = this.url.startsWith("localhost") ? "ws" : "wss";
    this.socket = new WebSocket(`${wsProtocol}://${this.url}/ws`);

    this.socket.onmessage = (event) => {
      const data = event.data;

      if (data.startsWith("session:")) {
        this.sessionId = data.split(":")[1];
        console.log("Sessão aberta:", this.sessionId);
        return;
      }

      if (data.startsWith("connected:")) {
        this.clientId = data.split(":")[1];
        console.log("Entrou no jogo:", this.clientId);
        return;
      }

      if (data === "disconnected") {
        this.clientId = null;
        return;
      }

      try {
        const msg = JSON.parse(data);

        if (msg.type === "game_info") {
          this.gameInfo = msg;
          console.log("Info do jogo:", msg);
          return;
        }

        this.gameState = msg;
      } catch (e) {
        console.error("Erro ao parsear estado do jogo:", e, data);
      }
    };

    this.socket.onopen = () => console.log("WebSocket conectado");
    this.socket.onerror = (err) => console.error("Erro no WebSocket:", err);
  }

  sendConnect() {
    if (!this.isSocketOpen()) return;
    this.clientId = null;
    this.socket.send(JSON.stringify({ action: "connect" }));
  }

  sendDisconnect() {
    if (!this.isSocketOpen()) return;
    this.socket.send(JSON.stringify({ action: "disconnect" }));
  }

  requestGameInfo() {
    if (!this.isSocketOpen()) return;
    this.socket.send(JSON.stringify({ action: "get_game_info" }));
  }

  get_game_state() {
    return this.gameState;
  }

  get_game_info() {
    return this.gameInfo;
  }

  get_session_id() {
    return this.sessionId;
  }

  get_client_id() {
    return this.clientId;
  }

  async ping() {
    const t0 = Date.now();
    const httpProtocol = this.url.startsWith("localhost") ? "http" : "https";
    const response = await fetch(`${httpProtocol}://${this.url}/health`, { method: "GET" });
    if (!response.ok) throw new Error("Não foi possível pingar o servidor");
    this.lastPing = Date.now() - t0;
    return this.lastPing;
  }

  sendMove(move) {
    if (!this.isSocketOpen() || !this.clientId) return;
    if (!move.left && !move.right && !move.forward && !move.fire) return;

    this.socket.send(JSON.stringify({
      action: "move",
      left: move.left,
      right: move.right,
      thrust: move.forward,
      fire: move.fire,
    }));
  }
}
