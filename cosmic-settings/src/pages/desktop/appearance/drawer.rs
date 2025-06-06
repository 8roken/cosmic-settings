use cosmic::Task;
use cosmic::cosmic_theme::palette::{FromColor, Hsv, Srgb, Srgba};
use cosmic::widget::{ColorPickerModel, color_picker::ColorPickerUpdate};

use crate::app;

use super::{ContextView, theme_manager};

pub struct Content {
    pub custom_accent: ColorPickerModel,
    pub accent_window_hint: ColorPickerModel,
    pub application_background: ColorPickerModel,
    pub container_background: ColorPickerModel,
    pub interface_text: ColorPickerModel,
    pub control_component: ColorPickerModel,
}

crate::cache_dynamic_lazy! {
    static HEX: String = fl!("hex");
    static RGB: String = fl!("rgb");
}

impl From<&theme_manager::Manager> for Content {
    fn from(theme_manager: &theme_manager::Manager) -> Self {
        let theme = theme_manager.theme();
        Self {
            custom_accent: ColorPickerModel::new(
                &*HEX,
                &*RGB,
                None,
                theme_manager.get_color(&ContextView::CustomAccent),
            ),
            application_background: ColorPickerModel::new(
                &*HEX,
                &*RGB,
                Some(theme.background.base.into()),
                theme_manager.get_color(&ContextView::ApplicationBackground),
            ),
            container_background: ColorPickerModel::new(
                &*HEX,
                &*RGB,
                None,
                theme_manager.get_color(&ContextView::ContainerBackground),
            ),
            interface_text: ColorPickerModel::new(
                &*HEX,
                &*RGB,
                Some(theme.background.on.into()),
                theme_manager.get_color(&ContextView::InterfaceText),
            ),
            control_component: ColorPickerModel::new(
                &*HEX,
                &*RGB,
                Some(theme.palette.neutral_5.into()),
                theme_manager.get_color(&ContextView::ControlComponent),
            ),
            accent_window_hint: ColorPickerModel::new(
                &*HEX,
                &*RGB,
                None,
                theme_manager.get_color(&ContextView::AccentWindowHint),
            ),
        }
    }
}

impl Content {
    pub fn reset(&mut self, manager: &theme_manager::Manager) -> Task<app::Message> {
        let mut tasks = Vec::new();
        tasks.push(
            self.application_background
                .update::<app::Message>(ColorPickerUpdate::ActiveColor(Hsv::from_color(
                    manager
                        .get_color(&ContextView::ApplicationBackground)
                        .map(Srgb::from)
                        .unwrap_or_default(),
                ))),
        );
        tasks.push(
            self.application_background
                .update::<app::Message>(ColorPickerUpdate::AppliedColor),
        );

        tasks.push(
            self.accent_window_hint
                .update::<app::Message>(ColorPickerUpdate::ActiveColor(Hsv::from_color(
                    manager
                        .get_color(&ContextView::AccentWindowHint)
                        .map(Srgb::from)
                        .unwrap_or_default(),
                ))),
        );
        tasks.push(
            self.accent_window_hint
                .update::<app::Message>(ColorPickerUpdate::AppliedColor),
        );

        tasks.push(
            self.custom_accent
                .update::<app::Message>(ColorPickerUpdate::ActiveColor(Hsv::from_color(
                    manager
                        .get_color(&ContextView::CustomAccent)
                        .map(Srgb::from)
                        .unwrap_or_default(),
                ))),
        );
        tasks.push(
            self.custom_accent
                .update::<app::Message>(ColorPickerUpdate::AppliedColor),
        );

        tasks.push(
            self.container_background
                .update::<app::Message>(ColorPickerUpdate::ActiveColor(Hsv::from_color(
                    manager
                        .get_color(&ContextView::ContainerBackground)
                        .map(Srgb::from)
                        .unwrap_or_default(),
                ))),
        );
        tasks.push(
            self.container_background
                .update::<app::Message>(ColorPickerUpdate::AppliedColor),
        );

        tasks.push(
            self.interface_text
                .update::<app::Message>(ColorPickerUpdate::ActiveColor(Hsv::from_color(
                    manager
                        .get_color(&ContextView::InterfaceText)
                        .map(Srgb::from)
                        .unwrap_or_default(),
                ))),
        );
        tasks.push(
            self.interface_text
                .update::<app::Message>(ColorPickerUpdate::AppliedColor),
        );

        tasks.push(
            self.control_component
                .update::<app::Message>(ColorPickerUpdate::ActiveColor(Hsv::from_color(
                    manager
                        .get_color(&ContextView::ControlComponent)
                        .map(Srgb::from)
                        .unwrap_or_default(),
                ))),
        );
        tasks.push(
            self.control_component
                .update::<app::Message>(ColorPickerUpdate::AppliedColor),
        );

        cosmic::Task::batch(tasks)
    }
}
