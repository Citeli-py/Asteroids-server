use std::hint::black_box;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use uuid::Uuid;
use asteroids_server::game::GameManager;
use asteroids_server::networking::router::MovePayload;

fn generate_game() -> GameManager {
    let mut game = GameManager::new(); // já spawna max_asteroids (16)

    for _ in 0..255 {
        let id = Uuid::new_v4();
        let _ = game.players.add_player(&id);

        let fire = MovePayload { thrust: true, left: false, right: false, fire: true };
        game.handle_player_command(&id, &fire);
    }

    // processa os comandos de tiro para popular as bullets antes do bench
    game.players.update();

    game
}

fn bench_tick(c: &mut Criterion) {
    c.bench_function("100 ticks - max entities", |b| {
        b.iter_batched(
            generate_game,
            |mut game| {
                for _ in 0..100 {
                    black_box(game.tick());
                }
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, bench_tick);
criterion_main!(benches);