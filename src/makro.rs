#[macro_export]
macro_rules! define_io_ctx {
    ($(#[$m:meta])? struct $name:ident { $($rename:ident => $field:ident : $t:ty $(,)?)* }) => {
        $(#[$m])?
        #[repr(C)]
        #[allow(dead_code)]
        pub struct $name {
            $(pub $field : $t),*
        }

        impl $crate::io_ctx::IOContext for $name {
            fn metadata() -> $crate::io_ctx::IOContextMetadata {
                vec![
                    $((
                        stringify!($rename).to_string(),
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
                g_a => a: i32,
                g_b => b: i32,
                g_c => c: f32,
                g_d => d: i32,
                g_e => e: f32
            }
        );

        eprintln!("{:?}", <S as IOContext>::metadata());
    }
}
