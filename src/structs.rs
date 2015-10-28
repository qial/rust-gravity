extern crate rand;

use self::rand::thread_rng;
use self::rand::Rng;
use self::rand::distributions::{IndependentSample, Range};

// Constants for equations
const CONST_G: f64 = 6.67408;

// Create our model structs
pub struct Vec2D {
    pub x: f64,
    pub y: f64
}

pub struct Object {
    pub p: Vec2D, // meters, I guess?
    pub v: Vec2D, // meters/second
    pub m: f64,   // kilograms
}

impl Object {
    pub fn new(x: f64, y: f64, dx: f64, dy: f64, m: f64) -> Object {
        let p = Vec2D{ x: x,  y: y  };
        let v = Vec2D{ x: dx, y: dy };
        Object{ p: p, v: v, m: m }
    }

    pub fn update_pos(&mut self, dt: f64) {
        self.p.x += self.v.x * dt;
        self.p.y += self.v.y * dt;
    }

    // a{x,y} is acceleration force direction (and magnitude)
    pub fn update_v(&mut self, dt: f64, ax: f64, ay: f64) {
        self.v.x += dt * ax;
        self.v.y += dt * ay;
    }
}

// calculates the (x,y) gravity force vector m1 feels from m2
// magnitude of vector is strength of force
fn gravity_force_2d(m1: Object, m2: Object) -> (f64, f64) {
    // calculate force between points
    let f = gravity_force(&m1,&m2);
    let (dx, dy) = unit_dir(m1.p, m2.p);
    ((dx * f) / m1.m, (dy * f) / m2.m )
}

// calculates the gravity force magnitued between m1 and m2
fn gravity_force(m1: &Object, m2: &Object) -> f64 {
    let r2 = dist(&m1.p, &m2.p).powi(2);
    CONST_G * ((m1.m * m2.m) / r2)
}

// calculates unit vector in direction p1->p2
fn unit_dir(p1: Vec2D, p2: Vec2D) -> (f64, f64) {
    unit_vector(dir(p1,p2))
}

// calculates unit vector in direction Vec2D
fn unit_vector(v: Vec2D) -> (f64, f64) {
    let m = magnitude(&v);
    (v.x / m, v.y / m)
}

// calculates distance vector in direction p1->p2
fn dir(p1: Vec2D, p2: Vec2D) -> Vec2D {
    Vec2D{x: p2.x-p1.x, y: p2.y-p1.y}
}

// calculates distance between points (as Vec2Ds)
fn dist(p1: &Vec2D, p2: &Vec2D) -> f64 {
    let x2 = (p1.x - p2.x).powi(2);
    let y2 = (p1.y - p2.y).powi(2);
    let d2: f64 = x2 + y2;
    d2.sqrt()
}

// calculates magnitude of a Vec2D
fn magnitude(v: &Vec2D) -> f64 {
    let m2: f64 = v.x.powi(2) + v.y.powi(2);
    m2.sqrt()
}

pub fn make_random_objects() -> Vec<Object> {
    // Create first object
    let maxv = 10.0f64;
    let maxx = 800.0f64;
    let maxy = 600.0f64;
    let mut objs = Vec::new();
    let mut rng = rand::thread_rng();
    let range = Range::new(-1.0 * maxv, maxv);

    for _ in 0..10 {
        objs.push(Object::new(
            rng.next_f64() * maxx, 
            rng.next_f64() * maxy,
            range.ind_sample(&mut rng),
            range.ind_sample(&mut rng),
            10.0
        ));
    }
    objs
}
