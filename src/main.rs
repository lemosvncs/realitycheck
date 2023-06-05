use leptos::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! {cx,
        <section class="bg-black relative text-white">
            <div>
                <p class="text-white">"Oi"</p>
            </div>
        </section>
    }
} 

fn main() {
    mount_to_body(|cx| view! { cx, <App/>})
}
