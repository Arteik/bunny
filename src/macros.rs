#[macro_export]
macro_rules! simple_bunny {
    (
        $name:ident,
        aliases = [$($alias:expr),* $(,)?],
        hop = |$args:ident| $hop_block:block
    ) => {
        paste::paste! {
            pub mod [ <$name:lower> ] {
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
        $name:ident,
        aliases = [$($alias:expr),* $(,)?],
        uri = $uri_template:literal
    ) => {
        crate::simple_bunny! {
            $name,
            aliases = [$($alias),*],
            hop = |args| {
                let uri: warp::http::Uri = format!(
                    $uri_template,
                    &args.args
                )
                    .parse()
                    .unwrap();
                Box::new(warp::redirect::temporary(uri))
            }

        }
    };
}