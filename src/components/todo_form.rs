use crate::bindings::alert;
use crate::models::todo::Todo;
use stylist::yew::use_style;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TodoFormProps {
    pub onsubmit: Callback<Todo>,
}

#[function_component(TodoForm)]
pub fn todo_form(props: &TodoFormProps) -> Html {
    let title_ref = use_node_ref();
    let description_ref = use_node_ref();

    let form = use_style!(
        r#"
            background-color: #fafafa;
            padding: 16px;
            box-shadow: -4px 4px 12px rgba(0, 0, 0, 0.4);
            border-radius: 8px;
            display: flex;
            flex-direction: column;
            row-gap: 8px;
            max-width: 300px;
        "#
    );

    let onsubmit = {
        let onsubmit = props.onsubmit.clone();
        let title_ref = title_ref.clone();
        let description_ref = description_ref.clone();

        move |event: FocusEvent| {
            event.prevent_default();

            let title_elem = title_ref.cast::<HtmlInputElement>().unwrap();
            let description_elem = description_ref.cast::<HtmlInputElement>().unwrap();

            let title = title_elem.value();
            let description = description_elem.value();

            if title == "" {
                unsafe {
                    alert("The title cannot be empty");
                }
                return;
            }

            if description == "" {
                unsafe {
                    alert("The description cannot be empty");
                }
                return;
            }

            onsubmit.emit(Todo {
                title,
                description,
                id: rand::random::<i32>().to_string(),
            });

            title_elem.set_value("");
            description_elem.set_value("");
        }
    };

    html! {
        <form {onsubmit} class={form}>
            <input ref={title_ref} placeholder={"Todo title"} />
            <input ref={description_ref} placeholder={"Todo description"} />
            <button type="submit">{"Submit!"}</button>
        </form>
    }
}
