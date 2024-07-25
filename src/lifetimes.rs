type FnWithLifetimes<'a> = fn(a: &'a str, b: &'a str, c: &str);

const S: &str = "hello";

fn accepts_fn_with_lifetimes<'a>(f: FnWithLifetimes<'a>) {
    let s = "hello";
    let string = String::from("a");
    f(s, S, &string);
}

fn function_with_lifetimes(a: &str, b: &str, c: &str) {
    println!("{a} {b} {c}");
}

#[test]
fn test() {
    accepts_fn_with_lifetimes(function_with_lifetimes);
}
