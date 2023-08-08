#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
trait MyTrait {
    fn foo(&self, x: u32) -> u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn call_with_four(x: &impl MyTrait) -> u32 {
        x.foo(4)
    }

    // fn call_with_four2(x: &impl MyTrait) -> u32 {
    //     x.foo(4)
    // }

    #[test]
    fn mytest() {
        let mut mock = MockMyTrait::new();
        mock.expect_foo().with(eq(4)).times(1).returning(|x| x + 1);
        assert_eq!(5, call_with_four(&mock));
    }
}
