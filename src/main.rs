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
mod renders;

use crate::clock::clock;
use crate::projectile::projectile;
use crate::renders::render_sphere;

fn main() {
    // clock();
    // projectile();
    render_sphere()
}
