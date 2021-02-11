use ggez::*;
use mint::Point2;
use rand::Rng;

const TETRIS_WIDTH: usize = 10;
const TETRIS_HEIGHT: usize = 20;
const BLOCK_SIZE: usize = 30;

struct Tetris {
    nowPiece: Piece,
    backGround: Background,
}
struct Piece {
    pieceType: PieceType,
    turn: i32,
    x: i32,
    y: i32,
    color: graphics::Color,
}
struct Background {
    record: [[Block; TETRIS_WIDTH]; TETRIS_HEIGHT],
}

#[derive(Copy, Clone, Debug)]
struct Block {
    bb: graphics::Rect,
    color: graphics::Color,
    occupied: bool,
}
enum PieceType {
    Piece1,
    Piece2,
    Piece3,
    Piece4,
    Piece5,
    Piece6,
    Piece7,
}

impl ggez::event::EventHandler for Tetris {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
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
            nowPiece: Piece::randomPiece(),
            backGround: Background::new(),
        }
    }

    fn draw(&mut self, mb: &mut graphics::MeshBuilder) -> GameResult {
        //self.nowPiece.draw(mb)?;
        self.backGround.draw(mb)?;
        Ok(())
    }
}

impl Background {
    pub fn new() -> Background {
        Background {
            record: [[Block::new(); TETRIS_WIDTH]; TETRIS_HEIGHT],
        }
    }
    fn draw(&mut self, mb: &mut graphics::MeshBuilder) -> GameResult {
        for i in 0..TETRIS_HEIGHT {
            for j in 0..TETRIS_WIDTH {
                self.record[i][j].draw(
                    mb,
                    Point2 {
                        x: (j * BLOCK_SIZE + 200) as f32,
                        y: (i * BLOCK_SIZE) as f32,
                    },
                )?;
            }
        }
        Ok(())
    }
}

impl Block {
    pub fn new() -> Block {
        Block {
            bb: graphics::Rect::new(0.0, 0.0, BLOCK_SIZE as f32, BLOCK_SIZE as f32),
            color: graphics::BLACK,
            occupied: false,
        }
    }
    fn draw(&mut self, mb: &mut graphics::MeshBuilder, p: Point2<f32>) -> GameResult {
        self.bb.move_to(p);
        mb.rectangle(graphics::DrawMode::fill(), self.bb, graphics::WHITE);

        Ok(())
    }
}

impl Piece {
    pub fn randomPiece() -> Piece {
        Piece {
            pieceType: PieceType::Piece1,
            x: 4,
            y: 0,
            turn: 0,
            color: graphics::BLACK,
        }
    }
    fn draw(&mut self, mb: &mut graphics::MeshBuilder) -> GameResult {
        Ok(())
    }
}
