
pub struct Point {
    x: i32,
    y: i32
}


impl Point
{
    // Pseudo default constructor
    pub fn empty() -> Self {
        Point { x: 0, y: 0 }
    }

    // Constructor from XY
    pub fn from_xy(new_x:i32, new_y:i32) -> Self {
        Point { x: new_x, y: new_y }
    }

    // Pseudo copy-constructor
    pub fn from_copy(p2: &Self) -> Self {
        Point { x: p2.x, y: p2.y }
    }

    // Print point XY on console
    pub fn show(&self) {
        println!("X = {}; Y = {}", self.x, self.y);
    }
}
