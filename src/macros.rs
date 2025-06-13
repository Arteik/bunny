#[macro_export]
macro_rules! simple_bunny {
    (
        $modname:ident,
        $name:ident,
        aliases = [$($alias:expr),* $(,)?],
        hop = |$args:ident| $hop_block:block
    ) => {
        pub mod $modname {
            use crate::bunny::{BunnyAction, BunnyAlias, BunnyArgs, BunnyCommand};
            use warp;

            #[crate::linkme::distributed_slice(crate::COMMANDS)]
            #[linkme(crate = crate::linkme)]
            fn register() -> Box<dyn BunnyCommand> {
                Box::new($name::new())
            }

            #[derive(Debug)]
            struct $name;

            impl $name {
                const fn new() -> Self {
                    Self
                }
            }

            impl BunnyAlias for $name {
                fn aliases(&self) -> &'static [&'static str] {
                    &[$($alias),*]
                }
            }

            impl BunnyAction for $name {
                fn hop(&self, $args: BunnyArgs) -> Box<dyn warp::Reply> {
                    let uri: warp::http::Uri = $hop_block;
                    Box::new(warp::redirect::temporary(uri))
                }
            }
        }
    };
}