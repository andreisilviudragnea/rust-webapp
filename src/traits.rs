trait T {
    fn m(&self);
    fn n(&self);
}

trait S {
    fn m(&self);
    fn n(&self);
}

struct A {}

impl A {
    fn m(&self) {
        println!("m");
    }
}

impl T for A {
    fn m(&self) {
        println!("t");
        self.m();
    }

    fn n(&self) {
        println!("tn");
    }
}

impl S for A {
    fn m(&self) {
        println!("s");
        self.m();
    }

    fn n(&self) {
        println!("sn");
    }
}

struct Foo {}

trait Bar {
    fn bar(&self);
}

impl Foo {
    #[allow(dead_code)]
    fn bar(&mut self) {
        println!("In struct impl!")
    }
}

impl Bar for Foo {
    fn bar(&self) {
        println!("In trait impl!")
    }
}

#[test]
fn test() {
    let a = A {};
    a.m();

    println!("1");

    let t = &a as &dyn T;
    t.m();

    T::m(&a);

    println!("2");

    let s = &a as &dyn S;
    s.m();

    S::m(&a);

    println!("3");

    #[allow(unused_mut)]
    let mut f = Foo {};
    f.bar();

    println!("4");

    // a.n();
}
