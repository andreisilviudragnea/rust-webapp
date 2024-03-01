use std::marker::PhantomData;
use std::mem::ManuallyDrop;

#[test]
fn test1() {
    struct S {
        a: String,
        b: String,
    }

    let mut s = ManuallyDrop::new(S {
        a: "a".to_string(),
        b: "b".to_string(),
    });

    let _a = &mut s.a;
    let b = &mut s.b;

    // *a = "a1".to_string();
    *b = "b1".to_string();
}

#[test]
fn test2() {
    struct S<'a> {
        a: &'a str,
        b: &'a str,
    }

    let a = String::from("a");
    let b = String::from("b");

    let mut s = S {
        a: a.as_str(),
        b: b.as_str(),
    };

    let a = &mut s.a;
    let b = &mut s.b;

    *a = "a1";
    *b = "b1";
}

#[test]
fn test3() {
    struct Person {
        name: String,
        addresses: Vec<String>,
    }

    let mut person = Person {
        name: "Alice".to_string(),
        addresses: vec!["123 Main St".to_string()],
    };

    let name = &person.name;
    person.addresses.push("456 Elm St".to_string());
    println!("The name is: {}", name);
}

#[test]
fn test4() {
    struct S {
        a: A,
        b: B<A>,
    }

    struct A;
    struct B<T> {
        _t: T,
    }

    let a = A {};
    let b = B { _t: A {} };

    let mut s = S { a, b };

    let _a = &mut s.a;
    let _b = &mut s.b;
}

#[test]
fn test5() {
    struct S<'a> {
        a: A<'a>,
        b: B<A<'a>>,
    }

    #[derive(Debug)]
    struct A<'a> {
        t: PhantomData<&'a ()>,
    }

    #[derive(Debug)]
    struct B<T> {
        t: PhantomData<T>,
    }

    let a = A { t: PhantomData };
    let b: B<A> = B { t: PhantomData };

    let mut s = S { a, b };

    let a = &mut s.a;
    let b = &mut s.b;

    *a = A { t: PhantomData };
    *b = B { t: PhantomData };

    println!("{:?} {:?}", a, b);
}

#[test]
fn test6() {
    struct S {
        a: A,
        b: B,
    }

    struct A;
    struct B;

    impl B {
        fn b(&mut self, _a: &mut A) {
            println!("here");
        }
    }

    let a = A {};
    let b = B {};

    let mut s = S { a, b };

    let _a = &mut s.a;
    let _b = &mut s.b;

    _b.b(_a);
}

#[test]
fn test7() {
    struct S<'a> {
        a: A<'a>,
        b: B<'a>,
    }

    struct A<'a> {
        _t: PhantomData<&'a ()>,
    }
    struct B<'a> {
        _t: PhantomData<&'a ()>,
    }

    impl B<'_> {
        fn b(&mut self, _a: &mut A) {
            println!("here");
        }
    }

    let a = A { _t: PhantomData };
    let b = B { _t: PhantomData };

    let mut s = S { a, b };

    let _a = &mut s.a;
    let _b = &mut s.b;

    _b.b(_a);
}

#[test]
fn test8() {
    struct Holder<'a, 'r> {
        machine: Machine<'r, State<'a, 'r>>,
        b: State<'a, 'r>,
    }

    struct State<'a, 'r> {
        _t: PhantomData<&'a ()>,
        _r: PhantomData<&'r ()>,
    }

    struct Machine<'r, B> {
        phantom: PhantomData<*const B>,
        _r: PhantomData<&'r ()>,
    }

    impl<B> Machine<'_, B> {
        fn execute(&mut self, _b: &mut B) {
            println!("here");
        }
    }

    let mut holder = Holder {
        machine: Machine {
            phantom: PhantomData,
            _r: PhantomData,
        },
        b: State {
            _t: PhantomData,
            _r: PhantomData,
        },
    };

    let machine = &mut holder.machine;
    let mut b = &mut holder.b;

    machine.execute(&mut b);
}
