use gpui::*;
use gpui_component::{button::Button, label::Label, *};

pub enum TodoCardEvent {
    Delete,
}

pub struct TodoCard {
    text: SharedString,
}

impl TodoCard {
    pub fn new(text: SharedString) -> Self {
        Self { text }
    }
}

impl EventEmitter<TodoCardEvent> for TodoCard {}

impl Render for TodoCard {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view_handle = cx.entity();

        div()
            .h_flex()
            .p_3()
            .mb_2()
            .border_1()
            .border_color(rgb(0xc0c0c0))
            .child(Label::new(&self.text))
            .child(
                Button::new("delete")
                    .icon(IconName::Delete)
                    .on_click(move |_, _, cx| {
                        view_handle.update(cx, |_this, cx| {
                            cx.emit(TodoCardEvent::Delete);
                        })
                    })
                    .ml_auto(),
            )
    }
}
