use gpui_fps::fps_monitor;
use gpui_kit::component::button::*;
use gpui_kit::component::*;
use gpui_kit::prelude::*;
use gpui_kit::*;

pub struct HelloWorld {
    show_fps: bool,
}

impl Render for HelloWorld {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .relative()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .when(self.show_fps, |this| this.child(fps_monitor(window, cx)))
            .child("Hello, World!")
            .child(
                Button::new("ok")
                    .primary()
                    .label("Let's Go Now!")
                    .on_click(|_, _, _| println!("Clicked!")),
            )
            .child(
                Button::new("toggle_fps")
                    .primary()
                    .label("Toggle FPS")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.show_fps = !this.show_fps;
                        cx.notify();
                    })),
            )
    }
}

fn main() {
    gpui_kit::application().run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_kit::init(cx);

        gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| {
            cx.new(|_| HelloWorld { show_fps: true })
        })
        .expect("Failed to open window");
    });
}
