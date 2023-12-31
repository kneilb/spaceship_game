// TODO: Use a 3rd party plugin for physics
// bevy rapier or bevy xpbd

use bevy::prelude::*;

use std::collections::HashMap;

use crate::{asteroids::Asteroid, schedule::InGameSet, spaceship::Spaceship};

#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            collision_detector.in_set(InGameSet::CollisionDetection),
        )
        .add_systems(
            Update,
            (
                handle_collision::<Asteroid>,
                handle_collision::<Spaceship>,
            )
                .in_set(InGameSet::DespawnEntities),
        );
    }
}

fn collision_detector(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    // phase 1: detect collisions
    for (entity_a, transform_a, collider_a) in query.iter() {
        for (entity_b, transform_b, collider_b) in query.iter() {
            if entity_a != entity_b {
                let distance = transform_a
                    .translation()
                    .distance(transform_b.translation());
                if distance < collider_a.radius + collider_b.radius {
                    colliding_entities
                        .entry(entity_a)
                        .or_insert_with(Vec::new)
                        .push(entity_b);
                }
            }
        }
    }

    // phase 2: add results to colliders
    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collider
                .colliding_entities
                .extend(collisions.iter().copied());
        }
    }
}

fn handle_collision<T>(mut commands: Commands, query: Query<(Entity, &Collider), With<T>>)
where
    T: Component,
{
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {
            // Entity collided with another of the same type - ignore.
            if query.get(collided_entity).is_ok() {
                continue;
            }
            // Despawn the Entity.
            commands.entity(entity).despawn_recursive();
        }
    }
}
