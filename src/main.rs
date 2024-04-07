mod launch;

use ori::prelude::*;
use url::Url;

fn button_example() {
    fn ui(_: &mut ()) -> impl View {
        let regular = on_click(button(text!("Click me!")), |_, _| {});
        let fancy = on_click(button(text!("I'm fancy!")).fancy(4.0), |_, _| {});

        center(vstack![regular, fancy].gap(8.0))
    }

    launch::launch_example((), ui, 400, 300);
}

fn invalid_example() {
    fn ui(_: &mut ()) -> impl View {
        center(text!("Invalid example"))
    }

    launch::launch_example((), ui, 400, 300);
}

fn show_example(example: &str) {
    match example {
        "button" => button_example(),
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
