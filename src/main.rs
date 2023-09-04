use leptos::*;

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
        <p>
        <h1 class=("red", move || count() % 2 == 1)>Hello Leptos Counter!</h1>
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
        <ProgressBar progress=count/>
        </p>
    }
}

#[component]
fn ProgressBar(cx: Scope, progress: ReadSignal<i32>) -> impl IntoView {
    let double_count = move || progress() * 2;

    view! { cx,
        <div style:padding-top="20px">
            <progress
                max="100"
                // signals are functions, so this <=> `move || count.get()`
                value=double_count
            />
            <span style:padding-left="20px">
                {double_count}
            </span>
        </div>
    }
}
