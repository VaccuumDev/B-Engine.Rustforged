use bevy::prelude::*;

pub struct BPhys;

impl Plugin for BPhys {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, phys_tick);
    }
}

const GRAVITY_CONSTANT: f32 = 10f32;

/// Main physics component
#[derive(Component, Default)]
pub struct PhysBody {
    collider_radius: f32,
    mass: u16,    // Mass
    v: Vec3,      // dq/dt vector
    moment: Quat, // Moment of rotation
}

impl PhysBody {
    pub fn new(m: u16, r: f32) -> Self {
        PhysBody {
            mass: m,
            collider_radius: r,
            ..default()
        }
    }
}

#[allow(unused_parens)]
fn phys_tick(mut query: Query<(&mut PhysBody, &mut Transform)>, time: Res<Time>) {
    for (mut body, mut transform) in &mut query {
        transform.translation.x += body.v.x;
        transform.translation.y += body.v.y;
        transform.translation.z += body.v.z;
        transform.rotation += body.moment;

        if (transform.translation.distance(Vec3::ZERO) <= body.collider_radius) {
            body.v.y -= GRAVITY_CONSTANT;
            info!("{}", transform.translation);
        } else {
            body.v = Vec3::ZERO;
            info!("COLLISION");
        }
    }
}
