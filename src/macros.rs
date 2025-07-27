#[macro_export]
macro_rules! flatten_modules {
    ($($file:ident),+ $(,)?) => {
        $(
            mod $file;
            #[allow(unused_imports)]
            pub use $file::*;
        )*
    };
}
