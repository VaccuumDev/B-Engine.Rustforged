use bevy::prelude::*;

/** Calculate resisting force direction of 2 colliding objects
arguments are positions of colliding ojbects**/
pub fn resisting_force(pos1: Vec3A, pos2: Vec3A) -> Vec3A {
    let dir = (pos1 - pos2).normalize();
    dir
}
