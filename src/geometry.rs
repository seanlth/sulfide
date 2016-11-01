use std::f64;

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Neg;


#[derive(Clone, PartialEq)]
pub enum Unit {
    Percent(f64),
    Pixel(f64),
    Calc(String),
}

impl Unit {
    pub fn to_string(&self) -> String {
        match self {
            &Unit::Percent(v) => format!("{}%", v),
            &Unit::Pixel(v) => format!("{}px", v),
            &Unit::Calc(ref v) => format!("calc({})", v)
        }
    }
}

#[macro_export]
macro_rules! Pr {
    ($e:expr) => ( Unit::Percent($e));
}

#[macro_export]
macro_rules! Px {
    ($e:expr) => ( Unit::Pixel($e));
}

#[macro_export]
macro_rules! Clc {
    ($e:expr) => ( Unit::Calc($e));
}

impl Add<Unit> for Unit {
    type Output = Unit;

    fn add(self, u: Unit) -> Unit {
        match (self, u) {
            ( Unit::Percent(x), Unit::Percent(y) ) => Unit::Percent(x + y),
            ( Unit::Percent(x), Unit::Pixel(y) ) => Unit::Calc(format!("{}% + {}px", x, y)),
            ( Unit::Pixel(x),  Unit::Pixel(y) ) => Unit::Pixel(x + y),
            ( Unit::Pixel(x),  Unit::Percent(y) ) => Unit::Calc(format!("{}px + {}%", x, y)),
            ( Unit::Calc(x), Unit::Calc(y) ) =>  Unit::Calc(format!("{} + {}", x, y)),
            ( Unit::Calc(x), Unit::Percent(y) ) => Unit::Calc(format!("{} + {}%", x, y)),
            ( Unit::Calc(x), Unit::Pixel(y) ) => Unit::Calc(format!("{} + {}px", x, y)),
            ( Unit::Percent(x), Unit::Calc(y) ) => Unit::Calc(format!("{}% + {}", x, y)),
            ( Unit::Pixel(x), Unit::Calc(y) ) => Unit::Calc(format!("{}px + {}", x, y)),
        }
    }
}

impl Sub<Unit> for Unit {
    type Output = Unit;

    fn sub(self, u: Unit) -> Unit {
        match (self, u) {
            ( Unit::Percent(x), Unit::Percent(y) ) => Unit::Percent(x - y),
            ( Unit::Percent(x), Unit::Pixel(y) ) => Unit::Calc(format!("{}% - {}px", x, y)),
            ( Unit::Pixel(x),  Unit::Pixel(y) ) => Unit::Pixel(x - y),
            ( Unit::Pixel(x),  Unit::Percent(y) ) => Unit::Calc(format!("{}px - {}%", x, y)),
            ( Unit::Calc(x), Unit::Calc(y) ) =>  Unit::Calc(format!("{} - {}", x, y)),
            ( Unit::Calc(x), Unit::Percent(y) ) => Unit::Calc(format!("{} - {}%", x, y)),
            ( Unit::Calc(x), Unit::Pixel(y) ) => Unit::Calc(format!("{} - {}px", x, y)),
            ( Unit::Percent(x), Unit::Calc(y) ) => Unit::Calc(format!("{}% - {}", x, y)),
            ( Unit::Pixel(x), Unit::Calc(y) ) => Unit::Calc(format!("{}px - {}", x, y)),
        }    }
}

// impl Mul<f64> for Point {
//     type Output = Point;
//
//     fn mul(self, v: f64) -> Point {
//         Point::new(self.x * v, self.y * v)
//     }
// }
//
// impl Mul<Point> for f64 {
//     type Output = Point;
//
//     fn mul(self, v: Point) -> Point {
//         Point::new(self * v.x, self * v.y)
//     }
// }
//
// impl Div<f64> for Point {
//     type Output = Point;
//
//     fn div(self, v: f64) -> Point {
//         Point::new(self.x / v, self.y / v)
//     }
// }
//
// impl Div<Point> for Point {
//     type Output = Point;
//
//     fn div(self, v: Point) -> Point {
//         Point::new(self.x / v.x, self.y / v.y)
//     }
// }
//
// impl Neg for Point {
//     type Output = Point;
//
//     fn neg(self) -> Point {
//         Point::new(-self.x, -self.y)
//     }
// }



// pub struct asd <f64> {
//     pub x: f64,
//     pub y: f64
// }
//
//
// impl<f64> asd<f64> {
//     pub fn new() -> asd<f64> {
//         asd {
//             x: 1.0,
//             y: 1.0
//         }
//     }
// }

#[derive(Clone, PartialEq)]
pub struct Point {
    pub x: Unit,
    pub y: Unit
}

impl Point {
    pub fn new(x: Unit, y: Unit) -> Point {
        Point {
            x: x,
            y: y
        }
    }

    // pub fn print(&self) {
    //     let x = match self.x {
    //         Unit::Percent(v) => format!("{}%", v),
    //         Unit::Pixel(v) => format!("{}px", v)
    //     };
    //     let y = match self.y {
    //         Unit::Percent(v) => format!("{}%", v),
    //         Unit::Pixel(v) => format!("{}px", v)
    //     };
    //     println!("(x: {}, y: {})", x, y);
    // }
}

// impl Add<Point> for Point {
//     type Output = Point;
//
//     fn add(self, v: Point) -> Point {
//         Point::new(self.x + v.x, self.y + v.y)
//     }
// }
//
// impl Sub<Point> for Point {
//     type Output = Point;
//
//     fn sub(self, v: Point) -> Point {
//         Point::new(self.x - v.x, self.y - v.y)
//     }
// }
//
// impl Mul<f64> for Point {
//     type Output = Point;
//
//     fn mul(self, v: f64) -> Point {
//         Point::new(self.x * v, self.y * v)
//     }
// }
//
// impl Mul<Point> for f64 {
//     type Output = Point;
//
//     fn mul(self, v: Point) -> Point {
//         Point::new(self * v.x, self * v.y)
//     }
// }
//
// impl Div<f64> for Point {
//     type Output = Point;
//
//     fn div(self, v: f64) -> Point {
//         Point::new(self.x / v, self.y / v)
//     }
// }
//
// impl Div<Point> for Point {
//     type Output = Point;
//
//     fn div(self, v: Point) -> Point {
//         Point::new(self.x / v.x, self.y / v.y)
//     }
// }
//
// impl Neg for Point {
//     type Output = Point;
//
//     fn neg(self) -> Point {
//         Point::new(-self.x, -self.y)
//     }
// }
//
// pub const ZERO_POINT: Point = Point { x: 0.0, y: 0.0 };
// pub const MAX_POINT: Point = Point { x: f64::MAX, y: f64::MAX };


#[derive(Clone)]
pub struct Rect {
    pub width: Unit,
    pub height: Unit,
    pub position: Point
}

impl Rect {
    pub fn new(width: Unit, height: Unit, position: Point) -> Rect {
        Rect {
            width: width,
            height: height,
            position: position
        }
    }
}
