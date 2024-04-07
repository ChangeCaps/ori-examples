use ori::prelude::*;

#[allow(unused)]
fn set_canvas<T>(content: impl View<T>, width: u32, height: u32) -> impl View<T> {
    on_event(content, move |cx, _, _| {
        #[cfg(target_arch = "wasm32")]
        {
            use ori::winit::WinitWindow;
            use winit::platform::web::WindowExtWebSys;

            struct CanvasSet;

            if cx.contains_context::<CanvasSet>() {
                return;
            }

            let window = cx.window().downcast_raw::<WinitWindow>().unwrap();

            if let Some(canvas) = window.canvas() {
                canvas.set_width(width);
                canvas.set_height(height);

                web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .body()
                    .unwrap()
                    .append_child(&canvas)
                    .unwrap();

                cx.insert_context(CanvasSet);
            }
        }
    })
}

struct ExampleDelegate;

impl<T> Delegate<T> for ExampleDelegate {
    fn idle(&mut self, cx: &mut DelegateCx<T>, _data: &mut T) {
        cx.cmd(());
    }

    fn event(&mut self, _cx: &mut DelegateCx<T>, _data: &mut T, _event: &Event) {}
}

pub fn launch_example<T: 'static, V: View<T> + 'static>(
    data: T,
    width: u32,
    height: u32,
    mut ui: impl FnMut(&mut T) -> V + 'static,
) {
    let window = WindowDescriptor::new();

    Launcher::new(data)
        .window(window, move |data| set_canvas(ui(data), width, height))
        .delegate(ExampleDelegate)
        .launch();
}
