struct Rect {
    width: i32,
    height: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn destruct(self: Self) -> Self {
        self
    }
}

fn main() {
    let rect1: Rect = Rect {
        width: 10,
        height: 20,
    };

    // rect1.destruct();

    println!("The area is: {}", rect1.area());
}
