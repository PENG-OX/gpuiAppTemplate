use gpui::*;
use gpui_component::*;
use gpui_component_assets::Assets;

pub struct App;

impl App {
    fn center(&mut self) -> Div {
        h_flex()
            .size_full()
            .bg(orange_100())
            .child(div().w_20().h_full().bg(rgb(0x393939)))
            .child(div().size_full().bg(orange_50()))
            .child(div().w_20().h_full().bg(rgb(0x393939)))
    }
}

impl Render for App {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(div().bg(red()).h_10())
            .child(self.center())
            .child(div().bg(black()).h_10())
    }
}

pub fn create_app() {
    let app = gpui_platform::application().with_assets(Assets);

    app.run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);

        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(680.), px(480.)),
                cx,
            ))),
            ..Default::default()
        };
        cx.spawn(async move |cx| {
            let window = cx
                .open_window(options, |window, cx| {
                    let view = cx.new(|_| App);
                    cx.new(|cx| Root::new(view, window, cx))
                })
                .expect("failed to open window");

            window
                .update(cx, |_, window, _cx| {
                    window.activate_window();
                    window.set_window_title("gpui_app");
                })
                .expect("Failed to update window");
        })
        .detach();
        cx.on_window_closed(|cx, _window_id| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();
    });
}
