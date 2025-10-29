use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <html>
            <head>
                <title>My Leptos App</title>
            </head>
            <body>
                <p>"Hello, Leptos!"</p>
            </body>
        </html>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> });
}
