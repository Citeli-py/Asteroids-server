import { WORLD_SIZE } from "../constants.js";
import { Player } from "../entities/player.js";
import { Bullet } from "../entities/bullet.js";
import { Asteroid } from "../entities/asteroid.js";

export function warpPosition(px, py, ox, oy) {
  let dx = ox - px;
  let dy = oy - py;

  if (dx > WORLD_SIZE / 2) dx -= WORLD_SIZE;
  if (dx < -WORLD_SIZE / 2) dx += WORLD_SIZE;
  if (dy > WORLD_SIZE / 2) dy -= WORLD_SIZE;
  if (dy < -WORLD_SIZE / 2) dy += WORLD_SIZE;

  return { x: px + dx, y: py + dy };
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

function drawBackground(ctx, cameraX, cameraY, width, height) {
  const gridSize = 50;
  const startX = Math.floor(cameraX / gridSize) * gridSize;
  const startY = Math.floor(cameraY / gridSize) * gridSize;

  ctx.strokeStyle = "#ffffff53";
  ctx.lineWidth = 1;

  for (let x = startX - gridSize; x < cameraX + width + gridSize; x += gridSize) {
    ctx.beginPath();
    ctx.moveTo(x, cameraY - gridSize);
    ctx.lineTo(x, cameraY + height + gridSize);
    ctx.stroke();
  }
  for (let y = startY - gridSize; y < cameraY + height + gridSize; y += gridSize) {
    ctx.beginPath();
    ctx.moveTo(cameraX - gridSize, y);
    ctx.lineTo(cameraX + width + gridSize, y);
    ctx.stroke();
  }
}

export function drawWorld(ctx, canvas, player, players, bullets, asteroids) {
  const cameraX = player.x - canvas.width / 2;
  const cameraY = player.y - canvas.height / 2;

  ctx.save();
  ctx.translate(-cameraX, -cameraY);

  drawBackground(ctx, cameraX, cameraY, canvas.width, canvas.height);

  if (Array.isArray(players)) {
    players.forEach((p) => {
      const warped = warpPosition(player.x, player.y, p.x, p.y);
      if (isVisible(warped, cameraX, cameraY, canvas.width, canvas.height)) {
        new Player(p.id, warped.x, warped.y, p.angle).draw(ctx, p.id === player.id);
      }
    });
  }

  if (Array.isArray(bullets)) {
    bullets.forEach((b) => {
      const warped = warpPosition(player.x, player.y, b.x, b.y);
      if (isVisible(warped, cameraX, cameraY, canvas.width, canvas.height)) {
        new Bullet(b.id, warped.x, warped.y, b.angle, b.player_id).draw(ctx, b.player_id === player.id);
      }
    });
  }

  if (Array.isArray(asteroids)) {
    asteroids.forEach((a) => {
      const warped = warpPosition(player.x, player.y, a.x, a.y);
      if (isVisible(warped, cameraX, cameraY, canvas.width, canvas.height)) {
        new Asteroid(a.id, warped.x, warped.y, a.radius).draw(ctx);
      }
    });
  }

  ctx.restore();
}
