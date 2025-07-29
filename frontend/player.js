export class Player {
  constructor(id, x = 0, y = 0, angle=0) {
    this.id = id;
    this.x = x;
    this.y = y;
    this.angle = angle;
    this.speed = 0;
    this.vx = 0;
    this.vy = 0;
    this.turnSpeed = 0.05;
    this.acceleration = 0.2;
    this.friction = 0.98;
    this.color = 'red';
  }

  update(input) {
    if (input.left)  this.angle -= this.turnSpeed;
    if (input.right) this.angle += this.turnSpeed;

    if (input.forward) {
      this.vx += Math.cos(this.angle) * this.acceleration;
      this.vy += Math.sin(this.angle) * this.acceleration;
    }

    this.vx *= this.friction;
    this.vy *= this.friction;

    this.x += this.vx;
    this.y += this.vy;
  }

  draw(ctx, isSelf = false) {
    
    ctx.fillStyle = "red";
    if (isSelf)
      ctx.fillStyle = "lime";

    const size = 10; 
    ctx.save(); // Salva o estado atual

    // Move a origem para (x, y)
    ctx.translate(this.x, this.y);
    ctx.rotate(this.angle+Math.PI/2); // Aplica a rotação em radianos

    ctx.beginPath();
    ctx.moveTo(0, -size);           // Ponto superior (aponta para cima)
    ctx.lineTo(-size, size);        // Base esquerda
    ctx.lineTo(size, size);         // Base direita
    ctx.closePath();
    ctx.fill();

    ctx.restore(); // Restaura o estado original
  }

}
