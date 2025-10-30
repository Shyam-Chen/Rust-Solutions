use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button
            class="px-4 py-2 font-semibold text-white bg-blue-500 rounded-lg shadow-lg hover:bg-blue-600"
            on:click=move |_| set_count.set(count.get() + 1)
        >
            "Click Me"
        </button>
        <p>"Current count: "{count}</p>
        <p>"Double count: "{move || count.get() * 2}</p>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> });
}
