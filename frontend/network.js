export class Network {
  constructor() {
    this.gameState = {};
    this.clientId = null;

    this.socket = new WebSocket("ws://localhost:8080");

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

  get_client_id(){
    return this.clientId;
  }

  sendPosition(x, y, angle) {
    if (this.socket.readyState === WebSocket.OPEN && this.clientId) {
      const msg = `pos:${x},${y},${angle}`
      this.socket.send(msg);
    }
  }

  sendMove(move) {
    if (!(this.socket.readyState === WebSocket.OPEN && this.clientId)) {
      return; 
    }

    if (move.left)
      this.socket.send("LEFT");

    if (move.right)
      this.socket.send("RIGHT");

    if (move.forward)
      this.socket.send("UP");
  }
}
