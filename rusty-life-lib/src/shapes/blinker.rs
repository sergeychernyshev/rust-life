use super::LifeShape;

pub fn new() -> LifeShape {
    horizontal()
}

pub fn horizontal() -> LifeShape {
    LifeShape {
        width: 3,
        height: 3,
        cells: vec![false, false, false, true, true, true, false, false, false],
    }
}
pub fn vertical() -> LifeShape {
    LifeShape {
        width: 3,
        height: 3,
        cells: vec![false, true, false, false, true, false, false, true, false],
    }
}

#[test]
fn is_blinking() {
    let mut game = super::super::GameOfLife::from_shape(horizontal());
    game.calculate_next_step();
    assert_eq!(game.cells, vertical().cells);
    game.calculate_next_step();
    assert_eq!(game.cells, horizontal().cells);
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
