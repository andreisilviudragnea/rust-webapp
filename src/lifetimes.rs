type FnWithLifetimes<'a> = fn(a: &'a str, b: &'a str, c: &str);
type FnWithExplicitLifetimes<'a> = for<'b> fn(a: &'a str, b: &'a str, c: &'b str);
type FnWithExplicitLifetimes2 = for<'a> fn(a: &'a str, b: &'a str, c: &'a str);

const S: &str = "hello";

fn accepts_fn_with_lifetimes<'a>(f: FnWithLifetimes<'a>) {
    let s = "hello";
    let string = String::from("a");
    f(s, S, &string);
}

fn accepts_fn_with_explicit_lifetimes<'a>(f: FnWithExplicitLifetimes<'a>) {
    let s = "hello";
    let string = String::from("a");
    f(s, S, &string);
}

fn accepts_fn_with_explicit_lifetimes2(f: FnWithExplicitLifetimes2) {
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
    accepts_fn_with_explicit_lifetimes(function_with_lifetimes);
    accepts_fn_with_explicit_lifetimes2(function_with_lifetimes);
}
