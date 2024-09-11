use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (x, set_x) = create_signal(0);

    view! {
        <button
            class:red=move || count() % 2 == 1
            on:click=move |_| {
                set_count.update(|n| *n += 1);
                set_x.update(|n| *n += 100);
            }

            style="position:absolute"
            style:left=move || format!("{}px", x())
            style:background-color=move || format!("rgb({}, {}, 100)", x(), x())
        >
            "Click me: "
            {count}
        </button>

        <ProgressBar progress=count />
        <ProgressBar progress=count top=200 />
    }
}

/// I am progress bar
#[component]
fn ProgressBar(
    /// I am a top
    #[prop(default = 100)]
    top: u16,
    /// I am a progress
    progress: ReadSignal<i32>,
) -> impl IntoView {
    view! {
        <progress
            max=5
            value=progress

            style="position:absolute"
            style:top=format!("{}px", top)
        />
    }
}
