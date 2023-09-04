use leptos::*;

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let double_count = move || count() * 2;

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
        <ProgressBar progress=count max=100/>
        // Signal::derive creates a Signal wrapper from our derived signal
        // using double_count means it should move twice as fast
        <ProgressBar max=50 progress=Signal::derive(cx, double_count)/>
        </p>
    }
}

// Shows a progress bar using your max (optional) and progress
#[component]
fn ProgressBar(
    cx: Scope,
    /// How much progress should be displayed.
    #[prop(into)]
    // `Signal<T>` is a wrapper for several reactive types.
    // It can be helpful in component APIs like this, where we
    // might want to take any kind of reactive value
    progress: Signal<i32>,
    #[prop(default = 100)] max: u16,
) -> impl IntoView {
    view! { cx,
        <div style:padding-top="20px">
            <progress
                max=max
                // signals are functions, so this <=> `move || count.get()`
                value=progress
            />
            <span style:padding-left="20px">
                {progress}
            </span>
        </div>
    }
}
