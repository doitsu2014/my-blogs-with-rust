#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn NavBar() -> Element {
    rsx! {
        nav {
            class: "flex",
            Logo {}
            ul {
                li {
                    Link { to: Route::Home {}, "Home" }
                }
                li {
                    Link { to: Route::Blog { id: 123 }, "Blog" }
                }
            }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| String::from("..."));

    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        div {
            h1 { class: "text-green", "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
            button {
                onclick: move |_| async move {
                    if let Ok(data) = get_server_data().await {
                        log::info!("Client received: {}", data);
                        text.set(data.clone());
                        post_server_data(data).await.unwrap();
                    }
                },
                "Get Server Data"
            }
            p { "Server data: {text}"}
        }
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    println!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}

#[component]
fn Logo() -> Element {
    rsx! {
        div { class: "flex p-2",
            div { class: "logo bg-cyan-400 rounded-lg p-3 shadow-md",
                span { class: "font-bold text-xl text-white", "d" }
                span { class: "font-bold text-xl text-black", " tech" }
            }
        }
    }
}
