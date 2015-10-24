extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use rand::thread_rng;
use rand::Rng;


// Create our model structs
struct Vec2D {
    x: f64,
    y: f64
}

struct Object {
    p: Vec2D, // meters, I guess?
    v: Vec2D, // meters/second
    m: f64,   // kilograms
}

impl Object {
    fn new(x: f64, y: f64, dx: f64, dy: f64, m: f64) -> Object {
        let p = Vec2D{ x: x,  y: y  };
        let v = Vec2D{ x: dx, y: dy };
        Object{ p: p, v: v, m: m }
    }

    fn update_pos(&mut self, dt: f64) {
        self.p.x += self.v.x * dt;
        self.p.y += self.v.y * dt;
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
                let transform = c.transform//.trans(x, y) // move origin to center
                                           .trans(obj.p.x, obj.p.y);

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
        let mut i = 0;
        while i < self.objs.len() {
            self.objs[i].update_pos(args.dt);
            i+=1;
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
            [800, 600]
            //[200,200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create first object
    let maxv = 10.0f64;
    let maxx = 800.0f64;
    let maxy = 600.0f64;
    let mut objs = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        objs.push(Object::new(
            rng.next_f64() * maxx, 
            rng.next_f64() * maxy,
            rng.next_f64() * maxv,
            rng.next_f64() * maxv,
            10.0
        ));
    }

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        objs: objs
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