use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use mandelbrot::{ImageBuffer, write_buffer, pixel_to_point};
use graphics::math::{identity, translate, scale};
use num::Complex;
use graphics::rectangle::{square, rectangle_by_corners};
use piston::{Event, CursorEvent, Input, Button, ButtonState, MouseCursorEvent, ButtonEvent, ButtonArgs, MouseButton, EventLoop, Key};

pub struct App {
    /// OpenGL drawing backend
    gl: GlGraphics,
    buff: ImageBuffer,
    selection_start: Option<[f64; 2]>,
    selection_end: Option<([f64; 2])>,
    zoom_to: Option<[f64; 4]>,
    image_size: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    init_upper_left: Complex<f64>,
    init_lower_right: Complex<f64>,
    reset: bool,
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

            // draw the image
            image.draw(&im, &c.draw_state, c.scale(1.1, 1.1).transform, gl);

            // draw the selection rectangle
            if let (Some(start), Some(end)) = (selection_start, selection_end) {
                let rect = Rectangle::new_border(GREEN_TRANS, 2.0);
                let dims = rectangle_by_corners(start[0], start[1], end[0], end[1]);
                rect.draw(dims, &c.draw_state, c.transform, gl);
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        if self.reset {
            self.upper_left = self.init_upper_left;
            self.lower_right = self.init_lower_right;
            self.selection_start = None;
            self.selection_end = None;
            self.zoom_to = None;
            self.buff = write_buffer(self.image_size, self.upper_left, self.lower_right);
            self.reset = false;
        }

        // zoom the image
        if let Some([a, b, c, d]) = self.zoom_to {
            let [x0, y0, w, h] = rectangle_by_corners(a, b, c, d);
            let upper_left = pixel_to_point(self.image_size, (x0 as usize, y0 as usize), self.upper_left, self.lower_right);
            let lower_right = pixel_to_point(self.image_size, ((x0 + w) as usize, (y0 + h) as usize), self.upper_left, self.lower_right);

            self.buff = write_buffer(self.image_size, upper_left, lower_right);
            self.upper_left = upper_left;
            self.lower_right = lower_right;
        }
        self.zoom_to = None;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    let image_size = (1500, 1125);

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("mandelbrot", [1500, 1125])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let upper_left = Complex::new(-1.20, 0.35);
    let lower_right = Complex::new(-1.0, 0.20);

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        image_size,
        upper_left,
        lower_right,
        init_upper_left: upper_left,
        init_lower_right: lower_right,
        buff: write_buffer(image_size, upper_left, lower_right),
        selection_start: None,
        selection_end: None,
        zoom_to: None,
        reset: false,
    };

    let mut events = Events::new(EventSettings::new());
    let mut curr_mouse_pos = None;
    let ratio = app.image_size.0 as f64 / app.image_size.1 as f64;

    while let Some(e) = events.next(&mut window) {
        if let Some([x1, y1]) = e.mouse_cursor_args() {
            curr_mouse_pos = Some([x1, y1]);

            if let Some([x0, y0]) = app.selection_start {
                app.selection_end = if (x1 - x0).abs() > (y1 - y0).abs() {
                    Some([x1, y0 + (x1 - x0) / ratio])
                } else {
                    Some([x0 + ratio * (y1 - y0), y1])
                };

                let [x1, y1] = app.selection_end.unwrap();
            }
        }

        if let Some(args) = e.button_args() {
            if args.button == Button::Keyboard(Key::R) && args.state == ButtonState::Press {
                app.reset = true;
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