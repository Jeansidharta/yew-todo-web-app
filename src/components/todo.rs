use crate::models::todo::Todo;
use std::rc::Rc;
use stylist::yew::{styled_component, use_style};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TodoProps {
    pub todo: Rc<Todo>,
    pub on_delete: Callback<String>,
}

#[styled_component(TodoComponent)]
pub fn todo_component(props: &TodoProps) -> Html {
    let todo = &props.todo;

    let root = use_style!(
        r#"
            padding: 16px;
            background-color: #fafafa;
            box-shadow: -4px 4px 12px rgba(0, 0, 0, 0.4);
            margin: 8px 0;
            border-radius: 8px;
        "#
    );

    let title = use_style!(
        r#"
            margin-bottom: 8px;
            font-size: 20px;
            display: flex;
            justify-content: space-between;
        "#
    );

    let delete_button = use_style!(
        r#"
            border: none;
            box-shadow: -2px 2px 4px rgba(0, 0, 0, 0.2);
            cursor: pointer;
            border-radius: 100%;
            width: 24px;
            height: 24px;
            display: flex;
            justify-content: center;
            align-items: center;
        "#
    );

    let on_delete = {
        let on_delete = props.on_delete.clone();
        let todo_id = todo.id.clone();
        move |_: MouseEvent| {
            on_delete.emit(todo_id.clone());
        }
    };

    html! {
        <div class={root}>
            <div class={title}>
                <div><strong>{&todo.title}</strong></div>
                <button onclick={on_delete} class={delete_button}>{"X"}</button>
            </div>
            <div>{&todo.description}</div>
        </div>
    }
}
