use ggez::event;
use ggez::graphics;
use ggez::graphics::Rect;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

pub mod cpu;
use cpu::Cpu;



pub struct Chip8 {
    // pos_x: f32,
    cpu: Cpu
}

impl Chip8 {
    pub fn new(cpu:Cpu) -> GameResult<Chip8> {
        let s = Chip8 { cpu: cpu };//pos_x: 0.0, 
        Ok(s)
    }
    pub fn main_loop(cpu:Cpu) -> GameResult {
        let cb = ggez::ContextBuilder::new("Julian", "Prueba");
        let (ctx, event_loop) = &mut cb.build()?;
        let state = &mut Chip8::new(cpu)?;
        event::run(ctx, event_loop, state)
    }
}

impl event::EventHandler for Chip8 {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        //self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let pixel = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), Rect {
            x: 0.0,
            y: 0.0,
            w: 100.0,
            h: 100.0,
        }, graphics::WHITE)?;

        graphics::draw(ctx, &pixel, (na::Point2::new(0.0, 0.0),))?;
        graphics::present(ctx)?;
        Ok(())
    }
}

