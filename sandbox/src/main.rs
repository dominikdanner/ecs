use ecs::{world::World, Component, VecStorage};

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

fn main() {
    let mut world = World::new();

    world.spawn(Health(200.00));

    dbg!(world.archetype);
    dbg!(world.components);
}
