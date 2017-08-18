use collision::{Aabb, Discrete, MinMax};
use cgmath::{Array, BaseFloat, EuclideanSpace, VectorSpace, ElementWise};

pub mod sweep_prune;
pub mod brute_force;

use std::fmt::Debug;
use std::clone::Clone;
use std;

#[derive(Debug)]
pub struct BroadCollisionInfo<ID, S, V, P, A>
where
    ID: Debug + Clone,
    S: BaseFloat,
    V: VectorSpace<Scalar = S> + ElementWise + Array<Element = S>,
    P: EuclideanSpace<Scalar = S, Diff = V> + MinMax,
    A: Aabb<S, V, P> + Discrete<A>,
{
    id: ID,
    bound: A,
    m: std::marker::PhantomData<(S, V, P)>,
}

impl<ID, S, V, P, A> BroadCollisionInfo<ID, S, V, P, A>
where
    ID: Debug + Clone,
    S: BaseFloat,
    V: VectorSpace<Scalar = S>
        + ElementWise
        + Array<Element = S>,
    P: EuclideanSpace<Scalar = S, Diff = V> + MinMax,
    A: Aabb<S, V, P> + Discrete<A>,
{
    pub fn new(id: ID, bound: A) -> Self {
        Self {
            id,
            bound,
            m: std::marker::PhantomData,
        }
    }
}

pub trait BroadPhase<ID, S, V, P, A>
where
    ID: Debug + Clone,
    S: BaseFloat,
    V: VectorSpace<Scalar = S> + ElementWise + Array<Element = S>,
    P: EuclideanSpace<Scalar = S, Diff = V> + MinMax,
    A: Aabb<S, V, P> + Discrete<A>,
{
    fn compute(&mut self, shapes: &mut Vec<BroadCollisionInfo<ID, S, V, P, A>>) -> Vec<(ID, ID)>;
}