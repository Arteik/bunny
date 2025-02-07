use std::collections::HashMap;

use linkme::distributed_slice;
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Serialize, Deserialize, Debug)]
struct RawQuery {
    /// Raw query string; ?@decorator cmd ?[..args]
    q: String,
}

pub mod bunny {
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct BunnyArgs {
        /// Modifies BunnyAction behavior; enables targeting
        pub decorator: String,
        /// Resolved to a concrete BunnyAction
        pub cmd: String,
        /// Arguments provided to the given BunnyAction
        pub args: String,
    }

    impl From<RawQuery> for BunnyArgs {
        fn from(raw: RawQuery) -> Self {
            let mut query_split = raw.q.split_whitespace();

            let mut decorator = String::new();
            let mut cmd = String::new();

            if let Some(first) = query_split.next() {
                if first.starts_with('@') {
                    decorator = first[1..].to_string();
                    if let Some(second) = query_split.next() {
                        cmd = second.to_string();
                    }
                } else {
                    cmd = first.to_string();
                }
            }

            let args = query_split.collect::<Vec<&str>>().join(" ");

            BunnyArgs {
                decorator,
                cmd,
                args,
            }
        }
    }

    // pub trait BunnyAction {
    //     fn execute(args: &BunnyArgs) -> Box<dyn warp::Reply>;
    // }

    // pub trait SimpleRedirectBunnyAction: BunnyAction {
    //     fn build_uri(args: &BunnyArgs) -> warp::http::Uri;
    // }

    // impl<T: SimpleRedirectBunnyAction> BunnyAction for T {
    //     fn execute(args: &BunnyArgs) -> Box<dyn warp::Reply> {
    //         Box::new(warp::redirect(Self::build_uri(args)))
    //     }
    // }

    // pub trait TemplatedRedirectBunnyAction: SimpleRedirectBunnyAction {
    //     const TEMPLATE: &'static str = "{}";
    // }

    // impl<T: TemplatedRedirectBunnyAction> SimpleRedirectBunnyAction for T {
    //     fn build_uri(args: &BunnyArgs) -> warp::http::Uri {
    //         let x = T::TEMPLATE.format(&[args.args.clone()]);
    //         warp::http::Uri::from_str(&x).expect("foo")
    //     }
    // }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let log = warp::log("rustybunny::main");

    let example1 = warp::get()
        .and(warp::path("bunny"))
        .and(warp::query::<RawQuery>())
        .map(|p: RawQuery| p.q);

    let hello_world = warp::get()
        .and(warp::path::end())
        .map(|| "Hello, World at root!")
        .with(log);

    let routes = hello_world.or(example1);
    // .or(example1);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

trait BunnyAlias {
    fn aliases(&self) -> &'static [&'static str];
}

trait BunnyAction {
    fn execute(&self, args: bunny::BunnyArgs) -> Box<dyn warp::Reply>;
}

trait BunnyCommand: BunnyAlias + BunnyAction {}
 
impl<T: BunnyAlias + BunnyAction + Send + Sync> BunnyCommand for T {}

#[distributed_slice]
pub static COMMANDS: [Box<dyn BunnyCommand>];

struct Youtube;

static FOO: Box<dyn BunnyCommand + Send + Sync> = Box::new(Youtube::new());

impl Youtube {
    const fn new() -> Self {
        Self
    }
}

impl BunnyAlias for Youtube {
    fn aliases(&self) -> &'static [&'static str] {
        &["youtube", "yt"]
    }
}

impl BunnyAction for Youtube {
    fn execute(&self, args: bunny::BunnyArgs) -> Box<dyn warp::Reply> {
        Box::new(format!("www.youtube.com/results?search_query={}", args.args))
    }
}

// trait BunnyAlias {
//     fn aliases() -> &'static [&'static str];
// }

// trait BunnyAction {
//     fn execute(args: bunny::BunnyArgs) -> impl warp::Reply;
// }

