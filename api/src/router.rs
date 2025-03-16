pub enum Route {
    Search(&str),
    Summary(&str),
}

pub fn get_route(uri: &str) -> Option<Route> {
    match uri.split_once("?") {
        Some(("search", query)) => Some(Route::Search(query)),
        Some(("summary", query)) => Some(Route::Summary(query)),
        _ => None,
    }
}
