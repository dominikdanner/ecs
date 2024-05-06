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

    let player = world.spawn(Transform { x: 1.0, y: 199.0 });
    world.extend(&player, Health(100.));

    let mut entry = world.entry(&player);

    if let Some(transform) = entry.get_component::<Transform>() {
        dbg!(transform);
    }
}
