#[macro_export]
macro_rules! spaced_name {
    ($first:ident $( $rest:ident )*) => {
        concat!(stringify!($first), $( " ", stringify!($rest) ),*)
    };
}

#[macro_export]
macro_rules! simple_bunny {
    (
        $($name:ident)+,
        aliases = [$($alias:expr),* $(,)?],
        hop = |$args:ident| $hop_block:block
    ) => {
        paste::paste! {
            pub mod [< $($name:snake)_* >] {
                use crate::bunny::{BunnyAction, BunnyAlias, BunnyArgs, BunnyCommand};
                use warp;

                #[crate::linkme::distributed_slice(crate::COMMANDS)]
                #[linkme(crate = crate::linkme)]
                fn register() -> Box<dyn BunnyCommand> {
                    Box::new([<$($name)+>]::new())
                }

                #[derive(Debug)]
                struct [<$($name)+>];

                impl [<$($name)+>] {
                    const fn new() -> Self {
                        Self
                    }
                }

                impl BunnyAlias for [<$($name)+>] {
                    fn name(&self) -> &'static str {
                        crate::spaced_name!($($name)+)
                    }

                    fn aliases(&self) -> &'static [&'static str] {
                        &[$($alias),*]
                    }
                }

                impl BunnyAction for [<$($name)+>] {
                    fn hop(&self, $args: BunnyArgs) -> Box<dyn warp::Reply> {
                        $hop_block
                    }
                }
            }
        }
    }
}

#[macro_export]
macro_rules! templated_bunny {
    (
        $($name:ident)+,
        aliases = [$($alias:expr),* $(,)?],
        uri = $uri_template:literal
    ) => {
        crate::simple_bunny! {
            $($name)+,
            aliases = [$($alias),*],
            hop = |args| {
                crate::utils::uri_to_redirect(format!(
                    $uri_template,
                    urlencoding::encode(&args.args)
                ))
            }
        }
    };
}
