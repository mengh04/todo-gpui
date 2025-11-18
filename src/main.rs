use gpui::*;
use gpui_component::{
    button::Button,
    input::{Input, InputState},
    label::Label,
    *,
};

struct TodoCard {
    text: SharedString,
}

impl Render for TodoCard {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .p_3()
            .mb_2()
            .border_1()
            .border_color(rgb(0xc0c0c0))
            .child(Label::new(&self.text))
    }
}

struct Todo {
    input: Entity<InputState>,
    todo_cards: Vec<Entity<TodoCard>>,
}

impl Render for Todo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let input_handle = self.input.clone();
        let todo_cards_handle = self.todo_cards.clone();
        div()
            .v_flex()
            .size_full()
            .child(
                // 上面的内容区域，自适应高度
                div()
                    .id("todo-content")
                    .flex_grow()
                    .overflow_y_scroll()
                    .p_4()
                    // .child(cx.new(|_| TodoCard {
                    //     text: SharedString::new("Learn GPUI Components"),
                    // }))
                    // .children(self.todo_cards.iter()),
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
                        move |_, _window, cx| {
                            let text = input_handle.read(cx).value();

                            // todo_cards_handle.update(cx, |this, cx| {
                            //     this.
                            // })
                        },
                    )),
            )
    }
}

fn main() {
    let app = Application::new().with_assets(gpui_component_assets::Assets);

    app.run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);

        let bounds = Bounds::centered(None, size(px(350.), px(600.)), cx);

        cx.spawn(async move |cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |window, cx| {
                    let input_state = cx.new(|cx| InputState::new(window, cx));
                    let view = cx.new(|_| Todo {
                        input: input_state,
                        todo_cards: vec![],
                    });
                    // This first level on the window, should be a Root.
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
