use std::ops::Deref;

struct Foo {}

impl Foo {
    fn hello(&self) {
        println!("helo, world!");
    }
}

struct Bar {}

impl Deref for Bar {
    type Target = Foo;

    fn deref(&self) -> &Foo {
        &Foo {}        
    }
}

fn main() {
    let f = Foo{};
    f.hello();

    let b = Bar{};
    b.hello();
}
