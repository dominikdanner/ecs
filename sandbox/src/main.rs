use ecs::{
    storage::{Component, VecStorage},
    world::World,
};

#[derive(Debug)]
struct Health(f32);

impl Component for Health {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Stamina(f32);

impl Component for Stamina {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Transform {
    x: f32,
    y: f32,
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}

// Hint: Order of adding components to entity creates issues with archetypes!!!
fn main() {
    let mut world = World::new();

    let player_1 = world.spawn(Transform { x: 1.0, y: 199.0 });
    world.extend(&player_1, Health(100.));

    let player_2 = world.spawn(Transform { x: 23.0, y: 100.0 });
    world.extend(&player_2, Health(200.));

    for transform in world.query::<Transform>() {
        println!("Transform: x={}, y={}", transform.x, transform.y);
    }
}
