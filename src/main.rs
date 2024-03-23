use std::collections::VecDeque;
use std::time::Instant;
use glutin_window::GlutinWindow as Window;
use input::KeyState;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{PressEvent, ReleaseEvent};
use piston_window::TextureSettings;
use tetris_board::BoardState;

use crate::utils::{draw_block, draw_frame, draw_tetromino};

mod tetris_board;
mod utils;
mod input;

pub struct Game<'a> {
    gl: GlGraphics,
    font: GlyphCache<'a>,
    fps: VecDeque<f32>,
    tps: VecDeque<f32>,
    board: BoardState,
    key_states: Vec<KeyState>,
    ticks_since_gravity: u8,
    pause: bool,
    blinking_rows: Vec<u8>,
    blinking_ticks: u8,
    blinking_visibility: bool
}

impl<'a> Game<'a> {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let translate_x = 640.0 / 2.0 - 10.0 * self.board.width as f64;
        let translate_y = 480.0 / 2.0 - 10.0 * self.board.height as f64;

        self.gl.draw(args.viewport(), |c, gl| {
            clear([0.0, 0.0, 0.0, 1.0], gl);

            // Draw board frame
            draw_frame(
                translate_x,
                translate_y,
                self.board.width as f64 * 20.0,
                self.board.height as f64 * 20.0,
                [0.2, 0.2, 0.2, 1.0],
                [0.1, 0.1, 0.1, 1.0],
                10.0,
                c,
                gl,
            );

            // Draw next tetromino frame
            draw_frame(
                translate_x / 2.0 - 50.0,
                190.0,
                100.0,
                100.0,
                [0.2, 0.2, 0.2, 1.0],
                [0.1, 0.1, 0.1, 1.0],
                10.0,
                c,
                gl,
            );

            // Draw next tetromino
            draw_tetromino(translate_x / 2.0 - 40.0, 200.0, self.board.next_tetromino.shape[self.board.next_tetromino.rotation as usize], self.board.next_tetromino.color, c, gl);

            println!("{:?}", self.blinking_rows);

            // Draw locked blocks
            let mut y = 0;
            for row in &self.board.locked_state {
                if self.blinking_rows.contains(&y) && !self.blinking_visibility {
                    y+=1;
                    continue;
                }
                let mut x = 0;
                for block in row {
                    draw_block(
                        x as f64 * 20.0 + translate_x,
                        y as f64 * 20.0 + translate_y,
                        block.color,
                        c,
                        gl
                    );
                    x += 1;
                }
                y += 1;
            }

            if !self.pause {
                // Draw falling tetromino
                draw_tetromino(
                    self.board.falling_state.x as f64 * 20.0 + translate_x,
                    self.board.falling_state.y as f64 * 20.0 + translate_y,
                    self.board.falling_state.shape[self.board.falling_state.rotation as usize],
                    self.board.falling_state.color,
                    c,
                    gl,
                );
            }

            // Draw fps and tps in the corner
            graphics::text(
                [1.0, 1.0, 1.0, 1.0],
                25,
                average(&self.fps).to_string().as_str(),
                &mut self.font,
                c.transform.trans(0.0, 30.0),
                gl,
            )
            .unwrap();
            graphics::text(
                [1.0, 1.0, 1.0, 1.0],
                25,
                average(&self.tps).to_string().as_str(),
                &mut self.font,
                c.transform.trans(0.0, 60.0),
                gl,
            )
            .unwrap();
        });
    }
}

fn average(numbers: &VecDeque<f32>) -> f32 {
    let mut sum: f32 = 0.0;
    for n in numbers {
        sum += n;
    }
    sum / numbers.len() as f32
}

fn main() {
    let opengl = OpenGL::V4_5;

    let mut window: Window = WindowSettings::new("tetris", [640, 480])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .vsync(true)
        .resizable(false)
        .build()
        .unwrap();

    let font = GlyphCache::new("assets/NotoSans-Regular.ttf", (), TextureSettings::new())
        .unwrap_or_else(|e| panic!("Failed to load font: {}", e));

    let mut app = Game {
        gl: GlGraphics::new(opengl),
        font,
        fps: VecDeque::new(),
        tps: VecDeque::new(),
        board: BoardState::new(),
        key_states: vec![],
        ticks_since_gravity: 0,
        pause: false,
        blinking_rows: vec![],
        blinking_ticks: 0,
        blinking_visibility: false
    };

    let mut last_render = Instant::now();
    let mut last_tick = Instant::now();

    let mut events = Events::new(EventSettings {
        max_fps: 400,
        ups: 32,
        ups_reset: 2,
        swap_buffers: true,
        bench_mode: false,
        lazy: false,
    });

    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.render_args() {
            app.render(&args);

            let fps = 1000.0 / (last_render.elapsed().as_micros() as f32 / 1000.0);
            last_render = Instant::now();

            app.fps.push_back(fps);
            if app.fps.len() > 120 {
                app.fps.pop_front();
            }
        }

        if let Some(args) = event.update_args() {
            // println!("{}", args.dt);
            if !app.pause {
                app.tick_input();

                if app.ticks_since_gravity > 16 {
                    app.ticks_since_gravity = 0;
                    app.board.gravity();
                } else {
                    app.ticks_since_gravity += 1;
                }
            }

            println!("{}", app.blinking_ticks);
            app.blinking_ticks += 1;

            if app.board.blinking_rows_queue.len() > 0 {
                app.pause = true;

                app.blinking_ticks = 0;

                app.blinking_rows = app.board.blinking_rows_queue;
                app.board.blinking_rows_queue = vec![];
            }

            if app.blinking_ticks % 4 == 0 {
                app.blinking_visibility = !app.blinking_visibility;
            }

            if app.blinking_ticks == 32 {
                app.blinking_ticks = 0;
                app.blinking_rows = vec![];
                if app.pause {
                    app.board.gravity();
                }
                app.pause = false;
            }

            let tps = 1000.0 / (last_tick.elapsed().as_micros() as f32 / 1000.0);
            last_tick = Instant::now();

            app.tps.push_back(tps);
            if app.tps.len() > 120 {
                app.tps.pop_front();
            }
        }

        if let Some(args) = event.press_args() {
            app.handle_input(args, input::EventType::Press);
        }

        if let Some(args) = event.release_args() {
            app.handle_input(args, input::EventType::Release);
        }
    }
}
