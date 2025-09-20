export class Bullet {
    constructor(id, x, y, angle, playerId) {
        this.id = id;
        this.x = x;
        this.y = y;
        this.angle = angle;
        this.playerId = playerId;
    }

    draw(ctx, isSelf = false) {
        ctx.fillStyle = isSelf ? "lime" : "red"; // verde se for do jogador local

        const size = 5; // tamanho pequeno para bala
        ctx.save();

        ctx.translate(this.x, this.y);
        ctx.rotate(this.angle + Math.PI / 2);

        ctx.beginPath();
        ctx.arc(0, 0, size, 0, 2 * Math.PI); // círculo
        ctx.fill();

        ctx.restore();
    }
}
