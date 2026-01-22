use crate::b_physics::utils::resisting_force;
use bevy::{
    math::bounding::{Aabb3d, BoundingVolume},
    prelude::*,
};

pub struct BPhysics;

impl Plugin for BPhysics {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(FixedUpdate, tick);
    }
}

/// Physical body implementation for Bevy
///
/// This component adds object to physical processing, iw wil automatically give it a collider and
/// rapier3d::RigidBody
const GRAVITY_VECTOR: Vec3 = vec3(0f32, -0.01, 0f32);

#[derive(Component)]
pub struct PhysBody {
    speed: Vec3,      // dq/dt
    ang_vel: Vec3,    // d'phi'/dt
    collider: Aabb3d, // Collider uses local coordinates. Use to_global() or intersects()
    half_size: Vec3,
}

impl PhysBody {
    pub fn new(trans: Transform, hs: Vec3) -> Self {
        PhysBody {
            speed: Vec3::ZERO,
            ang_vel: Vec3::ZERO,
            collider: Aabb3d::new(trans.translation, hs),
            half_size: hs,
        }
    }

    pub fn collision(&mut self, dir: Vec3A) {
        let radius = self.collider.half_size();

        self.speed.y = -self.speed.y * 0.1 * dir.y;
        self.speed.x = -self.speed.x * 0.1 * dir.x;
        self.speed.z = -self.speed.z * 0.1 * dir.z;

        // Approximation of M = F*r*sin(alpha)
        self.ang_vel += Vec3::new(
            radius.x * self.speed.x,
            radius.y * self.speed.y,
            radius.z * self.speed.z,
        );
    }
    pub fn apply_force(&mut self, force: Vec3) {
        self.speed += force;
    }
}

/// Function, needed to setup physics and connect physical things with Bevyan component
#[allow(unused)]
fn setup(query: Query<(&mut Transform, &mut PhysBody)>) {}

/// This method updates physics calculations and synchronises position with physical model
#[allow(unused_parens)]
fn tick(mut query: Query<(&mut Transform, &mut PhysBody)>) {
    for (mut trans, mut body) in query.iter_mut() {
        // unimplemented
        /*if (body.collider.intersects(.collider)) {
            body.collision();
        }*/
        if (trans.translation.y - body.half_size.y <= 0f32) {
            body.collision(resisting_force(
                trans.translation.to_vec3a(),
                vec3a(trans.translation.x, -1f32, trans.translation.z),
            ));
        }

        trans.translation.x += body.speed.x;
        trans.translation.y += body.speed.y;
        trans.translation.z += body.speed.z;

        trans.rotation.x += body.ang_vel.x;
        trans.rotation.y += body.ang_vel.y;
        trans.rotation.z += body.ang_vel.z;

        let speed_copy: Vec3 = body.speed;

        // Gravity
        body.apply_force(GRAVITY_VECTOR);

        // Damping
        body.apply_force(vec3(
            -speed_copy.x * 0.1,
            -speed_copy.y * 0.1,
            -speed_copy.z * 0.1,
        ));
        let delta_ang = body.ang_vel;
        if delta_ang.length_squared() > 0.0 {
            let dq = Quat::from_xyzw(0.5 * delta_ang.x, 0.5 * delta_ang.y, 0.5 * delta_ang.z, 1.0)
                .normalize();
            trans.rotation = (dq * trans.rotation).normalize();
        }
    }
}
