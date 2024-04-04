use freya::prelude::*;

pub async fn launch() -> anyhow::Result<()> {
    launch_with_props(app, "RustyAmp", (400., 300.));
    Ok(())
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    render!(
        rect {
            height: "100%",
            width: "100%",
            background: "rgb(35, 35, 35)",
            color: "white",
            padding: "12",
            onclick: move |_| count += 1,
            label { "Click to increase -> {count}" }
        }
    )
}
