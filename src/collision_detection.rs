// TODO: Use a 3rd party plugin for physics
// bevy rapier or bevy xpbd

use bevy::prelude::*;

use std::collections::HashMap;

use crate::{
    asteroids::Asteroid,
    health::Health,
    schedule::InGameSet,
    spaceship::{Spaceship, SpaceshipMissile},
};

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

#[derive(Component, Debug)]
pub struct CollisionDamage {
    pub amount: f32,
}

impl CollisionDamage {
    pub fn new(amount: f32) -> Self {
        Self { amount }
    }
}

#[derive(Event, Debug)]
pub struct CollisionEvent {
    pub entity: Entity,
    pub collided_entity: Entity,
}

impl CollisionEvent {
    pub fn new(entity: Entity, collided_entity: Entity) -> Self {
        Self {
            entity,
            collided_entity,
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
                (
                    handle_collision::<Asteroid>,
                    handle_collision::<Spaceship>,
                    handle_collision::<SpaceshipMissile>,
                ),
                apply_collision_damage,
            )
                .chain()
                .in_set(InGameSet::EntityUpdates),
        )
        .add_event::<CollisionEvent>();
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

fn handle_collision<T>(
    mut collision_event_writer: EventWriter<CollisionEvent>,
    query: Query<(Entity, &Collider), With<T>>,
) where
    T: Component,
{
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {
            // Entity collided with another of the same type - ignore.
            if query.get(collided_entity).is_ok() {
                continue;
            }
            // Send the Event.
            collision_event_writer.send(CollisionEvent::new(entity, collided_entity));
        }
    }
}

fn apply_collision_damage(
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut health_query: Query<&mut Health>,
    collision_damage_query: Query<&CollisionDamage>,
) {
    for &CollisionEvent {
        entity,
        collided_entity,
    } in collision_event_reader.read()
    {
        let Ok(mut health) = health_query.get_mut(entity) else {
            continue;
        };

        let Ok(damage) = collision_damage_query.get(collided_entity) else {
            continue;
        };

        health.value -= damage.amount;
    }
}
