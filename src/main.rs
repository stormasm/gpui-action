use gpui::*;

mod focus;
use focus::FocusStory;

actions!(ford, [Quit]);

fn main() {
    App::new().run(|cx| {
        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

        let bounds = Bounds::centered(None, size(px(600.0), px(600.0)), cx);
        let window_options = WindowOptions {
            bounds: Some(bounds),
            ..Default::default()
        };

        cx.open_window(window_options, |cx| cx.new_view(FocusStory::new));
    });
}
