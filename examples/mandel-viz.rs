use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use mandelbrot::{ImageBuffer, write_buffer};
use graphics::math::identity;
use num::Complex;
use graphics::rectangle::{square, rectangle_by_corners};

pub struct App {
    /// OpenGL drawing backend
    gl: GlGraphics,
    buff: ImageBuffer,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let image = Image::new().rect(rectangle_by_corners(0.0, 0.0, self.buff.rows as f64, self.buff.cols as f64));
        let texture_set = TextureSettings::new().convert_gamma(true);
        let im = Texture::from_memory_alpha(&self.buff.buf, self.buff.rows as u32, self.buff.cols as u32, &texture_set).unwrap();

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(types::Color::default(), gl);

            image.draw(&im, &c.draw_state, c.transform, gl);
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {}
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    // let mut window: Window = WindowSettings::new("mandelbrot", [2560, 1600])
    let mut window: Window = WindowSettings::new("mandelbrot", [2000, 1500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let image_size = (2000, 1500);

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        buff: write_buffer(image_size, Complex::new(-1.20, 0.35), Complex::new(-1.0, 0.20)),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            // app.update(&args);
        }
    }
}