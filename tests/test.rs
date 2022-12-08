use enum_dispatch_iter::EnumDispatchIter;

use enum_dispatch::enum_dispatch;

#[enum_dispatch]
trait MyTrait {
    fn trait_func(&self) -> i32;
}

macro_rules! test_init {
    () => {
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
    };
}

#[test]
fn iter_variants() -> Result<(), Box<dyn std::error::Error>> {
    test_init!();

    let sum = MyEnum::into_iter()
        .map(|variant| variant.trait_func())
        .reduce(|sum, val| sum + val);

    assert_eq!(sum, Some(3));

    Ok(())
}
