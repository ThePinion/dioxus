//! This example demonstrates the following:
//! Futures in a callback, Router, and Forms

use dioxus::prelude::*;

fn main() {
    launch_desktop(app);
}

fn app() -> Element {
    let onsubmit = move |evt: FormEvent| async move {
        let resp = reqwest::Client::new()
            .post("http://localhost:8080/login")
            .form(&[
                ("username", &evt.values()["username"]),
                ("password", &evt.values()["password"]),
            ])
            .send()
            .await;

        match resp {
            // Parse data from here, such as storing a response token
            Ok(_data) => println!("Login successful!"),

            //Handle any errors from the fetch here
            Err(_err) => {
                println!("Login failed - you need a login server running on localhost:8080.")
            }
        }
    };

    rsx! {
        h1 { "Login" }
        form { onsubmit,
            input { r#type: "text", id: "username", name: "username" }
            label { "Username" }
            br {}
            input { r#type: "password", id: "password", name: "password" }
            label { "Password" }
            br {}
            button { "Login" }
        }
    }
}
