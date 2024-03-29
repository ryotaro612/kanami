use gdk4::Display;
use gtk4::CssProvider;
use std::env;
use std::path::Path;

pub(crate) fn load_css(option: &Option<String>) {
    let provider = CssProvider::new();
    match option {
        Some(actual) => provider.load_from_path(actual),
        None => {
            let home = env::var("HOME").expect("HOME was not set.");
            let user_path = Path::new(&home).join(".config/kanami/style.css");
            let path = if user_path.exists() {
                Some(user_path)
            } else {
                let default = Path::new("/etc").join("kanami/style.css");
                if default.exists() {
                    Some(default)
                } else {
                    None
                }
            };
            if let Some(a) = path {
                provider.load_from_path(a);
            }
        }
    };

    gtk4::style_context_add_provider_for_display(
        &Display::default().expect("Failed to connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
