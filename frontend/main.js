import { Player } from "./player.js";
import { Network } from "./network.js";
import { Bullet } from "./bullet.js";

const canvas = document.getElementById("game");
const ctx = canvas.getContext("2d");

let input = { left: false, right: false, forward: false, fire: false };
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

  if (key === " ") { 
    input.fire = isDown;
    evt.preventDefault(); // evita scroll da página
  }
}


document.addEventListener("keydown", (e) => handleInput(e, true));
document.addEventListener("keyup", (e) => handleInput(e, false));

function gameLoop() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);

  latestGameState = network.get_game_state();

  localPlayer.update(input);
  network.sendMove(input);

  let players = latestGameState["Players"];
  let bullets = latestGameState["Bullets"];

  if (Array.isArray(players)) {
    players.forEach((player) => {
      new Player(player.id, player.x, player.y, player.angle)
        .draw(ctx, player.id === network.get_client_id());
    });
  }

  if (Array.isArray(bullets)) {
    bullets.forEach((bullet) => {
      new Bullet(bullet.id, bullet.x, bullet.y, bullet.angle, bullet.player_id)
        .draw(ctx, bullet.player_id === network.get_client_id());
    });
  }

  requestAnimationFrame(gameLoop);
}

gameLoop();
