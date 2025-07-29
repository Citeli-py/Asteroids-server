import { Player } from "./player.js";
import { Network } from "./network.js";

const canvas = document.getElementById("game");
const ctx = canvas.getContext("2d");

let input = { left: false, right: false, forward: false };
let localPlayer = null;
let localPlayerId = null;
let latestGameState = { Players: [] };

const network = new Network();

localPlayerId = network.get_client_id();
localPlayer = new Player(localPlayerId, 100, 100);

function handleInput(evt, isDown) {
  const key = evt.key.toLowerCase();

  if (key === "a") input.left = isDown;
  if (key === "d") input.right = isDown;
  if (key === "w") input.forward = isDown;
}

document.addEventListener("keydown", (e) => handleInput(e, true));
document.addEventListener("keyup", (e) => handleInput(e, false));

function gameLoop() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  
  latestGameState = network.get_game_state();

  localPlayer.update(input);
  network.sendPosition(localPlayer.x, localPlayer.y, localPlayer.angle);
  //localPlayer.draw(ctx, true); 

  let players = latestGameState['Players'];

  if (Array.isArray(players)) {
    players.forEach((player) => {
      new Player(player.id, player.x, player.y, player.angle).draw(ctx, player.id === network.get_client_id());
    });
  }


  requestAnimationFrame(gameLoop);
}

gameLoop();
