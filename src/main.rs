mod launch;

use ori::prelude::*;

fn button_example() {
    fn ui(_: &mut ()) -> impl View {
        let regular = on_click(button(text!("Click me!")), |_, _| {});
        let fancy = on_click(button(text!("I'm fancy!")).fancy(4.0), |_, _| {});

        center(vstack![regular, fancy].gap(8.0))
    }

    launch::launch_example((), ui, 400, 300);
}

fn main() {
    console_error_panic_hook::set_once();

    let location = web_sys::window().unwrap().location();

    match location.pathname().unwrap().as_str() {
        "/button" => button_example(),
        _ => {}
    }
}
