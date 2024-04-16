mod canvas;
mod clock;
mod color;
mod image;
mod intersection;
mod lights;
mod material;
mod matrix;
mod normals;
mod projectile;
mod ray;
mod renders;
mod sphere;
mod transformations;
mod tuple;
mod utils;

use crate::clock::clock;
use crate::projectile::projectile;
use crate::renders::{render_sphere, render_sphere_with_shading};

fn main() {
    // clock();
    // projectile();
    // render_sphere();
    render_sphere_with_shading()
}
