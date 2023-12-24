struct S<T> {
    #[cfg(feature = "f")]
    t: T,
    phantom: std::marker::PhantomData<T>,
}

fn f<#[cfg(feature = "f")] T: std::fmt::Display>(#[cfg(feature = "f")] t: T) {
    #[cfg(feature = "f")]
    println!("{t}");

    #[cfg(not(feature = "f"))]
    println!("f");
}

impl<T> S<T> {
    fn m(&self) {
        println!("m");
    }
}

#[test]
fn test() {
    let s = S::<i32> {
        #[cfg(feature = "f")]
        t: 0,
        phantom: std::marker::PhantomData,
    };

    #[cfg(feature = "f")]
    f(s.t);

    #[cfg(not(feature = "f"))]
    f();

    s.m();
}
