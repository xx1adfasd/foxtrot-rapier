use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
// use bevy_xpbd_3d::prelude::*;

/// Sets up and configures the XPBD physics.
pub(super) fn plugin(app: &mut App) {
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default().with_default_system_setup(false))
        .add_plugins(crate::physics_time::TimePlugin)
        .add_systems(
            crate::physics_time::PhysicsSchedule,
            (
                RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::SyncBackend)
                    .in_set(PhysicsSet::SyncBackend),
                RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::StepSimulation)
                    .in_set(PhysicsSet::StepSimulation),
                RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::Writeback)
                    .in_set(PhysicsSet::Writeback),
            ),
        );
    // Using the default fixed timestep causes issues on faster (165 Hz) machines.
    //  .insert_resource(Time::new_with(Physics::variable(1.0 / 60.)));
}

// remain here as reference.
// #[derive(PhysicsLayer)]
// pub(crate) enum CollisionLayer {
//     Player,1
//     Character,2
//     Terrain,3
//     CameraObstacle,4
//     Sensor,5
// }
