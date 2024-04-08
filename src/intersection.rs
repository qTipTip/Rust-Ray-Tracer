use crate::sphere::Sphere;
use crate::utils;
use std::any::Any;
use std::ops::Index;

#[derive(Debug, Clone)]
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

    pub fn get_hit(&self) -> Option<Intersection> {
        let min_intersect = self
            .objects
            .iter()
            .filter(|a| a.time >= 0.0)
            .min_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        match min_intersect {
            None => None,
            Some(i) => {
                if i.time < 0.0 {
                    None
                } else {
                    Some(i.clone())
                }
            }
        }
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
            objects: vec![i1.clone(), i2.clone()],
        };

        assert_eq!(i.objects[0], i1);
        assert_eq!(i.objects[1], i2);
    }

    mod hits {
        use crate::intersection::{Intersection, Intersections};
        use crate::sphere::Sphere;

        #[test]
        fn intersections_yields_hit() {
            let i1 = Intersection {
                time: 1.0,
                object: Sphere::unit(),
            };
            let i2 = Intersection {
                time: 2.0,
                object: Sphere::unit(),
            };

            let i = Intersections {
                objects: vec![i1.clone(), i2],
            };

            let hit = i.get_hit().unwrap();

            assert_eq!(i1, hit);
        }

        #[test]
        fn intersections_yields_hit_some_negative() {
            let i1 = Intersection {
                time: -1.0,
                object: Sphere::unit(),
            };
            let i2 = Intersection {
                time: 2.0,
                object: Sphere::unit(),
            };

            let i = Intersections {
                objects: vec![i1, i2.clone()],
            };

            let hit = i.get_hit().unwrap();

            assert_eq!(i2, hit);
        }

        #[test]
        fn intersections_yields_hit_all_negative() {
            let i1 = Intersection {
                time: -1.0,
                object: Sphere::unit(),
            };
            let i2 = Intersection {
                time: -2.0,
                object: Sphere::unit(),
            };

            let i = Intersections {
                objects: vec![i1, i2.clone()],
            };

            let hit = i.get_hit();
            assert_eq!(hit, None);
        }

        #[test]
        fn intersection_hits_lowest_nonnegative() {
            let i1 = Intersection {
                time: 5.0,
                object: Sphere::unit(),
            };
            let i2 = Intersection {
                time: 7.0,
                object: Sphere::unit(),
            };

            let i3 = Intersection {
                time: -3.0,
                object: Sphere::unit(),
            };
            let i4 = Intersection {
                time: 2.0,
                object: Sphere::unit(),
            };

            let i = Intersections {
                objects: vec![i1, i2, i3, i4.clone()],
            };

            let hit = i.get_hit().unwrap();
            assert_eq!(hit, i4);
        }
    }
}
