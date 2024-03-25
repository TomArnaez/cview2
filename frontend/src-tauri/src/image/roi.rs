use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

#[enum_dispatch(PixelIterable)]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum ROI {
    Rect(Rect),
    Line(Line),
}

#[enum_dispatch]
pub trait PixelIterable {
    fn pixels(&self) -> Box<dyn Iterator<Item=Point>>;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Rect {
    pub width: u32,
    pub height: u32,
    pub pos: Point,
}

impl PixelIterable for Rect {
    fn pixels(&self) -> Box<dyn Iterator<Item=Point>> {
        let width = self.width;
        let height = self.height;
        let x_start = self.pos.x;
        let y_start = self.pos.y;

        Box::new((0..height).flat_map(move |y| {
            (0..width).map(move |x| Point { x: x + x_start, y: y + y_start })
        }))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Line {
    pub start: Point,
    pub finish: Point,
}

impl PixelIterable for Line {
    fn pixels(&self) -> Box<dyn Iterator<Item = Point>> {
        let dx = i32::abs((self.finish.x as i32) - (self.start.x as i32));
        let dy = -i32::abs((self.finish.y as i32) - (self.start.y as i32));
        let sx = if self.start.x < self.finish.x { 1 } else { -1 };
        let sy = if self.start.y < self.finish.y { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = self.start.x as i32;
        let mut y = self.start.y as i32;
        let end_x = self.finish.x as i32;
        let end_y = self.finish.y as i32;

        let iter = std::iter::from_fn(move || {
            if x == end_x && y == end_y {
                return None;
            }

            let current_point = Point {
                x: x as u32,
                y: y as u32,
            };

            let e2 = 2 * err;
            if e2 >= dy {
                if x == end_x {
                    return None;
                }
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                if y == end_y {
                    return None;
                }
                err += dx;
                y += sy;
            }

            Some(current_point)
        });

        Box::new(iter)
    }
}