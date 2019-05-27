use amethyst::{
    ecs::prelude::{Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent},
    core::{
        transform::Transform,
        nalgebra::Vector3,
    },
};

use crate::{
    components::Blast,
    resources::SpriteResource,
};


const BLAST_STARTING_SPEED: f32 = 100.0;
const BLAST_HITBOX_RADIUS: f32 = 2.0;
const BLAST_VELOCITY_FACTOR: f32 = 0.5;


pub fn fire_blast(entities: &Entities, blast_resource: &ReadExpect<SpriteResource>, sprite_number: usize, fire_position: Vector3<f32>, damage: f32, x_velocity: f32, y_velocity: f32, lazy_update: &ReadExpect<LazyUpdate>) {
    let blast_entity: Entity = entities.create();

    let mut local_transform = Transform::default();
    local_transform.set_position(fire_position);

    let sprite_render = SpriteRender {
        sprite_sheet: blast_resource.sprite_sheet.clone(),
        sprite_number: sprite_number,
    };

    lazy_update.insert(blast_entity, sprite_render);
    lazy_update.insert(blast_entity, Blast {speed: BLAST_STARTING_SPEED, hitbox_radius: BLAST_HITBOX_RADIUS, damage: damage, x_velocity: x_velocity, y_velocity: y_velocity, velocity_factor: BLAST_VELOCITY_FACTOR});
    lazy_update.insert(blast_entity, local_transform);
    lazy_update.insert(blast_entity, Transparent);

}