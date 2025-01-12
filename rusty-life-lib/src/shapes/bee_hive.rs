use super::LifeShape;

pub fn new() -> LifeShape {
    LifeShape::from_string(" 🦠🦠 \n🦠  🦠\n 🦠🦠 ")
}

#[test]
fn is_still() {
    let mut game = super::super::GameOfLife::from_shape(new());
    game.calculate_next_step();
    assert_eq!(game.cells, new().cells);
}
