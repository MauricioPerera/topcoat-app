use topcoat::{
    Result,
    context::Cx,
    router::{Router, RouterBuilderDiscoverExt, page, path_param},
    view::{component, view},
};
use topcoat_app::greeting::format_greeting;

#[tokio::main]
async fn main() {
    topcoat::start(Router::builder().discover().build()).await.unwrap();
}

#[path_param]
struct Name(str);

#[page("/greet/{name}")]
async fn greet_page(cx: &Cx) -> Result {
    let name = path_param::<Name>(cx);
    let greeting = format_greeting(name);
    view! {
        <!DOCTYPE html>
        <html>
            <head>
                <title>"Greet"</title>
                topcoat::dev::script()
            </head>
            <body>
                <h1>(greeting)</h1>
            </body>
        </html>
    }
}

#[page("/")]
async fn home() -> Result {
    view! {
        <!DOCTYPE html>
        <html>
            <head>
                <title>"Hello world"</title>
                topcoat::dev::script()
            </head>
            <body>
                hello(name: "World")
            </body>
        </html>
    }
}

#[component]
async fn hello(name: &str) -> Result {
    view! {
        <h1>"Hello, " (name) "!"</h1>
    }
}
