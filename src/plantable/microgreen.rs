use crate::plantable::Plantable;

// List basic needs
pub struct Microgreen {}

impl Plantable for Microgreen {
    fn needs_water(&self) -> bool {
        return true;
    }

    fn needs_light(&self) -> bool {
        return true;
    }
}
