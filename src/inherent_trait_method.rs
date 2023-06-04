struct A {}

impl A {
    fn met(&self) {
        println!("inherent");
    }
}

trait T {
    fn met(&self);
}

impl T for A {
    fn met(&self) {
        println!("trait");
    }
}

#[test]
fn test() {
    let a = A {};
    a.met();
}
