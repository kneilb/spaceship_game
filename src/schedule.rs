use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InGameSet {
    DespawnEntities,
    UserInput,
    EntityUpdates,
    CollisionDetection,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                InGameSet::DespawnEntities,
                // Flush commands here (see `apply_deferred` below)
                InGameSet::UserInput,
                InGameSet::EntityUpdates,
                InGameSet::CollisionDetection,
            )
                .chain(),
        )
        .add_systems(
            Update,
            apply_deferred
                .after(InGameSet::DespawnEntities)
                .before(InGameSet::UserInput),
        );
    }
}
