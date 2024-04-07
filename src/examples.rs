use ori::prelude::*;

use crate::launch;

const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
    Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, \
    quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. \
    Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. \
    Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

pub fn align_example(alignment: Alignment) {
    launch::launch_example(alignment, 400, 300, |alignment| {
        let content = align(*alignment, button(text!("Align")));
        let view = container(content)
            .background(palette().background())
            .border_width(1.0);

        pad(20.0, view)
    });
}

pub fn button_example() {
    launch::launch_example((), 400, 300, |_| {
        let regular = on_click(button(text!("Click me!")), |_, _| {});
        let fancy = on_click(button(text!("I'm fancy!")).fancy(4.0), |_, _| {});

        center(vstack![regular, fancy].gap(8.0))
    });
}

pub fn checkbox_example() {
    launch::launch_example(false, 400, 300, |checked| {
        let checkbox = on_click(checkbox(*checked), |_, c: &mut bool| *c = !*c);

        center(hstack![text!("Check me!"), checkbox].gap(8.0))
    });
}

pub fn collapsing_example() {
    launch::launch_example((), 400, 300, |_| {
        let view = collapsing(
            text!("Collapsing"),
            text!("This is a collapsing view.\nIt can be toggled by clicking on the title."),
        );

        top(max_width(120.0, pad_top(80.0, view)))
    });
}

pub fn container_example() {
    launch::launch_example((), 400, 300, |_| {
        let content = text!("Contained");

        center(
            vstack![
                text!("Not contained").color(Color::RED),
                container(pad(50.0, content))
            ]
            .gap(8.0),
        )
    });
}

pub fn flex_example() {
    launch::launch_example((), 400, 300, |_| {
        let view = hstack![
            container(pad(12.0, text!("No flex"))),
            flex(container(center(text!("Flex")))),
        ]
        .align(Align::Stretch)
        .gap(8.0);

        pad(20.0, view)
    });
}

pub fn pad_example() {
    launch::launch_example((), 400, 300, |_| {
        let inner = container(pad(10.0, text!("Padded")));
        let view = container(pad(20.0, inner))
            .background(palette().background())
            .border_width(1.0);

        center(view)
    });
}

pub fn scroll_example() {
    launch::launch_example((), 400, 300, |_| {
        center(max_size(200.0, vscroll(pad(10.0, text(LOREM_IPSUM)))))
    });
}

pub fn stack_example() {
    struct Data {
        justify: Justify,
        align: Align,
    }

    fn justify_button(justify: Justify) -> impl View<Data> {
        let label = text!("{:?}", justify).font_size(12.0);
        let button = button(label).padding(4.0);
        on_click(button, move |_, data: &mut Data| data.justify = justify)
    }

    fn align_button(align: Align) -> impl View<Data> {
        let label = text!("{:?}", align).font_size(12.0);
        let button = button(label).padding(4.0);
        on_click(button, move |_, data: &mut Data| data.align = align)
    }

    let data = Data {
        justify: Justify::Center,
        align: Align::Center,
    };

    launch::launch_example(data, 400, 600, |data| {
        let justify = vstack![
            text!("Justify"),
            justify_button(Justify::Start),
            justify_button(Justify::Center),
            justify_button(Justify::End),
            justify_button(Justify::SpaceBetween),
            justify_button(Justify::SpaceAround),
            justify_button(Justify::SpaceEvenly),
        ]
        .align(Align::Stretch)
        .gap(8.0);

        let align = vstack![
            text!("Align"),
            align_button(Align::Start),
            align_button(Align::Center),
            align_button(Align::End),
            align_button(Align::Stretch),
        ]
        .align(Align::Stretch)
        .gap(8.0);

        let options = hstack![flex(justify), flex(align)]
            .gap(8.0)
            .justify(Justify::SpaceBetween)
            .align(Align::Start);

        let view = vstack![
            text!("First").color(Color::RED),
            text!("Second").color(Color::GREEN),
            text!("Third").color(Color::BLUE),
            button(text!("Click me!")),
        ]
        .justify(data.justify)
        .align(data.align)
        .gap(8.0);

        let view = container(size(FILL, center(height(FILL, view)))).border_width(1.0);

        pad(20.0, vstack![options, flex(view)].gap(8.0))
    });
}

pub fn text_example() {
    launch::launch_example((), 400, 300, |_| {
        let normal = text!("This is some normal text");
        let bold = text!("This is some bold text").font_weight(FontWeight::BOLD);
        let italic = text!("This is some italic text").font_style(FontStyle::Italic);

        center(vstack![normal, bold, italic].gap(8.0))
    });
}

pub fn text_input_example() {
    launch::launch_example((), 400, 300, |_| {
        let input = text_input().placeholder("Type something...");

        center(container(pad(8.0, input)))
    });
}

pub fn tooltip_example() {
    launch::launch_example((), 400, 300, |_| {
        let view = tooltip(text!("Hover me!"), "This is a tooltip");

        center(view)
    });
}

pub fn wrap_example() {
    fn item() -> impl View {
        let view = background(palette().primary(), ()).border_radius(4.0);

        size(50.0, view)
    }

    launch::launch_example((), 400, 300, |_| {
        center(pad(
            50.0,
            hwrap![
                item(),
                item(),
                item(),
                item(),
                item(),
                item(),
                item(),
                item(),
                item()
            ]
            .gap(8.0),
        ))
    });
}

pub fn transform_example() {
    launch::launch_example((), 400, 300, |_| center(rotate(0.2, text("Transform"))));
}

pub fn zstack_example() {
    fn item() -> impl View {
        let view = background(palette().accent_lighter().desaturate(0.5), ()).border_radius(4.0);

        size(50.0, view)
    }

    launch::launch_example((), 400, 300, |_| {
        let items = Vec::from_iter((0..20).map(|_| item()));
        let view = hwrap(items).gap(8.0);

        let button = button(text("I am on top!")).fancy(4.0);
        let button = bottom_right(pad(10.0, button));

        center(pad([50.0, 0.0], zstack![view, button]))
    });
}
