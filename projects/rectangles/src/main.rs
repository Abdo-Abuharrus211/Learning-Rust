#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Technically not how fitting works, should check dimensions not area but whatever
    fn can_fit(&self, other: &Rectangle)->bool{
        self.area() > other.area()
    }
    // Example of Associated Function
    fn square(dim: u32)->Self{
        Self{
            width: dim,
            height: dim,
        }
    }
}

fn main() {
    let rect_one = Rectangle {
        width: 1080,
        height: 1920,
    };

    let rect_two = Rectangle {
        width: 143,
        height: 69,
    };
    let my_head = Rectangle::square(5);

    println!("The area of a rectangle is {} sqr pixels.", rect_one.area());
    println!("{:#?}", rect_one);
    println!("Can it fit? {}", rect_one.can_fit(&rect_two));
}

// fn area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }
