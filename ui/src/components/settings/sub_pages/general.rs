use dioxus::prelude::*;
use kit::{
    elements::{button::Button, select::Select, switch::Switch},
    icons::Icon,
};

use crate::{
    components::settings::SettingSection,
    state::{Action, State},
    utils::{
        get_available_themes,
        language::{change_language, get_available_languages, get_local_text},
    },
};

#[allow(non_snake_case)]
pub fn GeneralSettings(cx: Scope) -> Element {
    let state = use_context::<State>(&cx).unwrap();
    let initial_lang_value = state.read().settings.language.clone();

    let themes = get_available_themes();

    cx.render(rsx!(
        div {
            id: "settings-general",
            SettingSection {
                section_label: get_local_text("settings-general.splash-screen"),
                section_description: get_local_text("settings-general.splash-screen-description"),
                Switch {

                }
            },
            SettingSection {
                section_label: get_local_text("settings-general.theme"),
                section_description: get_local_text("settings-general.theme-description"),
                Select {
                    initial_value: if let Some(theme) = &state.read().ui.theme {
                        theme.name.clone()
                    } else {
                        "Default".to_string()
                    },
                    options: themes.iter().map(|t| t.name.clone()).collect(),
                    onselect: move |value| {
                        themes.iter().for_each(|t| {
                            if t.name == value {
                                state.write().mutate(Action::SetTheme(t.clone()));
                            }
                        })
                    }
                }
            },
            SettingSection {
                section_label: get_local_text("settings-general.theme-reset"),
                section_description: get_local_text("settings-general.theme-reset-description"),
                Button {
                    text: get_local_text("settings-general.theme-reset-cta"),
                    icon: Icon::Trash,
                    appearance: kit::elements::Appearance::Secondary,
                    onpress: move |_| {
                        state.write().mutate(Action::ClearTheme);
                    }
                }
            },
            SettingSection {
                section_label: get_local_text("settings-general.app-language"),
                section_description: get_local_text("settings-general.change-language"),
                Select {
                    initial_value: initial_lang_value,
                    options: get_available_languages(),
                    onselect: move |value| {
                        let new_app_lang = change_language(value);
                        state.write().mutate(Action::SetLanguage(new_app_lang));
                    }
                }
            },
        }
    ))
}