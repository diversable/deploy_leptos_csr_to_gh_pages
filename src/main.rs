use leptos::*;

#[component]
fn ProgressBar(
    // #[prop(optional)]
    // #[prop(default = 0)]
    // #[prop(into)]
    progress: ReadSignal<i32>,
    #[prop(optional)]
    #[prop(default = 100)]
    max: i32,
) -> impl IntoView {
    view! {
        <p>Progress: {progress} & Max: {max}</p>
        <progress value=progress max=max></progress>
    }
}

#[component]
fn ListExample(count: ReadSignal<i32>) -> impl IntoView {
    let vals = vec![1, 2, 3];

    view! {
        <ul>
        {vals.into_iter()
            .map(|n| view!{
                <li>
                    "x"{n} " -> " {move || n * count()}
                </li>
            })
            .collect::<Vec<_>>()
        }
    </ul>
    }
}

#[component]
fn App(increment: i32) -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
    <div class="container">
        <h1>"Welcome to Leptos"</h1>
        <h2><i>"On Github Pages"</i></h2>

        <button
            on:click= move |_| {
                set_count(count() + increment)
            }
        >
            "Click me: "
            {count}
        </button>

        <ListExample count=count />
    </div>
    }
}

fn main() {
    mount_to_body(|| {
        view! {
            <App increment=5 />
        }
    })
}
