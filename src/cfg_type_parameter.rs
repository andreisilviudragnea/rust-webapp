struct S<#[cfg(feature = "f")] T> {
    #[cfg(feature = "f")]
    t: T,
}

fn f<#[cfg(feature = "f")] T: std::fmt::Display>(#[cfg(feature = "f")] t: T) {
    #[cfg(feature = "f")]
    println!("{t}");

    #[cfg(not(feature = "f"))]
    println!("f");
}

#[cfg(feature = "f")]
macro_rules! s_type {
    [$t:ident] => {
        S<$t>
    }
}

#[cfg(not(feature = "f"))]
macro_rules! s_type {
    [$t:ident] => {
        S
    }
}

impl<#[cfg(feature = "f")] T> s_type![T] {
    fn m(&self) {
        println!("m");
    }
}

#[test]
fn test() {
    let s = S {
        #[cfg(feature = "f")]
        t: 0,
    };

    #[cfg(feature = "f")]
    f(s.t);

    #[cfg(not(feature = "f"))]
    f();

    s.m();
}
