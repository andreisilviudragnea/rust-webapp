struct A {
    b: B,
}

impl A {
    fn use_c(&self, c: &mut C) {
        c.mutate();
        self.b.use_c(c);
    }
}

struct B {}

impl B {
    fn use_c(&self, c: &mut C) {
        c.mutate();
    }
}

#[derive(Default)]
struct C {
    count: u32,
}

impl C {
    fn mutate(&mut self) {
        println!("Mutate: {}", self.count);
        self.count += 1;
    }
}

#[test]
fn test() {
    let mut c = C::default();
    let a = A { b: B {} };

    a.use_c(&mut c);
    a.b.use_c(&mut c);
}
