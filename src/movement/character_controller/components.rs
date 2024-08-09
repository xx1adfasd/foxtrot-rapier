use crate::movement::character_controller::AnimationState;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_tnua::{prelude::*, TnuaAnimatingState};
use bevy_tnua_rapier3d::*;
use serde::{Deserialize, Serialize};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Jump>().register_type::<Walk>();
}

#[derive(Bundle)]
pub(crate) struct CharacterControllerBundle {
    pub(crate) walking: Walk,
    pub(crate) sprinting: Sprinting,
    pub(crate) jumping: Jump,
    pub(crate) collider: Collider,
    pub(crate) rigid_body: RigidBody,
    pub(crate) locked_axes: LockedAxes,
    pub(crate) collision_layers: CollisionGroups,
    pub(crate) tnua_sensor_shape: TnuaRapier3dSensorShape,
    pub(crate) tnua_controller: TnuaControllerBundle,
    pub(crate) tnua_rapier3d_io: TnuaRapier3dIOBundle,
    pub(crate) float_height: FloatHeight,
    pub(crate) animation_state: TnuaAnimatingState<AnimationState>,

    pub(crate) colliding_entities: CollidingEntities,
    active_collision_types: ActiveCollisionTypes,
    active_events: ActiveEvents,
    // mass: ColliderMassProperties,
}

impl CharacterControllerBundle {
    pub(crate) fn capsule(height: f32, radius: f32, scale_y: f32) -> Self {
        //Note that the collider is a lying down capsule for the fox,
        // so in fact the radius *2 is its height,
        // and the height is its length
        Self {
            walking: default(),
            sprinting: default(),
            jumping: default(),
            collider: Collider::capsule_z(height, radius),
            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
            collision_layers: CollisionGroups::new(
                Group::GROUP_2,
                Group::GROUP_1 | Group::GROUP_2 | Group::GROUP_3 | Group::GROUP_5,
                // [CollisionLayer::Character],
                // [
                //     CollisionLayer::Player,
                //     CollisionLayer::Character,
                //     CollisionLayer::Terrain,
                //     CollisionLayer::Sensor,
                // ],
            ),
            tnua_sensor_shape: TnuaRapier3dSensorShape(Collider::capsule_z(
                height * 0.95,
                radius * 0.95,
            )),
            tnua_controller: default(),
            tnua_rapier3d_io: default(),
            float_height: FloatHeight((radius / 2.) * scale_y),
            animation_state: default(),
            colliding_entities: default(),
            active_collision_types: default(),
            active_events: ActiveEvents::COLLISION_EVENTS,
            // mass: ColliderMassProperties::Mass(100.),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct Walk {
    /// Top speed on the ground
    pub(crate) speed: f32,
    /// Direction in which we want to walk and turn this tick.
    pub(crate) direction: Option<Vec3>,
}

impl Default for Walk {
    fn default() -> Self {
        Self {
            speed: 8.,
            direction: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct Jump {
    /// The full height of the jump, if the player does not release the button
    pub(crate) height: f32,
    /// Was jump requested this frame?
    pub(crate) requested: bool,
}

#[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct Sprinting {
    /// The speed multiplier when sprinting
    pub(crate) multiplier: f32,
    /// Was sprinting requested?
    pub(crate) requested: bool,
}

impl Default for Sprinting {
    fn default() -> Self {
        Self {
            multiplier: 1.5,
            requested: false,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
/// Must be larger than the height of the entity's center from the bottom of its
/// collider, or else the character will not float and Tnua will not work properly
pub(crate) struct FloatHeight(pub(crate) f32);

impl Default for Jump {
    fn default() -> Self {
        Self {
            height: 1.0,
            requested: false,
        }
    }
}
