use cosmic_settings_page::Section;
use cosmic::widget::settings;
use cosmic::{Apply, Element, widget};
use super::{Message, Page};
use slab::Slab;

#[allow(clippy::too_many_lines)]
pub fn render() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let active_hint = descriptions.insert(fl!("window-management-appearance", "active-hint"));
    let gaps = descriptions.insert(fl!("window-management-appearance", "gaps"));

    Section::default()
        .title(fl!("window-management-appearance"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let descriptions = &section.descriptions;

            settings::section()
                .title(&section.title)
                .add(settings::item::builder(&descriptions[active_hint]).control(
                    widget::spin_button(
                        page.theme_builder.active_hint.to_string(),
                        page.theme_builder.active_hint,
                        1,
                        0,
                        64,
                        Message::WindowHintSize,
                    ),
                ))
                .add(
                    settings::item::builder(&descriptions[gaps]).control(widget::spin_button(
                        page.theme_builder.gaps.1.to_string(),
                        page.theme_builder.gaps.1,
                        1,
                        page.theme_builder.active_hint,
                        500,
                        Message::GapSize,
                    )),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Appearance)
        })
}

