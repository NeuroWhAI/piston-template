use {
    piston::{
        window::WindowSettings,
        event_loop::*,
        input::*,
    },
    glutin_window::GlutinWindow as Window,
    opengl_graphics::{
        GlGraphics,
        OpenGL,
    },
};


pub struct App {
    gl: GlGraphics,
    // TODO: Some state of this app.
}

impl App {
    fn new(gl: GlGraphics) -> Self {
        App {
            gl,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(color::BLACK, gl);

            // TODO: Draw something.
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // TODO: Update some state.
    }
}


fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "hello-piston",
        [800, 600])
        .graphics_api(opengl)
        .resizable(false)
        .build()
        .unwrap();

    let mut app = App::new(GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
