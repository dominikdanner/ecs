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
    _x: f32,
    _y: f32,
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}

// Hint: Order of adding components to entity creates issues with archetypes!!!
fn main() {
    let mut world = World::new();

    let player = world.spawn(Transform { _x: 1.0, _y: 199.0 });
    let mut player_entry = world.entry_mut(&player);
    player_entry.add_component(Health(200.0));

    if let Some(health) = player_entry.get_component::<Health>() {
        dbg!(health);
    }

    if let Some(transform) = player_entry.get_component::<Transform>() {
        dbg!(transform);
    }
}
