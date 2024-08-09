use crate::{
    level_instantiation::on_spawn::player,
    movement::character_controller::CharacterControllerBundle, GameState,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct Npc;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Npc>()
        .add_systems(Update, spawn.run_if(in_state(GameState::Playing)));
}

fn spawn(follower: Query<(Entity, &Transform), Added<Npc>>, mut commands: Commands) {
    for (entity, transform) in follower.iter() {
        commands
            .entity(entity)
            .insert((CharacterControllerBundle::capsule(
                player::HEIGHT,
                player::RADIUS,
                transform.scale.y,
            ),))
            .with_children(|parent| {
                parent.spawn((
                    Name::new("NPC Dialog Collider"),
                    Collider::cylinder(player::HEIGHT / 2., player::RADIUS * 5.),
                    CollisionGroups::new(Group::GROUP_5, Group::GROUP_1),
                    // CollisionLayers::new([CollisionLayer::Sensor], [CollisionLayer::Player]),
                    ActiveEvents::COLLISION_EVENTS,
                    ActiveCollisionTypes::default(),
                    Sensor,
                ));
            });
    }
}
