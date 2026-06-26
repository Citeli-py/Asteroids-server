import { Network } from "./src/networking/network.js";
import { drawWorld } from "./src/ui/renderer.js";
import { drawHUD } from "./src/ui/hud.js";
import { showScreen } from "./src/ui/screens.js";

const canvas = document.getElementById("game");
const ctx = canvas.getContext("2d");

let input = { left: false, right: false, forward: false, fire: false };
let localPlayerId = null;
let latestGameState = { Players: [], Bullets: [], Asteroids: [] };
let pingIntervalId = null;
let playerWasSeen = false;

const network = new Network();

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
    evt.preventDefault();
  }
}

document.addEventListener("keydown", (e) => handleInput(e, true));
document.addEventListener("keyup", (e) => handleInput(e, false));

// -----------------------------
// 🕹️ Loop principal do jogo
// -----------------------------
function gameLoop() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);

  latestGameState = network.get_game_state();
  network.sendMove(input);

  const players = latestGameState["Players"] || [];
  const bullets = latestGameState["Bullets"] || [];
  const asteroids = latestGameState["Asteroids"] || [];

  const player = players.find((p) => p.id === localPlayerId);

  if (player) {
    playerWasSeen = true;
    drawWorld(ctx, canvas, player, players, bullets, asteroids);
  } else if (playerWasSeen) {
    clearInterval(pingIntervalId);
    pingIntervalId = null;
    playerWasSeen = false;
    showScreen("dead");
    return;
  }

  drawHUD(ctx, canvas, players, asteroids, localPlayerId, network.lastPing);

  requestAnimationFrame(gameLoop);
}

// -----------------------------
// 🔌 Inicialização
// -----------------------------
function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function waitFor(getter) {
  while (getter() === null) await sleep(200);
}

async function pingLoop() {
  try {
    await network.ping();
  } catch (e) {
    console.error("Não foi possível medir o ping:", e);
  }
}

async function startGame() {
  localPlayerId = null;
  playerWasSeen = false;
  latestGameState = { Players: [], Bullets: [], Asteroids: [] };
  showScreen("connecting");

  network.openSocket();
  await waitFor(() => network.get_client_id());
  localPlayerId = network.get_client_id();

  showScreen(null);
  pingIntervalId = setInterval(pingLoop, 5000);
  await pingLoop();
  gameLoop();
}

document.getElementById("btn-play").addEventListener("click", startGame);
document.getElementById("btn-restart").addEventListener("click", startGame);
