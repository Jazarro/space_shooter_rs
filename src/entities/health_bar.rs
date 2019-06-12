use amethyst::{
    prelude::Builder,
    ecs::prelude::World,
};

use crate::{
    components::HealthBar,
};


const X_POSITION: f32 = 332.0;
const Y_POSITION: f32 = 200.0;


pub fn initialise_health_bar(world: &mut World) {

    world
        .create_entity()
        .with(HealthBar{
            x_pos: X_POSITION,
            y_pos: Y_POSITION,
            health_stack: vec![],
        })
        .build();
}