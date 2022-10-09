use stylist::yew::use_style;
use yew::prelude::*;

use crate::components::{todo::TodoComponent, todo_form::TodoForm};
use crate::models::todo::Todo;

#[function_component(Home)]
pub fn home() -> Html {
    let todos = use_state(|| Vec::<Todo>::new());

    let todos_container = use_style!(
        r#"
            display: flex;
            flex-direction: column;
            row-gap: 16px;
            margin: 32px 0;
        "#
    );

    let onsubmit = {
        let todos = todos.clone();
        Callback::from(move |todo: Todo| {
            let mut new_todos = todos.to_vec();
            new_todos.push(todo);
            todos.set(new_todos);
        })
    };

    let on_delete = {
        let todos = todos.clone();
        Callback::from(move |todo_id| {
            let mut new_todos = todos.to_vec();
            let index = new_todos
                .iter()
                .position(|todo| todo.id == todo_id)
                .expect("Todo ID not found.");

            new_todos.remove(index);
            todos.set(new_todos);
        })
    };

    html! {
        <div>
            <h1>{"Todo app"}</h1>
            <TodoForm {onsubmit} />
            <div class={todos_container}>
                {for (*todos)
                    .iter()
                    .map(|todo| html! {
                        <TodoComponent
                            todo={todo.clone()}
                            on_delete={on_delete.clone()}
                            key={todo.id.to_string()}
                        />
                    })
                }
            </div>
        </div>
    }
}
