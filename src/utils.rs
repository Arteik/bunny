pub fn uri_to_redirect(s: String) -> Box<dyn warp::Reply> {
    let uri: warp::http::Uri = s
        .parse()
        .unwrap();
    Box::new(warp::redirect::temporary(uri))
}