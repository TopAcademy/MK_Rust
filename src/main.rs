
use mktop::Point;

fn main()
{
    let p1 = Point::empty();
    let p2 = Point::from_xy(10, 33);
    let p3 = Point::from_copy(&p2);
    
    p1.show();
    p2.show();
    p3.show();
}
