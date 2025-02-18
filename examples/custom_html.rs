//! This example shows how to use a custom index.html and custom <HEAD> extensions
//! to add things like stylesheets, scripts, and third-party JS libraries.

use dioxus::desktop::Config;
use dioxus::prelude::*;

fn main() {
    LaunchBuilder::desktop()
        .with_cfg(
            Config::new().with_custom_head("<style>body { background-color: red; }</style>".into()),
        )
        .launch(app);

    LaunchBuilder::desktop()
        .with_cfg(
            Config::new().with_custom_index(
                r#"
<!DOCTYPE html>
<html>
  <head>
    <title>Dioxus app</title>
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <style>body { background-color: blue; }</style>
  </head>
  <body>
    <div id="main"></div>
  </body>
</html>
        "#
                .into(),
            ),
        )
        .launch(app);
}

fn app() -> Element {
    rsx! {
        div { h1 { "hello world!" } }
    }
}
