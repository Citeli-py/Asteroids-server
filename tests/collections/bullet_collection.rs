use asteroids_server::collections::bullet_collection::BulletCollection;


#[test]
fn test_dont_max_bullet_overflow_when_adding_multiple_bullets() {
    let mut collection = BulletCollection::new();
    let player_id = uuid::Uuid::new_v4();

    let make_bullets = |n: usize| -> Vec<_> {
        (0..n)
            .map(|_| asteroids_server::entities::bullet::Bullet::new(player_id, 0.0, 0.0, 1.0, 0.0))
            .collect()
    };

    // enche até o limite
    collection.add_bullets(make_bullets(255));
    assert_eq!(collection.get_bullets().len(), 255);

    // tenta adicionar mais — não deve ultrapassar 255
    collection.add_bullets(make_bullets(10));
    assert_eq!(collection.get_bullets().len(), 255);
}

#[test]
fn test_add_bullets_partially_when_near_limit() {
    let mut collection = BulletCollection::new();
    let player_id = uuid::Uuid::new_v4();

    let make_bullets = |n: usize| -> Vec<_> {
        (0..n)
            .map(|_| asteroids_server::entities::bullet::Bullet::new(player_id, 0.0, 0.0, 1.0, 0.0))
            .collect()
    };

    // adiciona 250, depois tenta adicionar 10 — só 5 cabem
    collection.add_bullets(make_bullets(250));
    collection.add_bullets(make_bullets(10));
    assert_eq!(collection.get_bullets().len(), 255);
}