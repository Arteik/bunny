use crate::templated_bunny;

templated_bunny!(
    Youtube,
    aliases = ["youtube", "yt"],
    uri = "https://www.youtube.com/results?search_query={}"
);