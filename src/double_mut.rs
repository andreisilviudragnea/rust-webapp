struct Inner {}

impl Inner {
    fn m(&mut self) {
        println!("hello");
    }
}

struct Outer<'a> {
    inner: &'a mut Inner,
}

impl Outer<'_> {
    fn n(&mut self) {
        println!("world");
    }
}

#[test]
fn test() {
    let mut inner = Inner {};
    let mut outer = Outer { inner: &mut inner };
    outer.inner.m(); // instead of inner.m();
    outer.n();
}
