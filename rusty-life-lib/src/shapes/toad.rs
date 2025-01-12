use super::LifeShape;

pub fn new() -> LifeShape {
    LifeShape::from_string("    \n 🦠🦠🦠\n🦠🦠🦠 \n    ")
}

#[test]
fn is_oscilating() {
    const PERIOD: usize = 2;

    let mut game = super::super::GameOfLife::from_shape(new());

    for _ in 0..PERIOD {
        game.calculate_next_step();
    }

    assert_eq!(game.cells, new().cells);
}
