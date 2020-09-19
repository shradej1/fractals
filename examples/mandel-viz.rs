use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use mandelbrot::{ImageBuffer, write_buffer};
use graphics::math::{identity, translate, scale};
use num::Complex;
use graphics::rectangle::{square, rectangle_by_corners};
use piston::{Event, CursorEvent, Input, Button, ButtonState, MouseCursorEvent, ButtonEvent, ButtonArgs, MouseButton};

pub struct App {
    /// OpenGL drawing backend
    gl: GlGraphics,
    buff: ImageBuffer,
    selection_start: Option<[f64; 2]>,
    selection_end: Option<([f64; 2])>,
    zoom_to: Option<[f64; 4]>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let image = Image::new().rect(rectangle_by_corners(0.0, 0.0, self.buff.rows as f64, self.buff.cols as f64));
        let texture_set = TextureSettings::new().convert_gamma(true);
        let im = Texture::from_memory_alpha(&self.buff.buf, self.buff.rows as u32, self.buff.cols as u32, &texture_set).unwrap();

        const GREEN_TRANS: types::Color = [0.0, 1.0, 0.0, 1.0];
        let (selection_start, selection_end) = (self.selection_start, self.selection_end);
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(types::Color::default(), gl);

            // translate();
            // scale();

            image.draw(&im, &c.draw_state, c.transform, gl);

            if let (Some(start), Some(end)) = (selection_start, selection_end) {
                dbg!(start, end);
                let rect = Rectangle::new_border(GREEN_TRANS, 2.0);
                let dims = rectangle_by_corners(start[0], start[1], end[0], end[1]);
                rect.draw(dims, &c.draw_state, c.transform, gl);
            }
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
        selection_start: None,
        selection_end: None,
        zoom_to: None,
    };

    let mut events = Events::new(EventSettings::new());
    let mut curr_mouse_pos = None;
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.mouse_cursor_args() {
            curr_mouse_pos = Some(args);
            if app.selection_start.is_some() {
                app.selection_end = curr_mouse_pos;
            }
        }

        if let Some(ButtonArgs { state, button, scancode }) = e.button_args() {
            if button == Button::Mouse(MouseButton::Left) {
                match state {
                    ButtonState::Press => { app.selection_start = curr_mouse_pos; }
                    ButtonState::Release => {
                        if let (Some(start), Some(end)) = (app.selection_start, app.selection_end) {
                            app.zoom_to = Some([start[0], start[1], end[0], end[1]]);
                            app.selection_start = None;
                            app.selection_end = None;
                        }
                    }
                }
            }
        }

        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}