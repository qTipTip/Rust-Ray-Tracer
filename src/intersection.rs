use crate::utils;
use std::any::Any;
use std::ops::Index;
use crate::sphere::Sphere;

#[derive(Debug, Copy, Clone)]
pub struct Intersection {
    pub time: f64,
    pub object: Sphere, // TODO: Make this an object that implements Intersect
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
pub struct Intersections {
    pub objects: Vec<Intersection>,
}

impl Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        &self.objects[index]
    }
}

impl Intersections {
    pub fn len(&self) -> usize {
        self.objects.len()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::sphere::Sphere;

    #[test]
    fn intersection_stores_time_and_object() {
        let i = Intersection {
            time: 1.3,
            object: Sphere::unit(),
        };

        assert_eq!(i.time, 1.3);
        assert_eq!(i.object, Sphere::unit());
    }

    #[test]
    fn intersection_aggregates_are_accessible() {
        let i1 = Intersection {
            time: 1.3,
            object: Sphere::unit(),
        };
        let i2 = Intersection {
            time: 1.4,
            object: Sphere::unit(),
        };

        let i = Intersections {
            objects: vec![i1, i2],
        };

        assert_eq!(i.objects[0], i1);
        assert_eq!(i.objects[1], i2);
    }
}
