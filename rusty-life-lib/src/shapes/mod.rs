pub mod beacon;
pub mod bee_hive;
pub mod blinker;
pub mod block;
pub mod boat;
pub mod glider;
pub mod loaf;
pub mod toad;
pub mod tub;

pub struct LifeShape {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<bool>,
}

impl LifeShape {
    fn from_string(cells: &str) -> LifeShape {
        let width = cells.lines().next().unwrap().chars().count();
        let height = cells.lines().count();

        let mut result = Vec::new();
        for c in cells.chars().filter(|c| *c != '\n' && *c != '\r') {
            result.push(c != ' ');
        }
        LifeShape {
            width,
            height,
            cells: result,
        }
    }
}
