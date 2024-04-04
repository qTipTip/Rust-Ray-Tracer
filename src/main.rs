mod canvas;
mod clock;
mod color;
mod image;
mod intersection;
mod matrix;
mod projectile;
mod ray;
mod sphere;
mod transformations;
mod tuple;
mod utils;

use crate::clock::clock;
use crate::projectile::projectile;

fn main() {
    clock();
    projectile();
}
