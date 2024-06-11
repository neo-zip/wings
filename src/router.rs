pub const CUSTOM_ROUTES: &[(&'static str, &'static str)] = &[
    ("/", "/index"),
];

pub fn swap_to_custom_route(path: &str) -> &str {
    for (route, new_route) in CUSTOM_ROUTES {
        if path == *route {
            return new_route;
        }
    }

    return path;
}