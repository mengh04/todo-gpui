use gpui::*;
use gpui_component::{
    button::Button,
    input::{Input, InputState},
    *,
};

use super::todo_card::{TodoCard, TodoCardEvent};

pub struct Todo {
    input: Entity<InputState>,
    todo_cards: Vec<Entity<TodoCard>>,
    subscriptions: Vec<Subscription>,
}

impl Todo {
    /// Create a new `Todo` component given an `InputState` entity.
    /// This provides a public constructor so callers don't need to access fields directly.
    pub fn new(input: Entity<InputState>) -> Self {
        Self {
            input,
            todo_cards: vec![],
            subscriptions: vec![],
        }
    }
}

impl Render for Todo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view_handle = cx.entity();

        div()
            .v_flex()
            .size_full()
            .child(
                div()
                    .id("todo-content")
                    .flex_grow()
                    .overflow_y_scroll()
                    .p_4()
                    .children(self.todo_cards.clone()),
            )
            .child(
                div()
                    .h_flex()
                    .p_3()
                    .gap_3()
                    .border_t_1()
                    .border_color(rgb(0xe0e0e0))
                    .child(Input::new(&self.input))
                    .child(Button::new("add").icon(IconName::Plus).on_click(
                        move |_, window, cx| {
                            view_handle.update(cx, |this, cx| {
                                let text = this.input.read(cx).value();
                                if !text.is_empty() {
                                    let new_card =
                                        cx.new(|_| TodoCard::new(SharedString::from(text)));

                                    let subscription =
                                        cx.subscribe(&new_card, |this, emitter, event, _cx| {
                                            // Use `match` for exhaustive/clear handling and to avoid
                                            // irrefutable `if let` warnings.
                                            match event {
                                                TodoCardEvent::Delete => {
                                                    this.todo_cards.retain(|card| card != &emitter);
                                                }
                                            }
                                        });

                                    this.subscriptions.push(subscription);
                                    this.todo_cards.push(new_card);

                                    this.input.update(cx, |input, cx| {
                                        input.set_value("", window, cx);
                                    })
                                }
                            })
                        },
                    )),
            )
    }
}
