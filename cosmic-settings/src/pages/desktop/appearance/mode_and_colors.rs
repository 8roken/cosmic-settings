use cosmic_settings_page::Section;
use cosmic::widget::{
    button, color_picker::ColorPickerUpdate, container,
    scrollable, settings, text,
};
use cosmic::cosmic_theme::Spacing;
use cosmic::widget::icon::{from_name, icon};
use cosmic::cosmic_theme::palette::Srgba;
use cosmic::iced_core::{Alignment, Length};
use cosmic_settings_wallpaper as wallpaper;
use cosmic::{Apply, Element};
use super::{Message, Page};
use crate::pages::desktop::wallpaper::widgets::color_image;

#[allow(clippy::too_many_lines)]
pub fn render() -> Section<crate::pages::Message> {
    crate::slab!(descriptions {
        auto_txt = fl!("auto");
        auto_switch = fl!("auto-switch");
        accent_color = fl!("accent-color");
        app_bg = fl!("app-background");
        container_bg = fl!("container-background");
        container_bg_desc = fl!("container-background", "desc");
        text_tint = fl!("text-tint");
        text_tint_desc = fl!("text-tint", "desc");
        control_tint = fl!("control-tint");
        control_tint_desc = fl!("control-tint", "desc");
        window_hint_toggle = fl!("window-hint-accent-toggle");
        window_hint = fl!("window-hint-accent");
        dark = fl!("dark");
        light = fl!("light");
    });

    let dark_mode_illustration = from_name("illustration-appearance-mode-dark").handle();
    let light_mode_illustration = from_name("illustration-appearance-mode-light").handle();
    let go_next_icon = from_name("go-next-symbolic").handle();

    Section::default()
        .title(fl!("mode-and-colors"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let Spacing { space_xxs, .. } = cosmic::theme::spacing();

            let descriptions = &section.descriptions;
            let palette = &page.theme_builder.palette.as_ref();
            let cur_accent = page
                .theme_builder
                .accent
                .map_or(palette.accent_blue, Srgba::from);

            let accent_palette_values = match (
                page.theme_mode.is_dark,
                page.accent_palette.dark.as_ref(),
                page.accent_palette.light.as_ref(),
            ) {
                (true, Some(dark_palette), _) => &dark_palette,
                (false, _, Some(light_palette)) => &light_palette,
                _ => &page.accent_palette.theme,
            };

            let mut accent_palette_row =
                cosmic::widget::row::with_capacity(accent_palette_values.len());

            for &color in accent_palette_values {
                accent_palette_row = accent_palette_row.push(color_button(
                    Some(Message::PaletteAccent(color.into())),
                    color.into(),
                    cur_accent == color,
                    48,
                    48,
                ));
            }

            let accent_color_palette = cosmic::iced::widget::column![
                text::body(&descriptions[accent_color]),
                scrollable::horizontal(
                    accent_palette_row
                        .push(if let Some(c) = page.custom_accent.get_applied_color() {
                            container(color_button(
                                Some(Message::CustomAccent(ColorPickerUpdate::ToggleColorPicker)),
                                c,
                                cosmic::iced::Color::from(cur_accent) == c,
                                48,
                                48,
                            ))
                        } else {
                            container(
                                page.custom_accent
                                    .picker_button(Message::CustomAccent, None)
                                    .width(Length::Fixed(48.0))
                                    .height(Length::Fixed(48.0)),
                            )
                        })
                        .padding([0, 0, 16, 0])
                        .spacing(16)
                )
            ]
            .padding([16, 0, 0, 0])
            .spacing(space_xxs);

            let mut section = settings::section()
                .title(&section.title)
                .add(
                    container(
                        cosmic::iced::widget::row![
                            cosmic::iced::widget::column![
                                button::custom(
                                    icon(dark_mode_illustration.clone())
                                        .width(Length::Fixed(191.0))
                                        .height(Length::Fixed(100.0))
                                )
                                .class(button::ButtonClass::Image)
                                .padding([8, 0])
                                .selected(page.theme_mode.is_dark)
                                .on_press(Message::DarkMode(true)),
                                text::body(&descriptions[dark])
                            ]
                            .spacing(8)
                            .width(Length::FillPortion(1))
                            .align_x(Alignment::Center),
                            cosmic::iced::widget::column![
                                button::custom(
                                    icon(light_mode_illustration.clone(),)
                                        .width(Length::Fixed(191.0))
                                        .height(Length::Fixed(100.0))
                                )
                                .class(button::ButtonClass::Image)
                                .selected(!page.theme_mode.is_dark)
                                .padding([8, 0])
                                .on_press(Message::DarkMode(false)),
                                text::body(&descriptions[light])
                            ]
                            .spacing(8)
                            .width(Length::FillPortion(1))
                            .align_x(Alignment::Center)
                        ]
                        .spacing(8)
                        .width(Length::Fixed(478.0))
                        .align_y(Alignment::Center),
                    )
                    .center_x(Length::Fill),
                )
                .add(
                    settings::item::builder(&descriptions[auto_switch])
                        .description(
                            if !page.day_time && page.theme_mode.is_dark {
                                &page.auto_switch_descs[0]
                            } else if page.day_time && !page.theme_mode.is_dark {
                                &page.auto_switch_descs[1]
                            } else if page.day_time && page.theme_mode.is_dark {
                                &page.auto_switch_descs[2]
                            } else {
                                &page.auto_switch_descs[3]
                            }
                            .clone(),
                        )
                        .toggler(page.theme_mode.auto_switch, Message::Autoswitch),
                )
                .add(accent_color_palette)
                .add(
                    settings::item::builder(&descriptions[app_bg]).control(
                        page.application_background
                            .picker_button(Message::ApplicationBackground, Some(24))
                            .width(Length::Fixed(48.0))
                            .height(Length::Fixed(24.0)),
                    ),
                )
                .add(
                    settings::item::builder(&descriptions[container_bg])
                        .description(&descriptions[container_bg_desc])
                        .control(if page.container_background.get_applied_color().is_some() {
                            Element::from(
                                page.container_background
                                    .picker_button(Message::ContainerBackground, Some(24))
                                    .width(Length::Fixed(48.0))
                                    .height(Length::Fixed(24.0)),
                            )
                        } else {
                            container(
                                button::text(&descriptions[auto_txt])
                                    .trailing_icon(go_next_icon.clone())
                                    .on_press(Message::ContainerBackground(
                                        ColorPickerUpdate::ToggleColorPicker,
                                    )),
                            )
                            .into()
                        }),
                )
                .add(
                    settings::item::builder(&descriptions[text_tint])
                        .description(&descriptions[text_tint_desc])
                        .control(
                            page.interface_text
                                .picker_button(Message::InterfaceText, Some(24))
                                .width(Length::Fixed(48.0))
                                .height(Length::Fixed(24.0)),
                        ),
                )
                .add(
                    settings::item::builder(&descriptions[control_tint])
                        .description(&descriptions[control_tint_desc])
                        .control(
                            page.control_component
                                .picker_button(Message::ControlComponent, Some(24))
                                .width(Length::Fixed(48.0))
                                .height(Length::Fixed(24.0)),
                        ),
                )
                .add(
                    settings::item::builder(&descriptions[window_hint_toggle])
                        .toggler(page.no_custom_window_hint, Message::UseDefaultWindowHint),
                );
            if !page.no_custom_window_hint {
                section = section.add(
                    settings::item::builder(&descriptions[window_hint]).control(
                        page.accent_window_hint
                            .picker_button(Message::AccentWindowHint, Some(24))
                            .width(Length::Fixed(48.0))
                            .height(Length::Fixed(24.0)),
                    ),
                );
            }
            section
                .apply(Element::from)
                .map(crate::pages::Message::Appearance)
        })
}

/// A button for selecting a color or gradient.
pub fn color_button<'a, Message: 'a + Clone>(
    on_press: Option<Message>,
    color: cosmic::iced::Color,
    selected: bool,
    width: u16,
    height: u16,
) -> Element<'a, Message> {
    button::custom(color_image(
        wallpaper::Color::Single([color.r, color.g, color.b]),
        width,
        height,
        None,
    ))
    .padding(0)
    .selected(selected)
    .class(button::ButtonClass::Image)
    .on_press_maybe(on_press)
    .width(Length::Fixed(f32::from(width)))
    .height(Length::Fixed(f32::from(height)))
    .into()
}
