pub use enum_dispatch_iter_macro::EnumDispatchIter;

pub trait EnumDispatchIter: Sized {
    fn into_iter() -> std::vec::IntoIter<Self>;
}
