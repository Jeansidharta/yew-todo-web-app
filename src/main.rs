#![allow(unused_unsafe)]

use pages::home::Home;

mod bindings;
mod components;
mod models;
mod pages;

fn main() {
    yew::start_app::<Home>();
}
