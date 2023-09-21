use crate::plantable::Plantable;

// List basic needs
pub struct Microgreen {}

impl Microgreen {
    pub fn new() -> Microgreen {
        println! {"made it here"};
        Microgreen {}
    }
}

impl Plantable for Microgreen {
    fn water_requirements(&self) -> bool {
        return true;
    }

    fn light_requirements(&self) -> bool {
        return true;
    }
}
