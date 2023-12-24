trait A {
    fn m(&self);
}

trait B: A {}

struct C;

impl A for C {
    fn m(&self) {
        println!("m");
    }
}

impl B for C {}

#[test]
fn test() {
    let c: Box<dyn B> = Box::new(C {});
    let c: Box<dyn A> = c;
    c.m();
}
