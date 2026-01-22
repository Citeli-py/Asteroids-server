export class Asteroid {
  constructor(id, x = 0, y = 0, radius = 5) {
    this.id = id;
    this.x = x;
    this.y = y;
    this.radius = radius;

    this.color = "#aaa"; // cinza padrão
  }

  /**
   * Atualiza estado baseado no backend
   */
  syncFromServer(data) {
    this.x = data.x;
    this.y = data.y;
    this.radius = data.radius;
  }

  draw(ctx) {
    const sides = 5; // quanto menor, mais poligonal
    const step = (Math.PI * 2) / sides;

    ctx.strokeStyle = this.color;
    ctx.lineWidth = 2;
    ctx.beginPath();

    for (let i = 0; i <= sides; i++) {
      const angle = i * step;
      const x = this.x + Math.cos(angle) * this.radius;
      const y = this.y + Math.sin(angle) * this.radius;

      if (i === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    }

    ctx.closePath();
    ctx.stroke();
  }


}
