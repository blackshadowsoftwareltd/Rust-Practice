fn main() {
    let mut rect: Rectangle = Rectangle::new(10, 5);
    println!("old area : {:?}", rect.area());

    rect.update_width_height_with_ref(20, 10); //? wither ref
    println!("updated with ref : {:?}", rect.area());

    rect = rect.update_width_height(30, 15); //? without ref
    println!("updated without ref : {:?}", rect.area());

    rect.inc_width(5);
    println!("new area : {:?}", rect.area());
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn update_width_height_with_ref(&mut self, w: u32, h: u32) {
        self.width = w;
        self.height = h;
        // ? no need return if self is mut ref
    }
    fn update_width_height(mut self, w: u32, h: u32) -> Rectangle {
        self.width = w;
        self.height = h;
        self
    }
    fn inc_width(&mut self, delta: u32) {
        self.width += delta
    }
}