// trait BunnyCommand: BunnyAlias + BunnyAction {
//     fn register<F>(
//         map: &mut HashMap<&'static str, F>,
//     ) where F: Fn(bunny::BunnyArgs) -> Box<dyn warp::Reply> + 'static;
// }

// impl<T: BunnyAlias + BunnyAction> BunnyCommand for T {
//     fn register<F>(
//         map: &mut HashMap<&'static str, F>,
//     ) where F: Fn(bunny::BunnyArgs) -> Box<dyn warp::Reply> + 'static {
//         for alias in T::aliases() {
//             let x = |args| Box::new(T::execute(args));
//             map.insert(*alias, x);
//         }
//     }
// }

// // #[distributed_slice]
// // pub static COMMANDS: [Fn(
// //     &mut HashMap<&'static str, Box<dyn Fn(bunny::BunnyArgs) -> Box<dyn warp::Reply>>>,
// // )];

// struct Youtube;

// impl BunnyAlias for Youtube {
//     fn aliases() -> &'static [&'static str] {
//         &["youtube", "yt"]
//     }
// }

// impl BunnyAction for Youtube {
//     fn execute(args: bunny::BunnyArgs) -> impl warp::Reply {
//         format!("www.youtube.com/results?search_query={}", args.args)
//     }
// }

// struct Youtube;
// impl bunny::TemplatedRedirectBunnyAction for Youtube {
//     const TEMPLATE: &'static str = "https://www.youtube.com/results?search_query={}";
// }

// // Define the macro
// macro_rules! define_commands {
//     ($($cmd_name:ident => $closure:expr),*) => {
//         // Define the trait
//         trait BunnyCommand {
//             fn execute(&self, args: BunnyArgs);
//         }

//         // Define the structs and their implementations
//         $(
//             struct $cmd_name;

//             impl BunnyCommand for $cmd_name {
//                 fn execute(&self, args: BunnyArgs) {
//                     $closure(args);
//                 }
//             }
//         )*

//         // Define the enum
//         enum BunnyCommands {
//             $($cmd_name),*
//         }

//         // Implement the Command trait for the enum
//         impl BunnyCommand for BunnyCommands {
//             fn execute(&self, args: BunnyArgs) {
//                 match self {
//                     $(
//                         BunnyCommands::$cmd_name => cmd.execute(args),
//                     )*
//                 }
//             }
//         }
//     };
// }

// // Use the macro to define the commands and the enum
// define_commands!(
//     Youtube => |_args: BunnyArgs| todo!(),
// );

// // Define the macro
// macro_rules! define_commands {
//     ($($cmd_name:ident => $closure:expr),*) => {
//         // Define the trait
//         trait BunnyCommand {
//             fn build_action(&self, args: BunnyArgs);
//         }

//         // Define the structs and their implementations
//         $(
//             struct $cmd_name;

//             impl BunnyCommand for $cmd_name {
//                 fn build_action(&self, _args: BunnyArgs) {
//                     $closure;
//                 }
//             }
//         )*

//         // Define the enum
//         enum BunnyCommands {
//             $($cmd_name($cmd_name)),*
//         }

//         // Implement the Command trait for the enum
//         impl BunnyCommand for BunnyCommands {
//             fn build_action(&self, args: BunnyArgs) {
//                 match self {
//                     $(
//                         BunnyCommands::$cmd_name(cmd) => cmd.build_action(args),
//                     )*
//                 }
//             }
//         }
//     };
// }

// // Use the macro to define the commands and the enum
// define_commands!(
//     Youtube => {
//         |args: BunnyArgs| Response::builder().body(format!(
//             "decorator = {}, cmd = {}, args = {:#?}",
//             args.decorator, args.cmd, args.args
//         ));
//     }
// );

// struct TemplatedRedirectBunnyAction {
//     url: 'static
// }

// enum BunnyCommands {
//     Youtube(TemplatedRedirectBunnyAction)

// }
