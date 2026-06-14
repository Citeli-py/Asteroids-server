export class Network {
  constructor() {
    this.gameState = {};
    this.clientId = null;
    this.lastPing = null;
    this.socket = null;

    //this.url = "localhost:8080";
    this.url = "asteroids-server-ampj.onrender.com";
  }

  connect() {
    if (this.socket) {
      this.socket.onmessage = null;
      this.socket.onopen = null;
      this.socket.onerror = null;
      this.socket.close();
    }
    this.clientId = null;
    this.gameState = {};
    this.socket = new WebSocket(`wss://${this.url}/ws`);

    this.socket.onmessage = (event) => {
      const data = event.data;

      if (data.startsWith("connected:")) {
        this.clientId = data.split(":")[1];
        console.log("Connected as", this.clientId);
        return;
      }

      try {
        const state = JSON.parse(data);
        this.gameState = state;
      } catch (e) {
        console.error("Erro ao parsear estado do jogo:", e, data);
      }
    };

    this.socket.onopen = () => {
      console.log("Conectado ao servidor");
    };

    this.socket.onerror = (err) => {
      console.error("Erro no WebSocket:", err);
    };
  }

  get_game_state() {
    return this.gameState;
  }

  get_client_id() {
    return this.clientId;
  }

  async ping() {
    const t0 = Date.now();
    const response = await fetch(`https://${this.url}/health`, { method: "GET" });
    if (!response.ok) {
      throw new Error("Não foi possível pingar o servidor");
    }
    this.lastPing = Date.now() - t0;
    return this.lastPing;
  }

  sendPosition(x, y, angle) {
    if (this.socket && this.socket.readyState === WebSocket.OPEN && this.clientId) {
      this.socket.send(`pos:${x},${y},${angle}`);
    }
  }

  sendMove(move) {
    if (!(this.socket && this.socket.readyState === WebSocket.OPEN && this.clientId)) {
      return;
    }
    let message = "";

    if (move.left) message += "LEFT|";
    if (move.right) message += "RIGHT|";
    if (move.forward) message += "UP|";
    if (move.fire) message += "SHOT|";

    if (message !== "") {
      this.socket.send(message);
    }
  }
}
