import { Player } from "./player.js";
import { Network } from "./network.js";
import { Bullet } from "./bullet.js";

const canvas = document.getElementById("game");
const ctx = canvas.getContext("2d");

const WORLD_SIZE = 2000;

let input = { left: false, right: false, forward: false, fire: false };
let localPlayer = null;
let localPlayerId = null;
let latestGameState = { Players: [], Bullets: [] };

const network = new Network();

localPlayerId = network.get_client_id();
localPlayer = null

// -----------------------------
// 🎮 Entrada do jogador
// -----------------------------
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

// -----------------------------
// 🧭 Câmera e renderização
// -----------------------------

function drawBackground(ctx, cameraX, cameraY) {
  const gridSize = 50;
  const startX = Math.floor(cameraX / gridSize) * gridSize;
  const startY = Math.floor(cameraY / gridSize) * gridSize;

  ctx.strokeStyle = "#222";
  ctx.lineWidth = 1;

  // desenha o grid relativo à câmera
  for (let x = startX - gridSize; x < cameraX + canvas.width + gridSize; x += gridSize) {
    ctx.beginPath();
    ctx.moveTo(x, cameraY - gridSize);
    ctx.lineTo(x, cameraY + canvas.height + gridSize);
    ctx.stroke();
  }
  for (let y = startY - gridSize; y < cameraY + canvas.height + gridSize; y += gridSize) {
    ctx.beginPath();
    ctx.moveTo(cameraX - gridSize, y);
    ctx.lineTo(cameraX + canvas.width + gridSize, y);
    ctx.stroke();
  }
}

function isVisible(obj, cameraX, cameraY, width, height) {
  const FOV = 100;
  return (
    obj.x > cameraX - FOV &&
    obj.x < cameraX + width + FOV &&
    obj.y > cameraY - FOV &&
    obj.y < cameraY + height + FOV
  );
}

function warpPosition(px, py, ox, oy) {

    // dx, dy: deslocamento do jogador -> objeto no espaço normal
  let dx = ox - px;
  let dy = oy - py;

  // se passar da metade do mundo, atravessa a borda (pega o caminho mais curto)
  if (dx > WORLD_SIZE / 2) dx -= WORLD_SIZE;
  if (dx < -WORLD_SIZE / 2) dx += WORLD_SIZE;

  if (dy > WORLD_SIZE / 2) dy -= WORLD_SIZE;
  if (dy < -WORLD_SIZE / 2) dy += WORLD_SIZE;

  console.log("Delta:", dx, dy)

  return { x: px + dx, y: py + dy };
}

function drawWorld(ctx, player, players, bullets) {
  const cameraX = player.x - canvas.width / 2;
  const cameraY = player.y - canvas.height / 2;

  // console.log(player.x, player.y)
  // console.log(cameraX, cameraY)

  ctx.save();
  ctx.translate(-cameraX, -cameraY);

  // Fundo (grid)
  drawBackground(ctx, cameraX, cameraY);

  // Players
  if (Array.isArray(players)) {
    players.forEach((p) => {
      const warpped = warpPosition(player.x, player.y, p.x, p.y);
      if (isVisible(warpped, cameraX, cameraY, canvas.width, canvas.height)) {
        new Player(
          p.id, 
          warpped.x, 
          warpped.y, 
          p.angle
        ).draw(ctx, p.id === player.id);
      }
    });
  }

  // Bullets
  if (Array.isArray(bullets)) {
    bullets.forEach((b) => {
      const warpped = warpPosition(player.x, player.y, b.x, b.y);
      if (isVisible(warpped, cameraX, cameraY, canvas.width, canvas.height)) {
        console.log("BULLET", b.x, b.y);
        console.log(warpped.x, warpped.y);
        new Bullet(
          b.id, 
          warpped.x, 
          warpped.y, 
          b.angle, 
          b.player_id
        ).draw(ctx, b.player_id === player.id);
      }
    });
  }

  ctx.restore();
}

// -----------------------------
// 🕹️ Loop principal do jogo
// -----------------------------
function gameLoop() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);

  latestGameState = network.get_game_state();

  //localPlayer.update(input);
  network.sendMove(input);

  const players = latestGameState["Players"] || [];
  const bullets = latestGameState["Bullets"] || [];

  let player = players.find((p) => p.id === localPlayerId);

  drawWorld(ctx, player, players, bullets);

  requestAnimationFrame(gameLoop);
}

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function connect() {
  while (localPlayerId === null) {
    console.log("trying to connect")
    localPlayerId = network.get_client_id()
    await sleep(500);
  }
}

await connect();
gameLoop();
