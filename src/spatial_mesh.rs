use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct SpatialEntity {
    pub id: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub velocity_z: f32,
}

pub struct SpatialGrid {
    cell_size: f32,
    cells: HashMap<(i32, i32), Vec<SpatialEntity>>,
}

impl SpatialGrid {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
        }
    }

    pub fn insert(&mut self, entity: SpatialEntity) {
        let cx = (entity.x / self.cell_size).floor() as i32;
        let cz = (entity.z / self.cell_size).floor() as i32;
        self.cells.entry((cx, cz)).or_default().push(entity);
    }
}
