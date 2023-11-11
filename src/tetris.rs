use crate::setting::*;
use ggez::{
    graphics::{self, Canvas, DrawParam, Mesh},
    Context, GameResult,
};
use mint::Point2;
use rand::Rng;

struct Background {
    record: [[bool; TETRIS_WIDTH]; TETRIS_HEIGHT],
    color_map: [[usize; TETRIS_WIDTH]; TETRIS_HEIGHT],
}

impl Background {
    pub fn new() -> Background {
        Background {
            record: [[false; TETRIS_WIDTH]; TETRIS_HEIGHT],
            color_map: [[9; TETRIS_WIDTH]; TETRIS_HEIGHT],
        }
    }
    fn draw(&mut self, ctx: &mut Context, cv: &mut Canvas) -> GameResult<()> {
        for i in 0..TETRIS_HEIGHT {
            for j in 0..TETRIS_WIDTH {
                let mut bb = graphics::Rect::new(0.0, 0.0, BLOCK_SIZE as f32, BLOCK_SIZE as f32);
                bb.move_to(Point2 {
                    x: (j * BLOCK_SIZE + SCREEN_OFFSET as usize) as f32,
                    y: (i * BLOCK_SIZE) as f32,
                });
                let shape = Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    bb,
                    BLOCK_COLOR[self.color_map[i][j]],
                )?;
                let outline = Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::stroke(2.0),
                    bb,
                    graphics::Color::new(0.0, 0.0, 0.0, 0.2),
                )?;
                cv.draw(&shape, DrawParam::default());
                cv.draw(&outline, DrawParam::default());
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
                        PIECE_TYPE[old_piece.piece_type][rrow][rcol];
                    if PIECE_TYPE[old_piece.piece_type][rrow][rcol] {
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
        let mut full_line: Vec<usize> = Vec::new();
        for row in 0..TETRIS_HEIGHT {
            let mut full: bool = true;
            for col in 0..TETRIS_WIDTH {
                full &= self.record[row as usize][col as usize];
            }
            if full {
                full_line.push(row as usize);
            }
        }

        // clear full line
        for (_, &row_index) in full_line.iter().enumerate() {
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

#[derive(Copy, Clone, Debug)]
struct Piece {
    piece_type: usize,
    turn: i32,
    x: i32,
    y: i32,
    color: usize,
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
    fn draw(&mut self, ctx: &mut Context, cv: &mut Canvas) -> GameResult<()> {
        for i in 0..4 {
            for j in 0..4 {
                let mut bb = graphics::Rect::new(0.0, 0.0, BLOCK_SIZE as f32, BLOCK_SIZE as f32);
                bb.move_to(Point2 {
                    x: ((self.x + j) * BLOCK_SIZE as i32 + SCREEN_OFFSET) as f32,
                    y: ((self.y + i) * BLOCK_SIZE as i32) as f32,
                });
                let (rj, ri) = self.rotate_index(j as usize, i as usize);
                if PIECE_TYPE[self.piece_type][ri][rj] == true {
                    let shape = Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        bb,
                        BLOCK_COLOR[self.color as usize],
                    )?;
                    let outline = Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::stroke(2.0),
                        bb,
                        graphics::Color::BLACK,
                    )?;
                    cv.draw(&shape, DrawParam::default());
                    cv.draw(&outline, DrawParam::default());
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
                if PIECE_TYPE[self.piece_type][rrow][rcol]
                    && (self.y + row as i32 + y >= TETRIS_HEIGHT as i32
                        || self.y + row as i32 + y < 0
                        || self.x + col as i32 + x >= TETRIS_WIDTH as i32
                        || self.x + col as i32 + x < 0)
                {
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

pub struct Tetris {
    now_piece: Piece,
    back_ground: Background,
}

impl Tetris {
    pub fn new(_ctx: &mut Context) -> Tetris {
        Tetris {
            now_piece: Piece::random_piece(),
            back_ground: Background::new(),
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, cv: &mut Canvas) -> GameResult<()> {
        self.back_ground.draw(ctx, cv)?;
        self.now_piece.draw(ctx, cv)?;

        let mut predict = self.now_piece.clone();
        // predict piece
        for i in 1..20 {
            if !self.now_piece.piece_fit(0, i) || !self.piece_fit(0, i) {
                // draw predict
                predict.y += i - 1;
                break;
            }
        }
        for i in 0..4 {
            for j in 0..4 {
                let mut bb = graphics::Rect::new(0.0, 0.0, BLOCK_SIZE as f32, BLOCK_SIZE as f32);
                bb.move_to(Point2 {
                    x: ((predict.x + j) * BLOCK_SIZE as i32 + SCREEN_OFFSET) as f32,
                    y: ((predict.y + i) * BLOCK_SIZE as i32) as f32,
                });
                let (rj, ri) = predict.rotate_index(j as usize, i as usize);
                if PIECE_TYPE[predict.piece_type][ri][rj] == true {
                    let shape = Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        bb,
                        graphics::Color::new(0.0, 0.0, 0.0, 0.8),
                    )?;
                    let outline = Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::stroke(2.0),
                        bb,
                        graphics::Color::new(0.0, 0.0, 0.0, 0.8),
                    )?;
                    cv.draw(&shape, DrawParam::default());
                    cv.draw(&outline, DrawParam::default());
                }
            }
        }

        Ok(())
    }

    // now_piece drop down 1 block
    pub fn update(&mut self) -> GameResult {
        if self.now_piece.piece_fit(0, 1) && self.piece_fit(0, 1) {
            self.now_piece.move_piece(0, 1);
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
                let (rcol, rrow) = self.now_piece.rotate_index(col, row);
                if PIECE_TYPE[self.now_piece.piece_type][rrow][rcol]
                    && self.back_ground.record[(self.now_piece.y + row as i32 + y) as usize]
                        [(self.now_piece.x + col as i32 + x) as usize]
                {
                    return false;
                }
            }
        }
        true
    }
    pub fn move_piece(&mut self, x: i32, y: i32) {
        if self.now_piece.piece_fit(x, y) && self.piece_fit(x, y) {
            self.now_piece.move_piece(x, y);
        }
    }
    pub fn turn(&mut self) {
        self.now_piece.turn += 1;
        if self.now_piece.piece_fit(0, 0) && self.piece_fit(0, 0) {
            self.now_piece.turn %= 4;
        } else {
            self.now_piece.turn -= 1;
        }
    }
    pub fn speed_drop(&mut self) {
        for i in 1..20 {
            if !self.now_piece.piece_fit(0, i) || !self.piece_fit(0, i) {
                self.check_finish_line(0, i - 1);
                break;
            }
        }
    }
    fn check_finish_line(&mut self, x: i32, y: i32) {
        self.now_piece.move_piece(x, y);
        self.back_ground.store(self.now_piece);
        self.now_piece = Piece::random_piece();
        self.back_ground.finish_line();
        if !self.piece_fit(0, 0) {
            println!("Game Over !");
            self.reset();
        }
    }
    fn reset(&mut self) {
        self.back_ground = Background::new();
    }
}
