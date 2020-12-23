use ggez::event;
use ggez::graphics;
use ggez::graphics::Rect;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
// use cpu::Cpu;
// use crate::Cpu;


pub struct Graphics{
    state: GraphicsState,
    ctx:  ggez::Context,
    event_loop:  ggez::event::EventsLoop
}

impl Graphics{
    pub fn new() -> Graphics {
        let cb = ggez::ContextBuilder::new("Julian", "Prueba");
        let (ctx, event_loop) = cb.build().expect("Failed to build Screen");
        let state = GraphicsState::new().expect("Failed to create the inner state of the graph");
        Graphics{
            state: state,
            ctx: ctx,
            event_loop: event_loop
        }
    }
    pub fn start_graphics(&mut self) {
        
        event::run(&mut self.ctx, &mut self.event_loop, &mut self.state).expect("Failed to initializate Graphics");
    }

}


struct GraphicsState {
    pos_x: f32,
    // cpu: Cpu
    v_ram: [[bool; 64]; 32],
}

impl GraphicsState {
    pub fn new() -> GameResult<GraphicsState> {
        let s = GraphicsState { pos_x: 0.0, v_ram: [[false; 64]; 32], };// 
        Ok(s)
    }
    
    
}

impl event::EventHandler for GraphicsState {
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

        graphics::draw(ctx, &pixel, (na::Point2::new(self.pos_x, 0.0),))?;
        graphics::present(ctx)?;
        Ok(())
    }
}

