use dioxus::prelude::*;

const STYLE: &'static str = include_str!("./style.css");
use crate::{icons::{Icon, IconElement}, elements::input::Input};

#[derive(Props)]
pub struct Props<'a> {
    #[props(optional)]
    text: Option<String>,
    #[props(optional)]
    with_rename: Option<bool>,
    #[props(optional)]
    onrename: Option<EventHandler<'a, String>>,
    #[props(optional)]
    onpress: Option<EventHandler<'a>>,
}

pub fn get_text(cx: &Scope<Props>) -> String {
    match &cx.props.text {
        Some(val) => val.to_owned(),
        None => String::from(""),
    }
}

pub fn emit(cx: &Scope<Props>, s: String) {
    match &cx.props.onrename {
        Some(f) => f.call(s),
        None => {},
    }
}

pub fn emit_press(cx: &Scope<Props>) {
    match &cx.props.onpress {
        Some(f) => f.call(()),
        None => {},
    }
}


#[allow(non_snake_case)]
pub fn File<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let text = get_text(&cx);
    let placeholder = text.clone();
    let with_rename = cx.props.with_rename.unwrap_or_default();

    cx.render(rsx!(
        style {
            "{STYLE}"
        },
        div {
            class: "file",
            div {
                class: "icon",
                onclick: move |_| emit_press(&cx),
                IconElement {
                    icon: Icon::Document,
                },
            },
            with_rename.then(|| rsx! (
                Input {
                    placeholder: placeholder,
                    onreturn: move |s| emit(&cx, s)
                }
            )),
            (!with_rename).then(|| rsx! (
                label {
                    "{text}"
                }
            ))
        }
    ))
}
