use std::f64;

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Neg;


#[derive(Clone, PartialEq)]
pub enum Unit {
    None,
    Percent(f64),
    Pixel(f64),
    Calc(String),
}

impl Unit {
    pub fn to_string(&self) -> String {
        match self {
            &Unit::None => format!(""),
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

#[macro_export]
macro_rules! N {
    () => ( Unit::None );
}

impl Add<Unit> for Unit {
    type Output = Unit;

    fn add(self, u: Unit) -> Unit {
        match (self, u) {
            ( Unit::None, _) => Unit::None,
            ( _, Unit::None ) => Unit::None,
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
            ( Unit::None, _) => Unit::None,
            ( _, Unit::None ) => Unit::None,
            ( Unit::Percent(x), Unit::Percent(y) ) => Unit::Percent(x - y),
            ( Unit::Percent(x), Unit::Pixel(y) ) => Unit::Calc(format!("{}% - {}px", x, y)),
            ( Unit::Pixel(x),  Unit::Pixel(y) ) => Unit::Pixel(x - y),
            ( Unit::Pixel(x),  Unit::Percent(y) ) => Unit::Calc(format!("{}px - {}%", x, y)),
            ( Unit::Calc(x), Unit::Calc(y) ) =>  Unit::Calc(format!("{} - {}", x, y)),
            ( Unit::Calc(x), Unit::Percent(y) ) => Unit::Calc(format!("{} - {}%", x, y)),
            ( Unit::Calc(x), Unit::Pixel(y) ) => Unit::Calc(format!("{} - {}px", x, y)),
            ( Unit::Percent(x), Unit::Calc(y) ) => Unit::Calc(format!("{}% - {}", x, y)),
            ( Unit::Pixel(x), Unit::Calc(y) ) => Unit::Calc(format!("{}px - {}", x, y)),
        }
    }
}

impl Div<f64> for Unit {
    type Output = Unit;

    fn div(self, v: f64) -> Unit {
        match self {
            Unit::None => Unit::None,
            Unit::Percent(x) => Unit::Percent(x / v),
            Unit::Pixel(x) => Unit::Pixel(x / v),
            Unit::Calc(x) => Unit::Calc(format!("({}) / {}", x, v))
        }
    }
}


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
}


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
