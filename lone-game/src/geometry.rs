//pub mod rect;
use num_traits::int::PrimInt;
use num_traits::Float;

pub struct Point<T:PrimInt> {
    pub x: T,
    pub y: T
}

impl From<Point<i32>> for sdl2::rect::Point {
    fn from(other: Point<i32>) -> sdl2::rect::Point {
        sdl2::rect::Point::new(other.x, other.y)
    }
}
/* 
impl Into<Point<i32>> for sdl2::rect::Point {
    fn into(self) -> Point<i32> {
       Point<i32>{x: self.x, y: self.y}
    }
}*/

impl From<sdl2::rect::Point> for Point<i32> {
    fn from(sdlPoint: sdl2::rect::Point) -> Point<i32> {
        let result = Point {x: sdlPoint.x, y: sdlPoint.y };
        return result;
    }
}


pub struct Size<T:PrimInt> {
    pub width: T,
    pub height: T
}

pub struct Rect<T:PrimInt> {
    pub origin: Point<T>,
    pub size: Size<T>
}

pub struct PointF<T:Float> {
    pub x: T,
    pub y: T
}

pub struct SizeF<T:Float> {
    pub width: T,
    pub height: T
}

pub struct RectF<T:Float> {
    pub origin: PointF<T>,
    pub size: SizeF<T>
}

#[allow(dead_code)]
impl<T:PrimInt> Rect<T> {
    fn x(&self) -> T {
        self.origin.x
    }

    fn set_x(&mut self, value:T) {
        self.origin.x = value
    }

    fn y(&self) -> T {
        self.origin.y
    }

    fn set_y(&mut self, value:T) {
        self.origin.y = value
    }

    fn width(&self) -> T {
        self.size.width
    }

    fn set_width(&mut self, value:T) {
        self.size.width = value
    }

    fn height(&self) -> T {
        self.size.height
    }

    fn set_height(&mut self, value:T) {
        self.size.height = value
    }
}