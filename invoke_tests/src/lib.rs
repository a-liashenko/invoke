mod test_base;
mod test_generics;

struct Test;

#[invoke::invoke]
impl Test {
    #[invoke_fn]
    fn test(&self) {}
}
