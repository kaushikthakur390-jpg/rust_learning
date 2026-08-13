use std::fmt::Debug;

#[derive(Debug, Clone)]
struct Foo{
    n1: u8,
    n2: i32
}

impl Foo{
    fn bar(n2: i32) -> Foo{
        Foo { n1: 69, n2 }
    }

    fn harsh(&self, mutt: u8){
        println!("Hi");
    }

    fn joy(&mut self, mutt: u8){
        self.n1 += 1;
        println!("Hiii")
    }

    fn kosik(mut self, mutt: u8){
        self.n1 += 1;
        println!("ahnduiawd")
    }
}

impl FooMethods for Foo {
    fn bar(n: u8) -> Foo {
        Foo { n1: n, n2: 69 }
    }
}

trait FooMethods{
    fn bar(n1: u8) -> Foo;
}

fn main() {
    let mut foo = Foo{
        n1: 8,
        n2: 12
    };

    // METHOD
    foo.harsh(8);
    foo.joy(8);
    // foo.kosik(8);
    println!("{foo:?}");

    //Static
    Foo::bar(8);

    let x = "Hi";
    let mut y = String::from("Hi");

    y.push('H');
}

