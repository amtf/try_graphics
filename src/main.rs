use opengl_graphics::{OpenGL, GlGraphics};
use piston_window::{PistonWindow, WindowSettings, RenderArgs, UpdateArgs};
use piston::Events;
use piston_window::EventSettings;
use piston_window::UpdateEvent;
use piston_window::RenderEvent;
use piston_window::Graphics;

fn main() {
    let opengl = OpenGL::V4_5;
    let mut window: PistonWindow = WindowSettings::new("title", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let gl = GlGraphics::new(opengl);

    let mut app = App { gl };
    run_loop(&mut app, &mut window);    
}

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

struct App {
    pub(crate) gl: GlGraphics,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |_c, g| {
            g.clear_color(BLACK);
            g.clear_color(WHITE);
        });
    }

    fn update(&mut self, _args: &UpdateArgs) { 
    }
}

fn run_loop(app: &mut App, w: &mut PistonWindow) {
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(w) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

