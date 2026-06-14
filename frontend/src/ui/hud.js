import { WORLD_SIZE, MINIMAP_SIZE, MINIMAP_PADDING, VIEW_RADIUS } from "../constants.js";

export function drawHUD(ctx, canvas, players, asteroids, localPlayerId, lastPing) {
  const self = players.find((p) => p.id === localPlayerId);

  const mx = canvas.width - MINIMAP_SIZE - MINIMAP_PADDING;
  const my = MINIMAP_PADDING;
  const center = MINIMAP_SIZE / 2;
  const scale = center / VIEW_RADIUS;

  ctx.save();
  ctx.beginPath();
  ctx.rect(mx, my, MINIMAP_SIZE, MINIMAP_SIZE);
  ctx.clip();

  ctx.fillStyle = "rgba(0, 0, 0, 0.65)";
  ctx.fillRect(mx, my, MINIMAP_SIZE, MINIMAP_SIZE);

  if (self) {
    const toMinimap = (wx, wy) => {
      let dx = wx - self.x;
      let dy = wy - self.y;
      if (dx > WORLD_SIZE / 2) dx -= WORLD_SIZE;
      if (dx < -WORLD_SIZE / 2) dx += WORLD_SIZE;
      if (dy > WORLD_SIZE / 2) dy -= WORLD_SIZE;
      if (dy < -WORLD_SIZE / 2) dy += WORLD_SIZE;
      return { x: mx + center + dx * scale, y: my + center + dy * scale };
    };

    ctx.fillStyle = "#555";
    asteroids.forEach((a) => {
      const p = toMinimap(a.x, a.y);
      ctx.beginPath();
      ctx.arc(p.x, p.y, 2, 0, Math.PI * 2);
      ctx.fill();
    });

    ctx.fillStyle = "#f44";
    players.forEach((p) => {
      if (p.id === localPlayerId) return;
      const mp = toMinimap(p.x, p.y);
      ctx.beginPath();
      ctx.arc(mp.x, mp.y, 3, 0, Math.PI * 2);
      ctx.fill();
    });

    ctx.fillStyle = "#0f0";
    ctx.beginPath();
    ctx.arc(mx + center, my + center, 4, 0, Math.PI * 2);
    ctx.fill();
  }

  ctx.restore();

  ctx.strokeStyle = "#666";
  ctx.lineWidth = 1;
  ctx.strokeRect(mx, my, MINIMAP_SIZE, MINIMAP_SIZE);

  if (lastPing !== null) {
    ctx.font = "16px 'Courier New'";
    ctx.fillStyle = "#0f0";
    ctx.textAlign = "left";
    ctx.fillText(`Ping: ${lastPing} ms`, 16, 28);
  }
}
