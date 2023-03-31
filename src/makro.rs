#[macro_export]
macro_rules! define_io_ctx {
    ($(#[$m:meta])? struct $name:ident { $($field:ident : $t:ty $(,)?)* }) => {
        $(#[$m])?
        #[allow(dead_code)]
        pub struct $name {
            pub $($field : $t),*
        }

        impl $crate::io_ctx::IOContext for $name {
            fn metadata() -> Vec<(String, $crate::io_ctx::Type21)> {
                vec![
                    $((
                        stringify!($field).to_string(),
                        <$crate::Void as $crate::io_ctx::Reflektor<$t>>::reflected_type()
                    ),)*
                ]
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::io_ctx::IOContext;

    #[test] fn test() {
        define_io_ctx!(
            struct S {
                a: i32,
                b: i32,
                c: f32,
                d: i32,
                e: f32
            }
        );

        eprintln!("{:?}", <S as IOContext>::metadata());
    }
}
