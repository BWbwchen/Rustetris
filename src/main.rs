mod setting;
mod tetris;

use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{
    conf, event, event::EventHandler, graphics, Context, ContextBuilder, GameError, GameResult,
};

fn main() {
    // TODO
    let c = conf::Conf::new();
    let (mut ctx, event_loop) = ContextBuilder::new("Tetris Game", "BWbwchen")
        .default_conf(c)
        .build()
        .unwrap();

    let my_game = tetris::Tetris::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}

impl EventHandler<GameError> for tetris::Tetris {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while ctx.time.check_update_time(3) {
            self.update()?;
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        self.draw(ctx, &mut canvas)?;

        canvas.finish(ctx)?;
        Ok(())
    }
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeat: bool,
    ) -> Result<(), GameError> {
        match input.keycode {
            Some(KeyCode::Escape) => {
                ctx.request_quit();
                Ok(())
            }
            Some(KeyCode::Left) => {
                self.move_piece(-1, 0);
                Ok(())
            }
            Some(KeyCode::Right) => {
                self.move_piece(1, 0);
                Ok(())
            }
            Some(KeyCode::Down) => {
                self.move_piece(0, 1);
                Ok(())
            }
            Some(KeyCode::Up) => {
                self.turn();
                Ok(())
            }
            Some(KeyCode::Space) => {
                self.speed_drop();
                Ok(())
            }
            _ => {
                // do nothing
                Ok(())
            }
        }
    }
}
