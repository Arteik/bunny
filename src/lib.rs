use std::fmt::Debug;
use std::{collections::HashMap, sync::Arc};

use bunny::{BunnyArgs, BunnyFunction};
use linkme::distributed_slice;
use serde::{Deserialize, Serialize};
use warp::Filter;

pub use linkme;

use crate::table::generate_bunny_table;

mod default_commands;
mod macros;
mod utils;
mod table;

pub mod bunny {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, Default)]
    pub struct BunnyArgs {
        /// Modifies BunnyAction behavior; enables targeting
        pub decorator: String,
        /// Resolved to a concrete BunnyAction
        pub cmd: String, // TODO: This should be a regex to support ENG-XXX
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

    pub trait BunnyAlias {
        fn name(&self) -> &'static str;
        fn aliases(&self) -> &'static [&'static str];
    }

    pub type BunnyFunction = Box<dyn Fn(BunnyArgs) -> Box<dyn warp::Reply + 'static> + Send + Sync>;
    pub trait BunnyAction {
        fn hop(&self, args: bunny::BunnyArgs) -> Box<dyn warp::Reply>;
    }

    pub trait BunnyCommand: BunnyAlias + BunnyAction {}

    impl<T: BunnyAlias + BunnyAction + Send + Sync + Debug> BunnyCommand for T {}
}

pub fn build_command_map() -> HashMap<&'static str, BunnyFunction> {
    COMMANDS
        .iter()
        .flat_map(|cmd| {
            let aliases = cmd().aliases();
            aliases.iter().map(move |&alias| {
                let handler: BunnyFunction = Box::new(move |args: BunnyArgs| cmd().hop(args));
                (alias, handler)
            })
        })
        .collect()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RawQuery {
    /// Raw query string; ?@decorator cmd ?[..args]
    q: String,
}

#[distributed_slice]
pub static COMMANDS: [fn() -> Box<dyn crate::bunny::BunnyCommand>];

pub async fn serve_bunny() {
    pretty_env_logger::init();
    let log = warp::log("rustybunny");

    let command_map = Arc::new(build_command_map());
    println!("Commands: {:?}", build_command_map().keys());

    let with_map = warp::any().map({
        let map_clone = command_map.clone();
        move || map_clone.clone()
    });

    let bunny_router = warp::get()
        .and(warp::path("bunny"))
        .and(warp::query::<RawQuery>())
        .and(with_map)
        .map(
            move |p: RawQuery, cmd_map: Arc<HashMap<&str, BunnyFunction>>| {
                let args = BunnyArgs::from(p.clone());

                if let Some(hop_fn) = cmd_map.get(args.cmd.as_str()) {
                    hop_fn(args)
                } else {
                    let mut google_bargs = BunnyArgs::default();
                    google_bargs.args = p.q;
                    (cmd_map.get("google").unwrap())(google_bargs)
                }
            },
        );

    // This should be a static index of the available commands and their aliases. I'd also
    // like some  representation of the hop function but maybe it is extra effort
    let hello_world = warp::get()
        .and(warp::path("bunny"))
        .and(warp::path::end())
        .map(move || warp::reply::html(generate_bunny_table()))
        .with(log); 

    let routes = bunny_router.or(hello_world);

    println!("Serving at http://localhost:1234");

    warp::serve(routes).run(([127, 0, 0, 1], 1234)).await
}
