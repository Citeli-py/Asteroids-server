use asteroids_server::collections::bullet_collection::{BulletCollection, MAX_BULLETS};

fn make_bullets(n: usize) -> Vec<asteroids_server::entities::bullet::Bullet> {
    let player_id = uuid::Uuid::new_v4();
    (0..n)
        .map(|_| asteroids_server::entities::bullet::Bullet::new(player_id, 0.0, 0.0, 1.0, 0.0))
        .collect()
}

#[test]
fn test_dont_max_bullet_overflow_when_adding_multiple_bullets() {
    let mut collection = BulletCollection::new();

    // enche até o limite
    collection.add_bullets(make_bullets(MAX_BULLETS));
    assert_eq!(collection.get_bullets().len(), MAX_BULLETS);

    // tenta adicionar mais — não deve ultrapassar o limite
    collection.add_bullets(make_bullets(10));
    assert_eq!(collection.get_bullets().len(), MAX_BULLETS);
}

#[test]
fn test_add_bullets_partially_when_near_limit() {
    let mut collection = BulletCollection::new();

    // faltam 5 para o limite, depois tenta adicionar 10 — só 5 cabem
    collection.add_bullets(make_bullets(MAX_BULLETS - 5));
    collection.add_bullets(make_bullets(10));
    assert_eq!(collection.get_bullets().len(), MAX_BULLETS);
}
