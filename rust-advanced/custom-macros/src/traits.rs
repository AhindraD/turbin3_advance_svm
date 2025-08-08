use std::f64::consts::PI;

struct Rect {
    width: u32,
    height: u32,
}

struct Circle {
    radius: u32,
}

trait Shape {
    fn area(&self) -> u32;
    fn permeter(&self) -> u32;
}

impl Shape for Rect {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn permeter(&self) -> u32 {
        2 * (self.height + self.width)
    }
}

impl Shape for Circle {
    fn area(&self) -> u32 {
        PI as u32 * (self.radius * self.radius)
    }
    fn permeter(&self) -> u32 {
        PI as u32 * 2 * self.radius
    }
}

fn get_area_perimeter(s: impl Shape) -> (u32, u32) {
    let area = s.area();
    let peri = s.permeter();
    (area, peri)
}

fn main() {
    let rec1 = Rect {
        width: 2,
        height: 3,
    };

    let c1 = Circle { radius: 3 };
    println!("{:?}", get_area_perimeter(rec1));
    println!("{:?}", get_area_perimeter(c1));
}
