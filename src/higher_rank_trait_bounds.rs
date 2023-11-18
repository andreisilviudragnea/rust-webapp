struct Closure<F> {
    data: (u8, u16),
    func: F,
}

impl<F> Closure<F>
where
    F: for<'a> Fn(&'a (u8, u16)) -> &'a u8,
{
    fn call(&self) -> &u8 {
        (self.func)(&self.data)
    }
}

fn do_it(data: &(u8, u16)) -> &u8 {
    &data.0
}

#[test]
fn test() {
    let clo = Closure {
        data: (0, 1),
        func: do_it,
    };
    println!("{}", clo.call());
}
