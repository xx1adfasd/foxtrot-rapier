use anyhow::{Context, Result};
use bevy::prelude::*;

use bevy_rapier3d::prelude::*;
mod components;
use crate::level_instantiation::spawning::objects::player;
use crate::level_instantiation::spawning::AnimationEntityLink;
use crate::util::log_error::log_errors;
use crate::util::trait_extension::Vec3Ext;
use crate::GameState;
pub use components::*;

/// Handles movement of kinematic character controllers, i.e. entities with the TODO A movement is done by applying forces to the objects.
/// The default forces on a character going right are:  
/// ```text
/// ┌──────────────────────────────┐
/// │            Gravity           │
/// │               ↓              │
/// │              ╔═╗             │
/// │   Walking ─► ║ ║ ◄─ Drag     │
/// │              ╚═╝             │  
/// │                              │
/// └──────────────────────────────┘
/// ```
/// All physics values are assumed to be in SI units, e.g. forces are measured in N and acceleration in m/s².
///
/// The [`Walking`] and [`Jumping`] components are user friendly ways of influencing the corresponding forces.
/// There is no explicit maximum speed since the [`Drag`] counteracts all other forces until reaching an equilibrium.
/// The [`Grounded`] component is used to determine whether the character is on the ground or not.
/// To influence movement, apply your force by adding it to the character's total [`Force`]. Common ways to do this are:
/// - A continuous force like walking: `force.0 += acceleration * mass.0`, with `force`: [`Force`], `mass`: [`Mass`], and a user-defined `acceleration`: [`f32`]
/// - An instantaneous force (i.e. an impulse) like jumping: `force.0 += velocity * mass.0 / time.delta_seconds()`, with `force`: [`Force`], `mass`: [`Mass`], `time`: [`Res<Time>`](Time) and a user-defined `velocity`: [`f32`]
///
/// Note: you might notice that the normal force is not included in the above diagram. This is because the underlying TODO takes care of the character not penetrating colliders, thus emulating this force.
pub struct GeneralMovementPlugin;

impl Plugin for GeneralMovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Model>()
            .register_type::<Grounded>()
            .register_type::<Jumping>()
            .register_type::<Velocity>()
            .register_type::<Walking>()
            .register_type::<CharacterAnimations>()
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(update_grounded)
                    .with_system(apply_walking.after(update_grounded))
                    .with_system(apply_jumping)
                    .with_system(reset_movement_components)
                    //.with_system(rotate_characters)
                    .with_system(play_animations.pipe(log_errors)),
            );
    }
}

fn update_grounded(
    mut query: Query<(Entity, &Transform, &Collider, &mut Grounded, &Velocity, &Up)>,
    names: Query<&Name>,
    rapier_context: Res<RapierContext>,
) {
    for (entity, transform, collider, mut grounded, velocity, up) in &mut query {
        let falling = velocity.linvel.dot(up.0) < -1e-5;
        if !falling && false {
            grounded.force_set(false)
        } else if let Some((entity, toi)) = rapier_context.cast_shape(
            transform.translation,
            transform.rotation.into(),
            velocity.linvel,
            collider,
            player::HEIGHT / 2.0 + player::RADIUS,
            QueryFilter::new()
                .exclude_collider(entity)
                .exclude_sensors(),
        ) {
            let name = names.get(entity).unwrap();
            info!("{} hit by {:?}", name, toi);
            grounded.force_set(true);
        }
    }
}

pub fn reset_movement_components(
    mut forces: Query<&mut ExternalForce>,
    mut impulses: Query<&mut ExternalImpulse>,
    mut walking: Query<&mut Walking>,
    mut jumpers: Query<&mut Jumping>,
) {
    for mut force in &mut forces {
        *force = default();
    }
    for mut impulse in &mut impulses {
        *impulse = default();
    }
    for mut walk in &mut walking {
        walk.direction = None;
    }
    for mut jumper in &mut jumpers {
        jumper.requested = false;
    }
}

pub fn apply_jumping(
    time: Res<Time>,
    mut character_query: Query<(
        &Grounded,
        &mut ExternalForce,
        &mut Velocity,
        &ReadMassProperties,
        &Jumping,
        &Up,
    )>,
) {
    let dt = time.delta_seconds();
    for (grounded, mut force, mut velocity, mass, jump, up) in &mut character_query {
        if jump.requested && grounded.is_grounded() {
            force.force += up.0 * mass.0.mass * jump.speed / dt;

            // Kill any downward velocity. This ensures that repeated jumps are always the same height.
            // Otherwise the falling velocity from the last tick would dampen the jump velocity.
            let velocity_components = velocity.linvel.split(up.0);
            velocity.linvel = velocity_components.horizontal;
        }
    }
}

fn rotate_characters(time: Res<Time>, mut player_query: Query<(&Up, &Velocity, &mut Transform)>) {
    let dt = time.delta_seconds();
    for (up, velocity, mut transform) in player_query.iter_mut() {
        let horizontal_movement = velocity.linvel.split(up.0).horizontal;
        if horizontal_movement.is_approx_zero() {
            continue;
        }
        let target_transform =
            transform.looking_at(transform.translation + horizontal_movement, up.0);
        // Asymptotic averaging
        const SMOOTHNESS: f32 = 4.;
        let scale = (SMOOTHNESS * dt).min(1.);
        let rotation = transform.rotation.slerp(target_transform.rotation, scale);
        transform.rotation = rotation;
    }
}

fn play_animations(
    mut animation_player: Query<&mut AnimationPlayer>,
    characters: Query<(
        &Velocity,
        &Up,
        &Grounded,
        &AnimationEntityLink,
        &CharacterAnimations,
    )>,
) -> Result<()> {
    for (velocity, up, grounded, animation_entity_link, animations) in characters.iter() {
        let mut animation_player = animation_player
            .get_mut(animation_entity_link.0)
            .context("animation_entity_link held entity without animation player")?;

        let has_horizontal_movement = !velocity.linvel.split(up.0).horizontal.is_approx_zero();

        if !grounded.is_grounded() {
            animation_player
                .play(animations.aerial.clone_weak())
                .repeat();
        } else if has_horizontal_movement {
            animation_player.play(animations.walk.clone_weak()).repeat();
        } else {
            animation_player.play(animations.idle.clone_weak()).repeat();
        }
    }
    Ok(())
}

pub fn apply_walking(
    mut character_query: Query<(
        &mut ExternalForce,
        &Walking,
        &mut Velocity,
        &Grounded,
        &ReadMassProperties,
        &Up,
    )>,
) {
    for (mut force, walking, mut velocity, grounded, mass, up) in &mut character_query {
        let mass = mass.0.mass;
        if let Some(acceleration) = walking.get_acceleration(grounded.is_grounded()) {
            let walking_force = acceleration * mass;
            force.force += walking_force;
        } else if grounded.is_grounded() {
            let velocity_components = velocity.linvel.split(up.0);
            if velocity_components.horizontal.length_squared()
                < walking.stopping_speed * walking.stopping_speed
            {
                velocity.linvel = velocity_components.vertical;
            } else if let Some(braking_direction) =
                velocity_components.horizontal.try_normalize().map(|v| -v)
            {
                let braking_force = walking.braking_acceleration * braking_direction * mass;
                force.force += braking_force;
            }
        }
    }
}
