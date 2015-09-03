extern crate foo_bar;

use foo_bar::foo::Foo;

fn main() {
    let foo = Foo::new(1);

    foo.bar.baz();
}
