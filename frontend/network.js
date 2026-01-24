export class Network {
  constructor() {
    this.gameState = {};
    this.clientId = null;

    //this.url = "localhost:8080";
    this.url = "asteroids-server-ampj.onrender.com";

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

  get_client_id(){
    return this.clientId;
  }

  /**
   * Verifica o tempo até acessar o servidor
   * @returns {Number} - Ping em ms
   * @throws {Error} - Se nbão for possivel pingar
   */
  async ping() {
    const t0 = Date.now();

    const response = await fetch(`https://${this.url}/health`, {method: "GET"})

    console.log(response);

    return Date.now() - t0;
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
    let message = "";

    if (move.left)
      message += "LEFT|";
      //this.socket.send("LEFT");

    if (move.right)
      message += "RIGHT|";
      //this.socket.send("RIGHT");

    if (move.forward)
      message += "UP|"
      //this.socket.send("UP");

    if (move.fire)
      message += "SHOT|"
    
    if(message !== "") {
      console.log("SEND: ", message);
      this.socket.send(message);
    }
  }
}
