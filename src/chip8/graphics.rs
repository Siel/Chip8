use ggez::event;
use ggez::graphics;
use ggez::graphics::Rect;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

use std::sync::mpsc;
// use cpu::Cpu;
// use crate::Cpu;


pub struct Graphics{
    state: GraphicsState,
    ctx:  ggez::Context,
    event_loop:  ggez::event::EventsLoop
}

impl Graphics{
    pub fn new(rx: mpsc::Receiver<[[bool; 64]; 32]>) -> Graphics {
        let mut cb = ggez::ContextBuilder::new("Chip8", "Julian Otalvaro");
        cb = cb.window_setup(ggez::conf::WindowSetup{
            title: "Chip8 - Emulator by @siel".to_owned(),
            samples: ggez::conf::NumSamples::Zero,
            vsync: true,
            icon: "".to_owned(),
            srgb: true,
        });
        cb = cb.window_mode(ggez::conf::WindowMode{
            width: 1260.0,
            height: 620.0,
            maximized: false,
            fullscreen_type: ggez::conf::FullscreenType::Windowed,
            borderless: false,
            min_width: 0.0,
            max_width: 0.0,
            min_height: 0.0,
            max_height: 0.0,
            resizable: false,
        });
        let (ctx, event_loop) = cb.build().expect("Failed to build Screen");
        let state = GraphicsState::new(rx).expect("Failed to create the inner state of the graph");
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
    vram: [[bool; 64]; 32],
    rx: mpsc::Receiver<[[bool; 64]; 32]>
}

impl GraphicsState {
    pub fn new(rx: mpsc::Receiver<[[bool; 64]; 32]>) -> GameResult<GraphicsState> {
        let s = GraphicsState { pos_x: 0.0, vram: [[false; 64]; 32], rx: rx };// 
        Ok(s)
    }
    
    
}

impl event::EventHandler for GraphicsState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        //self.pos_x = self.pos_x % 800.0 + 1.0;
        self.vram = self.rx.recv().unwrap();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let w_pixel = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), Rect {
            x: 0.0,
            y: 0.0,
            w: 20.0,
            h: 20.0,
        }, graphics::WHITE)?;
        let b_pixel = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), Rect {
            x: 0.0,
            y: 0.0,
            w: 20.0,
            h: 20.0,
        }, graphics::BLACK)?;

        for i in 0..31 {
            for j in 0..63{
                let pixel = if self.vram[i][j] {w_pixel.clone()} else {b_pixel.clone()};
                graphics::draw(ctx, &pixel, (na::Point2::new((j as f32)*20.0 ,(i as f32)*20.0),))?;
            }
            
        }

        
        graphics::present(ctx)?;
        Ok(())
    }
}

