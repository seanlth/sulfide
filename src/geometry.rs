use std::f64;

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Neg;


#[derive(Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point {
            x: x,
            y: y
        }
    }

    pub fn normalise(&self) -> Point {
        let d = ( (self.x * self.x) + (self.y * self.y) ).sqrt();
        Point::new(self.x/d, self.y/d)
    }

    pub fn normal(&self) -> Point {
        Point::new( -self.y , self.x )
    }

    pub fn magnitude(&self) -> f64 {
        ( (self.x * self.x) + (self.y * self.y) ).sqrt()
    }

    pub fn distance(&self, v: Point) -> f64 {
        ( ( (self.x - v.x) * (self.x - v.x)) + ((self.y - v.y) * (self.y - v.y) ) ).sqrt()
    }

    pub fn dot(&self, v: Point) -> f64 {
        self.x * v.x + self.y * v.y
    }

    pub fn cross(&self, v: Point) -> f64 {
        self.x * v.y - self.y * v.x
    }

    pub fn print(&self) {
        println!("(x: {}, y: {})", self.x, self.y);
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, v: Point) -> Point {
        Point::new(self.x + v.x, self.y + v.y)
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, v: Point) -> Point {
        Point::new(self.x - v.x, self.y - v.y)
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, v: f64) -> Point {
        Point::new(self.x * v, self.y * v)
    }
}

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, v: Point) -> Point {
        Point::new(self * v.x, self * v.y)
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, v: f64) -> Point {
        Point::new(self.x / v, self.y / v)
    }
}

impl Div<Point> for Point {
    type Output = Point;

    fn div(self, v: Point) -> Point {
        Point::new(self.x / v.x, self.y / v.y)
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point::new(-self.x, -self.y)
    }
}

pub const ZERO_POINT: Point = Point { x: 0.0, y: 0.0 };
pub const MAX_POINT: Point = Point { x: f64::MAX, y: f64::MAX };


#[derive(Copy, Clone)]
pub struct Rect {
    pub width: f64,
    pub height: f64,
    pub position: Point
}

impl Rect {
    pub fn new(width: f64, height: f64, position: Point) -> Rect {
        Rect {
            width: width,
            height: height,
            position: position
        }
    }
}
