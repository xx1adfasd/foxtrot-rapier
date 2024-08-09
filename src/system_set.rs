use crate::GameState;
use bevy::prelude::*;
use bevy_dolly::prelude::DollyUpdateSet;
use bevy_gltf_blueprints::GltfBlueprintsSet;
use bevy_yarnspinner::prelude::YarnSpinnerSystemSet;
use bevy_yarnspinner_example_dialogue_view::ExampleYarnSpinnerDialogueViewSystemSet;

pub(super) fn plugin(app: &mut App) {
    app.configure_sets(
        Update,
        (
            GltfBlueprintsSet::AfterSpawn,
            YarnSpinnerSystemSet,
            GameSystemSet::ColliderSpawn,
            GameSystemSet::Navigation,
            GameSystemSet::PlayerEmbodiment,
            GameSystemSet::GeneralMovement,
            GameSystemSet::PlayAnimation,
            GameSystemSet::UpdateInteractionOpportunities,
            GameSystemSet::Dialog,
            ExampleYarnSpinnerDialogueViewSystemSet,
        )
            .chain(),
    )
    .configure_sets(
        Update,
        (
            GameSystemSet::ColliderSpawn,
            GameSystemSet::UpdateInteractionOpportunities,
            GameSystemSet::Navigation,
            GameSystemSet::PlayerEmbodiment,
            GameSystemSet::GeneralMovement,
            GameSystemSet::PlayAnimation,
            GameSystemSet::Dialog,
        )
            .run_if(in_state(GameState::Playing)),
    );

    //https://github.com/dimforge/bevy_rapier/issues/564

    app.configure_sets(
        PostUpdate,
        (GameSystemSet::CameraUpdate, DollyUpdateSet)
            .chain()
            .after(bevy_rapier3d::plugin::PhysicsSet::Writeback)
            .before(bevy::transform::TransformSystem::TransformPropagate)
            .run_if(in_state(GameState::Playing)),
    );
}

/// Used for ordering systems across Foxtrot.
/// Note that the order of items of this enum is not necessarily the order of execution.
/// Look at [`crate::system_set::plugin`] for the actual order used.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub(crate) enum GameSystemSet {
    /// Goes through entities tagged with `Collider` in Blender
    /// and inserts a proper XPBD collider
    ColliderSpawn,
    /// Run path finding
    Navigation,
    /// Update interaction opportunities with the environment
    UpdateInteractionOpportunities,
    /// Handle player input
    PlayerEmbodiment,
    /// Handle movement for character controllers
    GeneralMovement,
    /// Play animations
    PlayAnimation,
    /// Update the camera transform
    CameraUpdate,
    /// Interacts with Yarn Spinner for dialog logic
    Dialog,
}
