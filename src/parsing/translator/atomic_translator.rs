use crate::handle::Handled;
use crate::handle::order::OrderlyHandled;

pub struct AtomicTranslator<Satellite>(Box<dyn Fn(Vec<Satellite>) -> Option<Satellite>>);

impl<Satellite> AtomicTranslator<Satellite> {
    pub fn new(translation_routine: Box<dyn Fn(Vec<Satellite>) -> Option<Satellite>>) -> Self {
        Self(translation_routine)
    }

    pub fn translate(&self, src: Vec<Satellite>) -> Option<Satellite> {
        (self.0)(src)
    }
}

impl<Satellite> Handled for AtomicTranslator<Satellite>
{
    type HandleCoreType = u8;
}


impl<Satellite> OrderlyHandled for AtomicTranslator<Satellite> {}