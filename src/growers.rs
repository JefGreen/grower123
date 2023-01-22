pub mod raspberry_pie;

pub trait Grower {
    // what if it returned the next task instead like stop the water or turn off the lamps
    fn water(&self) -> ();

    fn light(&self) -> ();
}
