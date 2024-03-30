mod tuple;
mod color;
mod canvas;
mod image;
mod matrix;
mod utils;
mod transformations;
mod clock;
mod projectile;

use crate::clock::clock;
use crate::projectile::projectile;

fn main() {
    clock();
    projectile();
}
