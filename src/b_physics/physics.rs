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

const GRAVITY_VECTOR: Vec3 = vec3(0f32, -15f32, 0f32);

/// Physical Body component
#[derive(Component)]
pub struct PhysBody {
    speed: Vec3,      // dq/dt
    ang_vel: Vec3A,   // d'phi'/dt
    collider: Aabb3d, // Collider uses local coordinates. Use to_global() or intersects()
    half_size: Vec3,
    mass: f32,
}

impl PhysBody {
    pub fn new(trans: Transform, hs: Vec3, m: f32) -> Self {
        PhysBody {
            speed: Vec3::ZERO,
            ang_vel: Vec3A::ZERO,
            collider: Aabb3d::new(trans.translation, hs),
            half_size: hs,
            mass: m,
        }
    }

    pub fn collision(&mut self, dir: Vec3A) {
        let radius = self.collider.half_size();

        self.speed = -self.speed * dir.to_vec3() / self.mass;

        // Approximation of M = F*r*sin(alpha)
        //self.apply_moment(radius * self.speed.to_vec3a());
    }
    /// Sometimes you need to apply force...
    pub fn apply_force(&mut self, force: Vec3) {
        self.speed += force / vec3(self.mass, self.mass, self.mass);
    }
    ///...But in some cases you already know acceleration
    pub fn apply_acc(&mut self, acc: Vec3) {
        self.speed += acc;
    }

    pub fn apply_moment(&mut self, moment: Vec3A) {
        self.ang_vel = moment;
    }
}

/// Function, needed to setup physics and connect physical things with Bevyan component
#[allow(unused)]
fn setup(query: Query<(&mut Transform, &mut PhysBody)>) {}

/// This method updates physics calculations and synchronises position with physical model
#[allow(unused_parens)]
fn tick(mut query: Query<(&mut Transform, &mut PhysBody)>, time: Res<Time>) {
    for (mut trans, mut body) in query.iter_mut() {
        let speed_copy: Vec3 = body.speed;
        let ang_vel_copy: Vec3A = body.ang_vel;

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

        // applying speed
        trans.translation += body.speed * time.delta_secs();

        // rotation is Quat and ang_vel is Vec3, so i cant just equate them
        trans.rotation.x += body.ang_vel.x;
        trans.rotation.y += body.ang_vel.y;
        trans.rotation.z += body.ang_vel.z;

        // Gravity
        body.apply_acc(GRAVITY_VECTOR * time.delta_secs());

        // Damping
        body.apply_force(-speed_copy);
        body.apply_moment(-ang_vel_copy);

        /*let delta_ang = body.ang_vel;
        if delta_ang.length_squared() > 0.0 {
            let dq = Quat::from_xyzw(0.5 * delta_ang.x, 0.5 * delta_ang.y, 0.5 * delta_ang.z, 1.0)
                .normalize();
            trans.rotation = (dq * trans.rotation).normalize();
        }*/
    }
}
