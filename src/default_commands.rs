use crate::{simple_bunny, templated_bunny};

templated_bunny!(
    Google,
    aliases = ["google", "g"],
    uri = "https://www.google.com/search?q={}"
);

templated_bunny!(
    Youtube,
    aliases = ["youtube", "yt"],
    uri = "https://www.youtube.com/results?search_query={}"
);

templated_bunny!(
    Wikipedia,
    aliases = ["wiki", "w"],
    uri = "https://en.wikipedia.org/wiki/{}"
);

templated_bunny!(
    CratesIo,
    aliases = ["crate"],
    uri = "https://crates.io/search?q={}"
);

templated_bunny!(
    NixPkg,
    aliases = ["nix", "nixpkg"],
    uri = "https://search.nixos.org/packages?query={}"
);

templated_bunny!(
    Noogle,
    aliases = ["noog", "noogle"],
    uri = "https://noogle.dev/q?term={}"
);

templated_bunny!(
    DocsRs,
    aliases = ["rsdoc", "rsd"],
    uri = "https://docs.rs/releases/search?query={}"
);

simple_bunny!(
    Playground,
    aliases = ["play"],
    hop = |args| {
        let uri = match args.args.as_str() {
            "js" => "https://repljs.com/new",
            "cpp" => "https://cpp.sh/",
            "rs" | "rust" | _ => "https://play.rust-lang.org/",
        };
        crate::utils::uri_to_redirect(uri.to_string())
    }
);
