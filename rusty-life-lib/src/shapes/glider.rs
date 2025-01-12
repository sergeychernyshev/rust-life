use super::LifeShape;

pub fn new() -> LifeShape {
    LifeShape::from_string(" 🦠 \n  🦠\n🦠🦠🦠")
}

#[test]
// glider shifts diagonally by once cell every 4 steps
fn is_spaceship() {
    const PERIOD: usize = 4;
    const SHIFT_X: usize = 1;
    const SHIFT_Y: usize = 1;

    let shape = new();

    let field_width = shape.width + SHIFT_X;
    let field_height = shape.height + SHIFT_Y;

    let mut game = super::super::GameOfLife::new(field_width, field_height);
    game.add(shape, 0, 0);

    for _ in 0..PERIOD {
        game.calculate_next_step();
    }

    let mut game2 = super::super::GameOfLife::new(field_width, field_height);
    game2.add(new(), SHIFT_X, SHIFT_Y);
    assert_eq!(game.cells, game2.cells);
}
