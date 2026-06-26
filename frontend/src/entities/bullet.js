export class Bullet {
  constructor(id, x, y, angle, playerId) {
    this.id = id;
    this.x = x;
    this.y = y;
    this.angle = angle;
    this.playerId = playerId;
  }

  draw(ctx, isSelf = false) {
    ctx.fillStyle = isSelf ? "lime" : "red";

    ctx.save();
    ctx.translate(this.x, this.y);

    ctx.beginPath();
    ctx.arc(0, 0, 3, 0, Math.PI * 2);
    ctx.fill();

    ctx.restore();
  }
}
