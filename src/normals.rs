use crate::tuple::Tuple;

pub trait Normal {
    fn normal_at(&self, point: Tuple) -> Tuple;
}