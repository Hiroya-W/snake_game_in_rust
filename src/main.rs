use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

struct App {
    gl: GlGraphics,
    snake: Snake,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics;

        let GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(GREEN, gl);
        });

        self.snake.render(&mut self.gl, args);
    }

    fn update(&mut self) {
        self.snake.update();
    }
}

struct Snake {
    pos_x: i32,
    pos_y: i32,
    dir: Direction,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics;

        let RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square =
            graphics::rectangle::square((self.pos_x * 20) as f64, (self.pos_y * 20) as f64, 20_f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self) {
        match self.dir {
            Direction::Left => self.pos_x -= 1,
            Direction::Right => self.pos_x += 1,
            Direction::Up => self.pos_y -= 1,
            Direction::Down => self.pos_y += 1,
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("Snake Game", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            pos_x: 0,
            pos_y: 0,
            dir: Direction::Right,
        },
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
        if let Some(args) = e.update_args() {
            app.update();
        }
    }
}
