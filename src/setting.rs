use ggez::graphics;

pub const TETRIS_WIDTH: usize = 10;
pub const TETRIS_HEIGHT: usize = 20;
pub const BLOCK_SIZE: usize = 30;
pub const SCREEN_OFFSET: i32 = 250;

pub const PIECE_TYPE: [[[bool; 4]; 4]; 7] = [
    /*
     * ----
     */
    [
        [false, false, false, false],
        [true, true, true, true],
        [false, false, false, false],
        [false, false, false, false],
    ],
    /*
     *  |
     * ---
     */
    [
        [false, false, false, false],
        [false, false, true, false],
        [false, true, true, true],
        [false, false, false, false],
    ],
    /*
     * |
     * |---
     */
    [
        [false, false, false, false],
        [true, false, false, false],
        [true, true, true, false],
        [false, false, false, false],
    ],
    /*
     *    |
     * ---|
     */
    [
        [false, false, false, false],
        [false, false, true, false],
        [true, true, true, false],
        [false, false, false, false],
    ],
    /*
     * --
     * --
     */
    [
        [false, false, false, false],
        [false, true, true, false],
        [false, true, true, false],
        [false, false, false, false],
    ],
    /*
     *  |-
     * --
     */
    [
        [false, false, false, false],
        [false, false, true, true],
        [false, true, true, false],
        [false, false, false, false],
    ],
    /*
     * -|
     *  |-
     */
    [
        [false, false, false, false],
        [false, true, true, false],
        [false, false, true, true],
        [false, false, false, false],
    ],
];

pub const BLOCK_COLOR: [graphics::Color; 11] = [
    // RED
    graphics::Color::new(255.0, 0.0, 0.0, 1.0),
    // ORANGE
    graphics::Color::new(255.0, 127.0, 0.0, 1.0),
    // YELLOW
    graphics::Color::new(255.0, 255.0, 0.0, 1.0),
    // GREEN
    graphics::Color::new(0.0, 255.0, 0.0, 1.0),
    // BLUE
    graphics::Color::new(0.0, 0.0, 255.0, 1.0),
    // INDIGO
    graphics::Color::new(80.0, 0.0, 255.0, 1.0),
    // VIOLET
    graphics::Color::new(255.0, 0.0, 255.0, 1.0),
    // MAGENTA
    graphics::Color::new(255.0, 0.0, 123.0, 1.0),
    // GOLD
    graphics::Color::new(255.0, 182.0, 0.0, 1.0),
    // WHITE
    graphics::Color::WHITE,
    // BLACK
    graphics::Color::BLACK,
];
