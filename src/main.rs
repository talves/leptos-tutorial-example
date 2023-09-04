use leptos::*;

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
        <p>
        <h1>Hello Leptos Counter!</h1>
        <button
            on:click=move |_| {
                // let mut counter = count.get();
                // counter += 1;
                // set_count.set(counter);
                // Signals are Copy and 'static
                set_count.update(|n| *n += 1)
            }
        >
            "Click me: "
            {count}
        </button>
        </p>
    }
}
