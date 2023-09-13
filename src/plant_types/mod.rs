pub use self::mocked_pie::MockedPie;
mod mocked_pie;

pub trait Plantable {
    // what if it returned the next task instead like stop the water or turn off the lamps
    fn needs_water(&self) -> ();

    fn needs_light(&self) -> ();
}
