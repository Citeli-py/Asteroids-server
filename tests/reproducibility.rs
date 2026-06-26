use asteroids_server::game::GameManager;

// Extrai as posições dos asteroides como bits (comparação exata de f32),
// ordenadas — independe da ordem de iteração do HashMap.
fn asteroid_positions(game: &GameManager) -> Vec<(u32, u32)> {
    let mut pos: Vec<(u32, u32)> = game
        .asteroids
        .get_hitboxes()
        .iter()
        .map(|h| (h.pos.x.to_bits(), h.pos.y.to_bits()))
        .collect();
    pos.sort();
    pos
}

#[test]
fn same_seed_gives_same_world() {
    let a = GameManager::with_seed(42);
    let b = GameManager::with_seed(42);

    assert_eq!(asteroid_positions(&a), asteroid_positions(&b));
}

#[test]
fn different_seed_gives_different_world() {
    let a = GameManager::with_seed(1);
    let b = GameManager::with_seed(2);

    assert_ne!(asteroid_positions(&a), asteroid_positions(&b));
}
