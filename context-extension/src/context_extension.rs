//! # GPUI App extension example
//!
//! Add gpui to your Cargo.toml:
//!     gpui = { git = "https://github.com/zed-industries/zed", rev = "c04c5812b6295ab683fbf1900499330cbc2b3058"}

use gpui::{
    bounds, div, hsla, point, px, size, App, AppContext as _, Application, Context, FocusHandle,
    Global, Hsla, IntoElement, Menu, ParentElement as _, Render, Styled as _, TitlebarOptions,
    Window, WindowBounds, WindowOptions,
};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Theme {
    fg: Hsla,
    bg: Hsla,
    accent: Hsla,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            fg: hsla(220.0 / 360.0, 9.0 / 100.0, 72.0 / 100.0, 1.0),
            bg: hsla(220.0 / 360.0, 14.0 / 100.0, 18.0 / 100.0, 1.0),
            accent: hsla(207.0 / 360.0, 82.0 / 100.0, 66.0 / 100.0, 1.0),
        }
    }
}

impl Theme {
    pub fn get_global(cx: &App) -> &Arc<Theme> {
        &cx.global::<GlobalTheme>().0
    }
}

#[derive(Clone, Debug)]
pub struct GlobalTheme(pub Arc<Theme>);

impl Deref for GlobalTheme {
    type Target = Arc<Theme>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GlobalTheme {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Global for GlobalTheme {}

pub trait ActiveTheme {
    fn theme(&self) -> &Arc<Theme>;
}

impl ActiveTheme for App {
    fn theme(&self) -> &Arc<Theme> {
        &self.global::<GlobalTheme>().0
    }
}

pub struct AppExtensionExample {
    focus_handle: FocusHandle,
}

impl Render for AppExtensionExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme_colors = div().flex().gap(px(12.)).children(vec![
            div()
                .size_6()
                .rounded_full()
                .bg(cx.theme().fg)
                .border_1()
                .border_color(gpui::white().alpha(0.12)),
            div()
                .size_6()
                .rounded_full()
                .bg(cx.theme().bg)
                .border_1()
                .border_color(gpui::white().alpha(0.12)),
            div()
                .size_6()
                .rounded_full()
                .bg(cx.theme().accent)
                .border_1()
                .border_color(gpui::white().alpha(0.12)),
        ]);

        div()
            .flex()
            .flex_col()
            .flex_initial()
            .p_4()
            .w(px(200.0))
            .h(px(160.0))
            .justify_center()
            .items_center()
            .text_center()
            .text_xs()
            .text_color(cx.theme().fg)
            .bg(cx.theme().bg)
            .gap(px(6.))
            .child("Our theme colors!")
            .child(theme_colors)
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.set_menus(vec![Menu {
            name: "App Extensions".into(),
            items: vec![],
        }]);

        cx.set_global(GlobalTheme(Arc::new(Theme::default())));

        let window = cx
            .open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Context Extension".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(bounds(
                        point(px(0.0), px(0.0)),
                        size(px(200.), px(160.)),
                    ))),
                    ..Default::default()
                },
                |_window, cx| {
                    cx.new(|cx| AppExtensionExample {
                        focus_handle: cx.focus_handle(),
                    })
                },
            )
            .unwrap();

        window
            .update(cx, |view, window, cx| {
                window.focus(&view.focus_handle);
                cx.activate(true);
            })
            .unwrap();
    })
}
