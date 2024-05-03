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

fn main() {
    let mut world = World::new();

    let player = world.spawn(Health(100.00));
    let transform = world.spawn(Transform { x: 1.0, y: 199.0 });
    let stamina = world.spawn(Stamina(300.00));

    if let Some(component) = world.get_component::<Transform>(&transform) {
        println!("Transform: x={}, y={}", component.x, component.y);
    }
}
