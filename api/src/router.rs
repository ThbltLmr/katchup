use crate::request_parser::Uri;

pub enum Route {
    Search(String),
    Summary(String),
}

pub fn get_route(uri: Uri) -> Option<Route> {
    match uri.path.as_str() {
        "search" => Some(Route::Search(uri.query.unwrap_or(String::from("")))),
        "summary" => Some(Route::Summary(uri.query.unwrap_or(String::from("")))),
        _ => None,
    }
}
