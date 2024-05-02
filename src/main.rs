use gpui::*;

mod focus;
use focus::FocusStory;

actions!(kallax, [Quit]);

fn main() {
    App::new().run(|cx| {
        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

        let window_options = WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some(SharedString::from("Kallax")),
                appears_transparent: true,
                ..Default::default()
            }),
            bounds: Some(Bounds {
                size: size(px(800.), px(600.)).into(),
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |cx| cx.new_view(FocusStory::new));
    });
}
