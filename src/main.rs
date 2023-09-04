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
        <h1>"Iteration"</h1>
        <h2>"Static List"</h2>
        <p>"Use this pattern if the list itself is static."</p>
        <StaticList length=5/>
        <h2>"Dynamic List"</h2>
        <p>"Use this pattern if the rows in your list will change."</p>
        <DynamicList initial_length=5/>
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

/// A list of counters, without the ability
/// to add or remove any.
#[component]
fn StaticList(
    cx: Scope,
    /// How many counters to include in this list.
    length: usize,
) -> impl IntoView {
    // create counter signals that start at incrementing numbers
    let counters = (1..=length).map(|idx| create_signal(cx, idx));

    // when you have a list that doesn't change, you can
    // manipulate it using ordinary Rust iterators
    // and collect it into a Vec<_> to insert it into the DOM
    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! { cx,
                <li>
                    <button
                        on:click=move |_| set_count.update(|n| *n += 1)
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect::<Vec<_>>();

    // Note that if `counter_buttons` were a reactive list
    // and its value changed, this would be very inefficient:
    // it would rerender every row every time the list changed.
    view! { cx,
        <ul>{counter_buttons}</ul>
    }
}

/// A list of counters that allows you to add or
/// remove counters.
#[component]
fn DynamicList(
    cx: Scope,
    /// The number of counters to begin with.
    initial_length: usize,
) -> impl IntoView {
    // This dynamic list will use the <For/> component.
    // <For/> is a keyed list. This means that each row
    // has a defined key. If the key does not change, the row
    // will not be re-rendered. When the list changes, only
    // the minimum number of changes will be made to the DOM.

    // `next_counter_id` will let us generate unique IDs
    // we do this by simply incrementing the ID by one
    // each time we create a counter
    let mut next_counter_id = initial_length;

    // we generate an initial list as in <StaticList/>
    // but this time we include the ID along with the signal
    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(cx, id + 1)))
        .collect::<Vec<_>>();

    // now we store that initial list in a signal
    // this way, we'll be able to modify the list over time,
    // adding and removing counters, and it will change reactively
    let (counters, set_counters) = create_signal(cx, initial_counters);

    let add_counter = move |_| {
        // create a signal for the new counter
        let sig = create_signal(cx, next_counter_id + 1);
        // add this counter to the list of counters
        set_counters.update(move |counters| {
            // since `.update()` gives us `&mut T`
            // we can just use normal Vec methods like `push`
            counters.push((next_counter_id, sig))
        });
        // increment the ID so it's always unique
        next_counter_id += 1;
    };

    view! { cx,
        <div>
            <button on:click=add_counter>
                "Add Counter"
            </button>
            <ul>
                // The <For/> component is central here
                // This allows for efficient, key list rendering
                <For
                    // `each` takes any function that returns an iterator
                    // this should usually be a signal or derived signal
                    // if it's not reactive, just render a Vec<_> instead of <For/>
                    each=counters
                    // the key should be unique and stable for each row
                    // using an index is usually a bad idea, unless your list
                    // can only grow, because moving items around inside the list
                    // means their indices will change and they will all rerender
                    key=|counter| counter.0
                    // the view function receives each item from your `each` iterator
                    // and returns a view
                    view=move |cx, (id, (count, set_count))| {
                        view! { cx,
                            <li>
                                <button
                                    on:click=move |_| set_count.update(|n| *n += 1)
                                >
                                    {count}
                                </button>
                                <button
                                    on:click=move |_| {
                                        set_counters.update(|counters| {
                                            counters.retain(|(counter_id, _)| counter_id != &id)
                                        });
                                    }
                                >
                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}
