use std::{collections::HashMap, sync::Arc};
use std::fmt::Debug;

use bunny::{BunnyArgs, BunnyFunction};
use linkme::distributed_slice;
use serde::{Deserialize, Serialize};
use warp::{reply::Reply, Filter};

pub mod bunny {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone)]
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

    pub trait BunnyAlias {
        fn aliases(&self) -> &'static [&'static str];
    }
    
    pub type BunnyFunction = Box<dyn Fn(BunnyArgs) -> Box<dyn Reply + 'static> + Send + Sync>;
    pub trait BunnyAction {
        fn execute(&self, args: bunny::BunnyArgs) -> Box<dyn warp::Reply>;
    }
    
    pub trait BunnyCommand: BunnyAlias + BunnyAction {}

    impl<T: BunnyAlias + BunnyAction + Send + Sync + Debug> BunnyCommand for T {}
}

pub fn build_command_map(
) -> HashMap<
    &'static str,
    BunnyFunction
> {
    COMMANDS
        .iter()
        .flat_map(|cmd| {
            let aliases = cmd().aliases();
            aliases.iter().map(move |&alias| {
                let handler: BunnyFunction
                    = Box::new(move |args: BunnyArgs| {
                        cmd().execute(args)
                    });
                (alias, handler)
            })
        })
        .collect()
}

#[derive(Serialize, Deserialize, Debug)]
struct RawQuery {
    /// Raw query string; ?@decorator cmd ?[..args]
    q: String,
}

#[distributed_slice]
pub static COMMANDS: [fn() -> Box<dyn crate::bunny::BunnyCommand>];

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let log = warp::log("rustybunny::main");

    let command_map = Arc::new(build_command_map());

    let with_map = warp::any()
        .map({
            let map_clone  = command_map.clone();
            move || map_clone.clone()
        });

    let example1 = warp::get()
        .and(warp::path("bunny"))
        .and(warp::query::<RawQuery>())
        .and(with_map) 
        .map(move |p: RawQuery, cmd_map: Arc<HashMap<&str, BunnyFunction>>| {
            let args = BunnyArgs::from(p);

            if let Some(execute_fn) = cmd_map.get(args.cmd.as_str()) {
                execute_fn(args)
            } else {
                Box::new(String::from("Can't find this command!"))
            }
        });

    let hello_world = warp::get()
        .and(warp::path::end())
        .map(|| "Hello, World at root!")
        .with(log);

    let routes = hello_world
        .or(example1);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

mod youtube {
    use linkme::distributed_slice;

    use crate::bunny::{BunnyAction, BunnyAlias, BunnyArgs, BunnyCommand};

    #[distributed_slice(crate::COMMANDS)]
    fn register() -> Box<dyn BunnyCommand> {
        Box::new(Youtube::new())
    }

    #[derive(Debug)]
    struct Youtube;

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
        fn execute(&self, args: BunnyArgs) -> Box<dyn warp::Reply> {
            let uri = warp::http::Uri::builder()
                .scheme("https")
                .authority("youtube.com")
                .path_and_query(format!("/results?search_query={}", args.args))
                .build()
                .unwrap();
            Box::new(warp::redirect::temporary(uri))
        }
    }

}