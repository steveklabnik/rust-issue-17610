use self::bar::Bar;

mod bar;

pub struct Foo {
    pub bar: Bar
}

impl Foo {
    pub fn new(x: i32) -> Foo {
        Foo { bar: Bar::new(x) }
    }
}
