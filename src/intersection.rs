use crate::ray::Intersect;
use crate::utils;
use std::any::Any;

#[derive(Debug)]
struct Intersection {
    time: f64,
    object: Box<dyn Intersect>,
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        if (self.time - other.time).abs() > utils::F64_ERROR_MARGIN {
            return false;
        }

        if self.object.type_id() != other.object.type_id() {
            return false;
        };

        true
    }
}

#[derive(Debug)]
struct Intersections {
    objects: Vec<Intersection>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sphere::Sphere;

    #[test]
    fn intersection_stores_time_and_object() {
        let i = Intersection {
            time: 1.3,
            object: Box::new(Sphere::unit()),
        };

        assert_eq!(i.time, 1.3);
        assert_eq!(i.object, Box::new(Sphere::unit()));
    }

    #[test]
    fn intersection_aggregates_are_accessible() {
        let i1 = Intersection {
            time: 1.3,
            object: Box::new(Sphere::unit()),
        };
        let i2 = Intersection {
            time: 1.4,
            object: Box::new(Sphere::unit()),
        };

        let I = Intersections {
            objects: vec![i1, i2],
        };

        assert_eq!(I.objects[0], i1);
        assert_eq!(I.objects[1], i2);
    }
}
