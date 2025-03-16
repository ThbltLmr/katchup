pub enum Route {
    Search(&str),
    Summary(&str),
}

pub fn get_route(uri: &str) -> Option<Route> {}
