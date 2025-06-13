use crate::simple_bunny;

simple_bunny!(
    youtube,
    Youtube,
    aliases = ["youtube", "yt"],
    hop = |args| {
        warp::http::Uri::builder()
            .scheme("https")
            .authority("youtube.com")
            .path_and_query(format!("/results?search_query={}", args.args))
            .build()
            .unwrap()
    }
);