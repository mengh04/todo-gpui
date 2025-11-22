use gpui::*;
use gpui_component::{
    button::Button,
    input::{Input, InputState},
    label::Label,
    *,
};

// 1. 定义事件：告诉父组件“我要被删除了”
pub enum TodoCardEvent {
    Delete,
}

struct TodoCard {
    text: SharedString,
}

// 2. 让 TodoCard 具备发送事件的能力
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
                            // 3. 点击按钮时，发射 Delete 事件
                            cx.emit(TodoCardEvent::Delete);
                        })
                    })
                    .ml_auto(),
            )
    }
}

struct Todo {
    input: Entity<InputState>,
    todo_cards: Vec<Entity<TodoCard>>,
    // 4. 新增 subscriptions：用来持有“监听器”，如果不存下来，监听会立刻失效
    subscriptions: Vec<Subscription>,
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
                                    let new_card = cx.new(|_| TodoCard {
                                        text: SharedString::from(text),
                                    });

                                    // 5. 关键逻辑：订阅新卡片的事件
                                    // 参数：(父组件引用, 发送事件的子组件, 事件内容, cx)
                                    let subscription =
                                        cx.subscribe(&new_card, |this, emitter, event, _cx| {
                                            match event {
                                                TodoCardEvent::Delete => {
                                                    // 在列表中移除那个“发出声音”的卡片
                                                    this.todo_cards.retain(|card| card != &emitter);
                                                }
                                            }
                                        });

                                    // 保存这个监听器，并保存卡片
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

fn main() {
    let app = Application::new().with_assets(gpui_component_assets::Assets);

    app.run(move |cx| {
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
                        subscriptions: vec![], // 初始化为空
                    });
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
