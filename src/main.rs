extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

// Create our model structs
struct Object {
    x: f64, // meters, I guess?
    y: f64, // meters
    m: f64, // kilograms
    v: f64  // meters/second
}

impl Object {
    fn update_pos(&mut self, dt: f64) {
        self.x += self.v * dt;
        self.y += self.v * dt;
    }
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    objs: Vec<Object>
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const BLUE:  [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let circle = ellipse::circle(0.0, 0.0, 20.0);
        //let rotation = self.rotation;
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        let ref objs = self.objs;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            // draw objects
            for obj in objs {
                let transform = c.transform.trans(x, y) // move origin to center
                                           .trans(obj.x, obj.y);

                ellipse(BLUE, circle, transform, gl);
            }
            //let transform = c.transform.trans(x, y);
            //                           .rot_rad(rotation)
            //                           .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Move object by velocity
        for ref obj in &self.objs {
            //obj.update_pos(args.dt);
            //obj.x += obj.v * args.dt;
            //obj.y += obj.v * args.dt;
        }
        // Rotate 2 radians per second.
        //self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let window: Window = WindowSettings::new(
            "rustroids",
            //[800, 600]
            [200,200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create first object
    let obj1 = Object{ 
        x: 0.0,
        y: 0.0,
        m: 10.0,
        v: 1.0 
    };

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        objs: vec![ obj1 ]
    };

    for e in window.events() {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}