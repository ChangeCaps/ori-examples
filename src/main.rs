mod examples;
mod launch;

use ori::prelude::*;
use url::Url;

fn invalid_example() {
    fn ui(_: &mut ()) -> impl View {
        center(text!("Invalid example"))
    }

    launch::launch_example((), 400, 300, ui);
}

fn show_example(example: &str) {
    match example {
        /* align */
        "align" => examples::align_example(Alignment::new(0.6, 0.3)),
        "top_left" => examples::align_example(Alignment::TOP_LEFT),
        "top" => examples::align_example(Alignment::TOP),
        "top_right" => examples::align_example(Alignment::TOP_RIGHT),
        "left" => examples::align_example(Alignment::LEFT),
        "center" => examples::align_example(Alignment::CENTER),
        "right" => examples::align_example(Alignment::RIGHT),
        "bottom_left" => examples::align_example(Alignment::BOTTOM_LEFT),
        "bottom" => examples::align_example(Alignment::BOTTOM),
        "bottom_right" => examples::align_example(Alignment::BOTTOM_RIGHT),

        /* button */
        "button" => examples::button_example(),

        /* checkbox */
        "checkbox" => examples::checkbox_example(),

        /* collapsing */
        "collapsing" => examples::collapsing_example(),

        /* container */
        "container" => examples::container_example(),

        /* flex */
        "flex" => examples::flex_example(),

        /* padding */
        "pad" => examples::pad_example(),

        /* scroll */
        "scroll" => examples::scroll_example(),

        /* stack */
        "stack" => examples::stack_example(),

        /* text */
        "text" => examples::text_example(),

        /* text_input */
        "text_input" => examples::text_input_example(),

        /* tooltip */
        "tooltip" => examples::tooltip_example(),

        /* wrap */
        "wrap" => examples::wrap_example(),

        /* transform */
        "transform" => examples::transform_example(),

        /* zstack */
        "zstack" => examples::zstack_example(),

        /* invalid */
        _ => invalid_example(),
    }
}

fn main() -> anyhow::Result<()> {
    console_error_panic_hook::set_once();

    let location = web_sys::window().unwrap().location();
    let href = location.href().map_err(|e| anyhow::anyhow!("{:?}", e))?;
    let url = Url::parse(&href)?;

    let example = url
        .query_pairs()
        .find_map(|(key, value)| (key == "example").then_some(value));

    if let Some(example) = example {
        show_example(&example);
    } else {
        invalid_example();
    }

    Ok(())
}
