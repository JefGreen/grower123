use crate::plantable::Plantable;

// List basic needs
pub struct Microgreen {}

impl Plantable for Microgreen {
    fn needs_water(&self) {
        return true;
    }

    fn needs_light(&self) {
        return true;
    }
}
