use crate::handles::Handled;
use crate::handles::specials::OrderlyHandled;

pub(super) type LeafSatelliteBuilder<Context, Satellite> =
Box<dyn Fn(&mut Context, String) -> Satellite>;

pub(super) type SatelliteReducer<Context, Satellite> =
Box<dyn Fn(&mut Context, Vec<Satellite>) -> Satellite>;

impl<Context, Satellite> Handled for SatelliteReducer<Context, Satellite> {
    type HandleCoreType = u8;
}

impl<Context, Satellite> OrderlyHandled for SatelliteReducer<Context, Satellite> {}
