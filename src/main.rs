extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::num;

mod structs;

use structs::{Object, Vec2D};

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
        
        i = 0;
        while i < self.objs.len() {

        }
    }

    fn update_v( objs: &mut Vec<Object>, i: i32) {
        // find acceleration force for objs[i]
        //let mass = objs[i].m;
        // TODO gravity equations
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

    let objs = structs::make_random_objects();

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