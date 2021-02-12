use ggez::input::keyboard::KeyCode::*;
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::*;
use mint::Point2;
use rand::Rng;
use std::vec;

const TETRIS_WIDTH: usize = 10;
const TETRIS_HEIGHT: usize = 20;
const BLOCK_SIZE: usize = 30;

struct Tetris {
    nowPiece: Piece,
    backGround: Background,
}
#[derive(Copy, Clone, Debug)]
struct Piece {
    piece_type: i32,
    turn: i32,
    x: i32,
    y: i32,
    color: usize,
}
struct Background {
    record: [[bool; TETRIS_WIDTH]; TETRIS_HEIGHT],
    color_map: [[usize; TETRIS_WIDTH]; TETRIS_HEIGHT],
}

const PIECE_TYPE: [[[bool; 4]; 4]; 7] = [
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

const BLOCK_COLOR: [graphics::Color; 11] = [
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
    graphics::WHITE,
    // BLACK
    graphics::BLACK,
];

impl ggez::event::EventHandler for Tetris {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, 1) {
            self.update()?;
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        let mb = &mut graphics::MeshBuilder::new();

        self.draw(mb)?;

        let mesh = mb.build(ctx)?;
        match graphics::draw(ctx, &mesh, graphics::DrawParam::new()) {
            Ok(_) => (),
            Err(e) => println!("ERROR : {:#?}", e),
        }
        graphics::present(ctx)?;
        Ok(())
    }
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            Escape => event::quit(ctx),
            Left => self.move_piece(-1, 0),
            Right => self.move_piece(1, 0),
            Down => self.move_piece(0, 1),
            Up => self.turn(),
            Space => self.speed_drop(),
            _ => println!("key {:#?} is pressed", keycode),
        }
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("Tetris Game", "BWbwchen")
        .conf(c)
        .build()
        .unwrap();

    let my_game = &mut Tetris::new(ctx);

    event::run(ctx, event_loop, my_game).unwrap();
}

impl Tetris {
    pub fn new(ctx: &mut Context) -> Tetris {
        Tetris {
            nowPiece: Piece::random_piece(),
            backGround: Background::new(),
        }
    }

    fn draw(&mut self, mb: &mut graphics::MeshBuilder) -> GameResult {
        self.backGround.draw(mb)?;
        self.nowPiece.draw(mb)?;
        Ok(())
    }

    // nowPiece drop down 1 block
    fn update(&mut self) -> GameResult {
        if self.nowPiece.piece_fit(0, 1) && self.piece_fit(0, 1) {
            self.nowPiece.move_piece(0, 1);
        } else {
            self.check_finish_line(0, 0);
        }
        Ok(())
    }

    // check whether piece will overlap with background record
    fn piece_fit(&mut self, x: i32, y: i32) -> bool {
        // boundary check
        for row in 0..4 {
            for col in 0..4 {
                let (rcol, rrow) = self.nowPiece.rotate_index(col, row);
                if PIECE_TYPE[self.nowPiece.piece_type as usize][rrow][rcol]
                    && self.backGround.record[(self.nowPiece.y + row as i32 + y) as usize]
                        [(self.nowPiece.x + col as i32 + x) as usize]
                {
                    println!("[ERROR] piece will overlap with record");
                    return false;
                }
            }
        }
        true
    }
    fn move_piece(&mut self, x: i32, y: i32) {
        if self.nowPiece.piece_fit(x, y) && self.piece_fit(x, y) {
            self.nowPiece.move_piece(x, y);
        }
    }
    fn turn(&mut self) {
        self.nowPiece.turn += 1;
        if self.nowPiece.piece_fit(0, 0) && self.piece_fit(0, 0) {
            self.nowPiece.turn %= 4;
        } else {
            self.nowPiece.turn -= 1;
        }
    }
    fn speed_drop(&mut self) {
        for i in 1..20 {
            if !self.nowPiece.piece_fit(0, i) || !self.piece_fit(0, i) {
                self.check_finish_line(0, i - 1);
                break;
            }
        }
    }
    fn check_finish_line(&mut self, x: i32, y: i32) {
        self.nowPiece.move_piece(x, y);
        self.backGround.store(self.nowPiece);
        self.nowPiece = Piece::random_piece();
        self.backGround.finish_line();
    }
}

