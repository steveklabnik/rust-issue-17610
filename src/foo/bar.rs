pub struct Bar {
    x: i32
}

impl Bar {
    pub fn new(x: i32) -> Bar {
        Bar { x: x }
    }

    pub fn baz(&self) {
        println!("{}", self.x);
    }
}
