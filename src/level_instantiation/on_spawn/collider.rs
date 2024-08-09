use crate::util::error;
use crate::GameSystemSet;
use anyhow::Context;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider as RapierCollider, *};
use oxidized_navigation::NavMeshAffector;
use serde::{Deserialize, Serialize};
use std::iter;

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
struct Collider;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Collider>().add_systems(
        Update,
        spawn.pipe(error).in_set(GameSystemSet::ColliderSpawn),
    );
}

fn spawn(
    collider_marker: Query<Entity, With<Collider>>,
    mut commands: Commands,
    children: Query<&Children>,
    meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Handle<Mesh>>,
) -> anyhow::Result<()> {
    #[cfg(feature = "tracing")]
    let _span = info_span!("read_colliders").entered();
    for parent in collider_marker.iter() {
        for child in iter::once(parent).chain(children.iter_descendants(parent)) {
            let Ok(mesh_handle) = mesh_handles.get(child) else {
                continue;
            };
            // Unwrap cannot fail: we already load all the meshes at startup.
            let mesh = meshes.get(mesh_handle).unwrap();
            let collider = RapierCollider::from_bevy_mesh(
                mesh,
                &bevy_rapier3d::prelude::ComputedColliderShape::TriMesh,
            )
            .context("Failed to create collider from mesh")?;
            commands.entity(child).insert((
                collider,
                // CollisionLayers::new(
                //     [CollisionLayer::Terrain, CollisionLayer::CameraObstacle],
                //     [CollisionLayer::Character],
                // ),
                CollisionGroups::new(Group::GROUP_3 | Group::GROUP_4, Group::GROUP_2),
                ActiveEvents::COLLISION_EVENTS,
                ActiveCollisionTypes::default(),
                NavMeshAffector,
            ));
        }
        commands
            .entity(parent)
            .remove::<Collider>()
            .insert(RigidBody::Fixed);
    }
    Ok(())
}
