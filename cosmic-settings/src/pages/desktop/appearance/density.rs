use super::{Message, Page};
use cosmic::config::CosmicTk;
use cosmic::cosmic_config::ConfigGet;
use cosmic::cosmic_theme::Density;
use cosmic::iced_core::Length;
use cosmic::widget::{radio, settings, text};
use cosmic::{Apply, Element};
use cosmic_settings_page::Section;

pub fn render() -> Section<crate::pages::Message> {
    crate::slab!(descriptions {
        comfortable = fl!("interface-density", "comfortable");
        compact = fl!("interface-density", "compact");
        spacious = fl!("interface-density", "spacious");
    });

    Section::default()
        .title(fl!("interface-density"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, _page, section| {
            let descriptions = &section.descriptions;
            let config = CosmicTk::config().ok().unwrap();
            let density = config.get("interface_density").unwrap();

            settings::section()
                .title(&section.title)
                .add(settings::item_row(vec![
                    radio(
                        text::body(&descriptions[compact]),
                        Density::Compact,
                        Some(density),
                        Message::Density,
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .add(settings::item_row(vec![
                    radio(
                        text::body(&descriptions[comfortable]),
                        Density::Standard,
                        Some(density),
                        Message::Density,
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .add(settings::item_row(vec![
                    radio(
                        text::body(&descriptions[spacious]),
                        Density::Spacious,
                        Some(density),
                        Message::Density,
                    )
                    .width(Length::Fill)
                    .into(),
                ]))
                .apply(Element::from)
                .map(crate::pages::Message::Appearance)
        })
}
