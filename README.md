# Enum Dispatch Iterator

Derive macro for the [enum_dispatch crate](https://crates.io/crates/enum_dispatch) that enable to iterate over the variants.

Variants must all derive the Default trait.

## Example:

```rust
#[enum_dispatch]
trait MyTrait {
    fn trait_func(&self) -> i32;
}

#[derive(Default)]
struct Foo;
impl MyTrait for Foo {
    fn trait_func(&self) -> i32 {
        return 1;
    }
}

#[derive(Default)]
struct Bar;
impl MyTrait for Bar {
    fn trait_func(&self) -> i32 {
        return 2;
    }
}

#[enum_dispatch(MyTrait)]
#[derive(EnumDispatchIter)]
enum MyEnum {
    Foo,
    Bar,
}

fn main() {
    let sum = MyEnum::into_iter()
        .map(|variant| variant.trait_func())
        .reduce(|sum, val| sum + val);

    assert_eq!(sum, Some(3));
}
```

