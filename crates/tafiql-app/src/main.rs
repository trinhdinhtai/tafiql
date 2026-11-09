//! tafiql's GPUI shell: a localized window whose theme follows the system
//! appearance (dark default). The Result Grid and everything data-related
//! land here in later tickets; this window only renders strings from the
//! fluent i18n layer.

mod i18n;
mod theme;

use gpui::{
    div, prelude::*, px, size, App, Application, Bounds, Context, FontWeight, TitlebarOptions,
    Window, WindowBounds, WindowOptions,
};

use i18n::{I18n, UiLanguage};
use theme::Theme;

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.set_global(I18n::new(UiLanguage::English));

        let options = WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some("tafiql".into()),
                appears_transparent: false,
                traffic_light_position: None,
            }),
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(1024.), px(640.)),
                cx,
            ))),
            ..Default::default()
        };

        cx.open_window(options, |window, cx| {
            let shell = cx.new(|_| ShellView);
            window
                .observe_window_appearance({
                    let shell = shell.downgrade();
                    move |_, cx| {
                        // The theme is derived from window.appearance() on
                        // every render; this only triggers the redraw.
                        shell.update(cx, |_, cx| cx.notify()).ok();
                    }
                })
                .detach();
            shell
        })
        .unwrap();

        cx.activate(true);
    });
}

/// The root view of the scaffold window.
struct ShellView;

impl ShellView {
    /// The header: title + subtitle on the left, language toggle on the right.
    fn render_header(&self, i18n: &I18n, theme: &Theme) -> impl IntoElement {
        let title = i18n.tr("app-title");
        let subtitle = i18n.tr("shell-subtitle");
        let toggle = i18n.tr("language-toggle");
        div()
            .flex()
            .items_center()
            .justify_between()
            .bg(theme.panel)
            .p_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_size(px(18.))
                            .font_weight(FontWeight::MEDIUM)
                            .child(title),
                    )
                    .child(
                        div()
                            .text_size(px(12.))
                            .text_color(theme.muted_foreground)
                            .child(subtitle),
                    ),
            )
            .child(
                div()
                    .id("language-toggle")
                    .px_3()
                    .py_2()
                    .rounded_md()
                    .bg(theme.accent)
                    .text_color(theme.accent_foreground)
                    .text_size(px(12.))
                    .cursor_pointer()
                    .hover(|style| style.opacity(0.9))
                    .on_click(|_, _, cx| {
                        cx.global_mut::<I18n>().toggle_language();
                        cx.refresh_windows();
                    })
                    .child(toggle),
            )
    }
}

impl Render for ShellView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // The palette follows the system appearance; dark is the default
        // (user story 34).
        let theme = Theme::from_appearance(window.appearance());
        let welcome = cx.global::<I18n>().tr("shell-welcome");

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.background)
            .text_color(theme.foreground)
            .child(self.render_header(cx.global::<I18n>(), &theme))
            .child(
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(theme.muted_foreground)
                    .child(welcome),
            )
    }
}
