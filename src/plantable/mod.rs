pub use self::microgreen::Microgreen;
mod microgreen;

pub trait Plantable {
    // what if it returned the next task instead like stop the water or turn off the lamps
    fn water_requirements(&self) -> bool;

    fn light_requirements(&self) -> bool;
}
