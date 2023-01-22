use chrono;

struct Batch {
    // a plant type being grown by a grower at a specific time
    grower: Grower,
    plant_type: PlantType,
    planted_at: u32,
}

impl Batch {
    fn new(grower: &Grower, plant_type: &PlantType) -> Batch {
        println!("{:?}", chrono::offset::Utc::now());
    }

}