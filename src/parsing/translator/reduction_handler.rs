use std::marker::PhantomData;
use crate::handle::Handled;
use crate::handle::order::OrderlyHandled;

pub struct ReductionHandler<Satellite, F>
where
    F: Fn(Vec<&Satellite>) -> Satellite,
{
    func: F,
    phantom_data: PhantomData<Satellite>,
}

impl<Satellite, F> ReductionHandler<Satellite, F>
where
    F: Fn(Vec<&Satellite>) -> Satellite,
{
    pub fn new(routine: F) -> Self {
        Self { func: routine, phantom_data: Default::default() }
    }
}

impl<Satellite, F> Handled for ReductionHandler<Satellite, F>
where
    F: Fn(Vec<&Satellite>) -> Satellite,
{
    type HandleCoreType = u8;
}


impl<Satellite, F> OrderlyHandled for ReductionHandler<Satellite, F>
where
    F: Fn(Vec<&Satellite>) -> Satellite,
{}