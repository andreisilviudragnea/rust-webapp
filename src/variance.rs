trait A {}

trait B: A {}

struct C;

impl A for C {}

impl B for C {}

#[test]
fn test() {
    let c: Box<dyn B> = Box::new(C {});
    let _c: Box<dyn A> = c;
}
