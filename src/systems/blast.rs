use amethyst::{
    core::{
        transform::Transform,
        timing::Time,
    },
    ecs::prelude::{Entities, Join, System, ReadStorage, WriteStorage, Read},
};

use crate::{
    space_shooter::GAME_HEIGHT,
    components::Blast,
};


pub struct BlastSystem;
impl<'s> System<'s> for BlastSystem {

    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Blast>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, blasts, mut transforms, time): Self::SystemData) {
        for (blast_entity, blast_component, blast_transform) in (&*entities, &blasts, &mut transforms).join() {

            if blast_transform.translation()[1] > GAME_HEIGHT {
                let _result = entities.delete(blast_entity);
            }

            blast_transform.translate_x(blast_component.x_velocity * blast_component.velocity_factor * time.delta_seconds());
            blast_transform.translate_y((blast_component.y_velocity * blast_component.velocity_factor + blast_component.speed) * time.delta_seconds());
        }
    }
}