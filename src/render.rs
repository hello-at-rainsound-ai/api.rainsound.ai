pub fn is_running_on_render() -> bool {
    if let Ok(value) = std::env::var("RENDER") {
        value == "true"
    } else {
        false
    }
}
