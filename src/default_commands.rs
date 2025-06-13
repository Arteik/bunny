pub mod youtube {
    use crate::bunny::{BunnyAction, BunnyAlias, BunnyArgs, BunnyCommand};

    #[crate::linkme::distributed_slice(crate::COMMANDS)]
    #[linkme(crate = crate::linkme)]
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
        fn hop(&self, args: BunnyArgs) -> Box<dyn warp::Reply> {
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