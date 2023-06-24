#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(length: u32) -> Self {
        Rectangle {
            width: length,
            height: length
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn compare(&self, compare: &Rectangle) -> bool {
        self.width > compare.width && self.height > compare.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    fn joinRectangle(&mut self, join: &Rectangle) {
        self.width += join.width;
        self.height += join.height;
    }
}

fn rectangle() {
    let mut rect = Rectangle{
        width: 10,
        height: 20
    };

    let rect2 = Rectangle {
        width: 40,
        height: 50
    };

    rect.joinRectangle(&rect2);

    println!("{:?}", rect);
}