impl Background {
    pub fn new() -> Background {
        Background {
            record: [[false; TETRIS_WIDTH]; TETRIS_HEIGHT],
            color_map: [[9; TETRIS_WIDTH]; TETRIS_HEIGHT],
        }
    }
    fn draw(&mut self, mb: &mut graphics::MeshBuilder) -> GameResult {
        for i in 0..TETRIS_HEIGHT {
            for j in 0..TETRIS_WIDTH {
                let mut bb = graphics::Rect::new(0.0, 0.0, BLOCK_SIZE as f32, BLOCK_SIZE as f32);
                bb.move_to(Point2 {
                    x: (j * BLOCK_SIZE + 200) as f32,
                    y: (i * BLOCK_SIZE) as f32,
                });
                mb.rectangle(
                    graphics::DrawMode::fill(),
                    bb,
                    BLOCK_COLOR[self.color_map[i][j] as usize],
                );
            }
        }
        Ok(())
    }
    fn store(&mut self, old_piece: Piece) {
        for row in 0..4 {
            for col in 0..4 {
                let (rcol, rrow) = old_piece.rotate_index(col, row);
                if self.in_boundary(old_piece, row as i32, col as i32) {
                    self.record[(old_piece.y + row as i32) as usize]
                        [(old_piece.x + col as i32) as usize] |=
                        PIECE_TYPE[old_piece.piece_type as usize][rrow][rcol];
                    if PIECE_TYPE[old_piece.piece_type as usize][rrow][rcol] {
                        self.color_map[(old_piece.y + row as i32) as usize]
                            [(old_piece.x + col as i32) as usize] = old_piece.color;
                    }
                }
            }
        }
    }
    fn in_boundary(&mut self, old_piece: Piece, row: i32, col: i32) -> bool {
        if old_piece.y + row >= TETRIS_HEIGHT as i32
            || old_piece.y + row < 0
            || old_piece.x + col >= TETRIS_WIDTH as i32
            || old_piece.x + col < 0
        {
            return false;
        }
        true
    }
    fn finish_line(&mut self) {
        // detect how many full line
        let mut fullLine: Vec<usize> = Vec::new();
        for row in 0..TETRIS_HEIGHT {
            let mut full: bool = true;
            for col in 0..TETRIS_WIDTH {
                full &= self.record[row as usize][col as usize];
            }
            if full {
                fullLine.push(row as usize);
            }
        }

        //fullLine.reverse();

        // clear full line
        for (_, &row_index) in fullLine.iter().enumerate() {
            for row in (1..=row_index).rev() {
                for col in 0..TETRIS_WIDTH {
                    self.record[row][col] = self.record[row - 1][col];
                    self.color_map[row][col] = self.color_map[row - 1][col];
                }
            }
            // reset top line
            for col in 0..TETRIS_WIDTH {
                self.record[0][col] = false;
                self.color_map[0][col] = 9;
            }
        }
    }
}

impl Piece {
    pub fn random_piece() -> Piece {
        let mut rng = rand::thread_rng();
        Piece {
            piece_type: rng.gen_range(0..7),
            x: 3,
            y: 0,
            turn: 0,
            color: rng.gen_range(0..9),
        }
    }
    fn draw(&mut self, mb: &mut graphics::MeshBuilder) -> GameResult {
        for i in 0..4 {
            for j in 0..4 {
                let mut bb = graphics::Rect::new(0.0, 0.0, BLOCK_SIZE as f32, BLOCK_SIZE as f32);
                bb.move_to(Point2 {
                    x: ((self.x + j) * BLOCK_SIZE as i32 + 200) as f32,
                    y: ((self.y + i) * BLOCK_SIZE as i32) as f32,
                });
                let (rj, ri) = self.rotate_index(j as usize, i as usize);
                if PIECE_TYPE[self.piece_type as usize][ri][rj] == true {
                    mb.rectangle(
                        graphics::DrawMode::fill(),
                        bb,
                        BLOCK_COLOR[self.color as usize],
                    );
                }
            }
        }
        Ok(())
    }
    // check whether we can drop 1 block
    fn piece_fit(&mut self, x: i32, y: i32) -> bool {
        // boundary check
        for row in 0..4 {
            for col in 0..4 {
                let (rcol, rrow) = self.rotate_index(col, row);
                if PIECE_TYPE[self.piece_type as usize][rrow][rcol]
                    && (self.y + row as i32 + y >= TETRIS_HEIGHT as i32
                        || self.y + row as i32 + y < 0
                        || self.x + col as i32 + x >= TETRIS_WIDTH as i32
                        || self.x + col as i32 + x < 0)
                {
                    println!("[ERROR] piece will out of boundary");
                    return false;
                }
            }
        }
        true
    }
    fn move_piece(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
    fn rotate_index(self, col: usize, row: usize) -> (usize, usize) {
        match self.turn % 4 {
            0 => return (col, row),
            1 => return (row, 3 - col),
            2 => return (3 - col, 3 - row),
            3 => return (3 - row, col),
            _ => return (col, row),
        };
    }
}
