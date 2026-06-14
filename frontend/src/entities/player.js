export class Player {
  constructor(id, x = 0, y = 0, angle = 0) {
    this.id = id;
    this.x = x;
    this.y = y;
    this.angle = angle;
    this.vx = 0;
    this.vy = 0;
    this.turnSpeed = 0.05;
    this.acceleration = 0.2;
    this.friction = 0.98;
  }

  draw(ctx, isSelf = false) {
    ctx.fillStyle = isSelf ? "lime" : "red";

    const size = 10;
    ctx.save();
    ctx.translate(this.x, this.y);
    ctx.rotate(this.angle + Math.PI / 2);

    ctx.beginPath();
    ctx.moveTo(0, -size);
    ctx.lineTo(-size, size);
    ctx.lineTo(size, size);
    ctx.closePath();
    ctx.fill();

    ctx.restore();
  }
}
