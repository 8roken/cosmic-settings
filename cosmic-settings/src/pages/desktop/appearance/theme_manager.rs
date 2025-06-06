use cosmic::cosmic_config::{Config, ConfigGet, ConfigSet, CosmicConfigEntry, Error};
use cosmic::cosmic_theme::palette::{FromColor, Hsv, Srgb, Srgba};
use cosmic::cosmic_theme::{
    CornerRadii, DARK_THEME_BUILDER_ID, LIGHT_THEME_BUILDER_ID, Spacing, Theme, ThemeBuilder,
    ThemeMode,
};
use cosmic::iced_core::Color;
use cosmic::widget::ColorPickerModel;

use super::Roundness;
use cosmic::config::CosmicTk;
use serde::Serialize;

pub struct Manager {
    theme: Theme,
    mode: ThemeMode,
    mode_config: Option<Config>,
    builder: ThemeBuilder,
    builder_config: Option<Config>,
    accent_palette: AccentPalette,
    custom_accent: Option<Srgb>,

    // temp for refactoring
    settings_config: crate::config::Config,
}

crate::cache_dynamic_lazy! {
    static HEX: String = fl!("hex");
    static RGB: String = fl!("rgb");
}

// pub members & Clone is only for refactoring (I think)
#[derive(Default, Clone)]
pub struct AccentPalette {
    pub dark: Option<Vec<Srgba>>,
    pub light: Option<Vec<Srgba>>,
    pub theme: Vec<Srgba>,
}

impl Default for Manager {
    fn default() -> Self {
        let settings_config = crate::config::Config::new();

        let theme_mode_config = ThemeMode::config().ok();
        let theme_mode = theme_mode_config
            .as_ref()
            .map(|c| match ThemeMode::get_entry(c) {
                Ok(t) => t,
                Err((errors, t)) => {
                    for e in errors {
                        tracing::error!("{e}");
                    }
                    t
                }
            })
            .unwrap_or_default();

        let accent_palette = AccentPalette {
            dark: settings_config.accent_palette_dark().ok(),
            light: settings_config.accent_palette_light().ok(),
            theme: Vec::new(),
        };

        let theme = if let Ok(c) = if theme_mode.is_dark {
            Theme::dark_config()
        } else {
            Theme::light_config()
        } {
            Theme::get_entry(&c).unwrap_or_default()
        } else {
            if theme_mode.is_dark {
                Theme::dark_default()
            } else {
                Theme::light_default()
            }
        };

        let theme_builder_config = if theme_mode.is_dark {
            ThemeBuilder::dark_config()
        } else {
            ThemeBuilder::light_config()
        }
        .ok();

        let mut theme_builder = theme_builder_config.as_ref().map_or_else(
            || {
                if theme_mode.is_dark {
                    ThemeBuilder::dark()
                } else {
                    ThemeBuilder::light()
                }
            },
            |c| match ThemeBuilder::get_entry(c) {
                Ok(t) => t,
                Err((errors, t)) => {
                    for e in errors {
                        tracing::error!("{e}");
                    }
                    t
                }
            },
        );

        theme_builder = theme_builder
            .clone()
            .accent(theme.accent.base.color)
            .bg_color(theme.bg_color())
            .corner_radii(theme.corner_radii)
            .destructive(theme.destructive.base.color)
            .spacing(theme.spacing)
            .success(theme.success.base.color)
            .warning(theme.warning.base.color)
            .neutral_tint(theme.palette.neutral_5.color)
            .text_tint(theme.background.on.color);

        theme_builder.gaps = theme.gaps;

        let custom_accent = theme_builder.accent.filter(|c| {
            let c = Srgba::new(c.red, c.green, c.blue, 1.0);
            c != theme.palette.accent_blue
                && c != theme.palette.accent_green
                && c != theme.palette.accent_indigo
                && c != theme.palette.accent_orange
                && c != theme.palette.accent_pink
                && c != theme.palette.accent_purple
                && c != theme.palette.accent_red
                && c != theme.palette.accent_warm_grey
                && c != theme.palette.accent_yellow
        });

        let tk_config = CosmicTk::config().ok();

        Self {
            mode: theme_mode,
            mode_config: theme_mode_config,
            builder_config: theme_builder_config,
            builder: theme_builder,
            custom_accent,
            theme,
            accent_palette,
            settings_config,
        }
    }
}

impl Manager {
    pub fn replace_theme(&mut self, theme: Theme) {
        self.theme = theme;
        let mut builder = self
            .builder
            .clone()
            .accent(self.theme.accent.base.color)
            .bg_color(self.theme.bg_color())
            .corner_radii(self.theme.corner_radii)
            .destructive(self.theme.destructive.base.color)
            .spacing(self.theme.spacing)
            .success(self.theme.success.base.color)
            .warning(self.theme.warning.base.color)
            .neutral_tint(self.theme.palette.neutral_5.color)
            .text_tint(self.theme.background.on.color);
        builder.gaps = self.theme.gaps;

        self.builder = builder;
    }

    pub fn set_builder(&mut self, builder: ThemeBuilder) {
        self.builder = builder;
        if let Some(config) = self.builder_config.as_ref() {
            _ = self.builder.write_entry(config);
        };
    }

    #[inline]
    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    #[inline]
    pub fn mode(&self) -> &ThemeMode {
        &self.mode
    }

    #[inline]
    pub fn builder(&self) -> &ThemeBuilder {
        &self.builder
    }

    pub fn builder_mut(&mut self) -> &mut ThemeBuilder {
        &mut self.builder
    }

    pub fn accent_palette(&self) -> &AccentPalette {
        &self.accent_palette
    }

    pub fn custom_accent(&self) -> &Option<Srgb> {
        &self.custom_accent
    }

    pub fn theme_mode_config(&self) -> &Option<Config> {
        &self.mode_config
    }

    pub fn builder_config(&self) -> &Option<Config> {
        &self.builder_config
    }

    pub fn dark_mode(&mut self, enabled: bool) -> Result<bool, cosmic_config::Error> {
        if let Some(config) = self.mode_config.as_ref() {
            return self.mode.set_is_dark(config, enabled);
        }
        Ok(true)
    }

    pub fn auto_switch(&mut self, enabled: bool) {
        self.mode.auto_switch = enabled;

        if let Some(config) = self.mode_config.as_ref() {
            _ = config.set::<bool>("auto_switch", enabled);
        }
    }

    pub fn set_window_hint(&mut self, color: Option<Srgb>) -> Option<bool> {
        let config = self.builder_config.as_ref()?;

        self.builder.set_window_hint(config, color).ok()
    }

    pub fn set_bg_color(&mut self, color: Option<Color>) -> Option<bool> {
        let config = self.builder_config.as_ref()?;

        self.builder
            .set_bg_color(config, color.map(Srgba::from))
            .ok()
    }

    pub fn set_primary_container_bg(&mut self, color: Option<Color>) -> Option<bool> {
        let config = self.builder_config.as_ref()?;

        self.builder
            .set_primary_container_bg(config, color.map(Srgba::from))
            .ok()
    }

    pub fn set_accent(&mut self, color: Option<Color>) -> Option<bool> {
        let config = self.builder_config.as_ref()?;

        self.builder.set_accent(config, color.map(Srgb::from)).ok()
    }

    pub fn set_text_tint(&mut self, color: Option<Color>) -> Option<bool> {
        let config = self.builder_config.as_ref()?;

        self.builder.set_accent(config, color.map(Srgb::from)).ok()
    }

    pub fn set_neutral_tint(&mut self, color: Option<Color>) -> Option<bool> {
        let config = self.builder_config.as_ref()?;

        self.builder
            .set_neutral_tint(config, color.map(Srgb::from))
            .ok()
    }

    pub fn set_spacing(&mut self, spacing: Spacing) -> Option<bool> {
        let config = self.builder_config.as_ref()?;

        self.builder.set_spacing(config, spacing).ok()
    }

    pub fn set_corner_radii(&mut self, corner_radii: CornerRadii) -> Option<bool> {
        let config = self.builder_config.as_ref()?;

        self.builder.set_corner_radii(config, corner_radii).ok()
    }

    pub fn set_gap_size(&mut self, gap: u32) -> Option<bool> {
        let mut limited_by_active_hint = false;
        let mut gaps = self.builder.gaps;

        let Some(config) = self.builder_config.as_ref() else {
            return None;
        };

        // Ensure that the gap is never less than what the active hint size is.
        gaps.1 = if gap < self.builder.active_hint {
            limited_by_active_hint = true;
            self.builder.active_hint
        } else {
            gap
        };

        if let Err(err) = self.builder.set_gaps(config, gaps) {
            tracing::error!(?err, "Error setting the gap");
            return None;
        }

        self.theme_config_write("gaps", gaps);
        Some(limited_by_active_hint)
    }

    pub fn set_active_hint(&mut self, active_hint: u32) -> Option<bool> {
        let mut changed_gap = false;

        let Some(config) = self.builder_config.as_ref() else {
            return None;
        };

        if let Err(err) = self.builder.set_active_hint(config, active_hint) {
            tracing::error!(?err, "Error setting the active hint");
            return None;
        }

        // Update the gap if it's less than the active hint
        if active_hint > self.builder.gaps.1 {
            let mut gaps = self.builder.gaps;
            gaps.1 = active_hint;
            if self.builder.set_gaps(config, gaps).unwrap_or_default() {
                self.theme_config_write("gaps", gaps);
            }
            changed_gap = true
        }

        // Update the active_hint in the config
        self.theme_config_write("active_hint", active_hint);

        Some(changed_gap)
    }

    pub fn reset_theme(&mut self) {
        self.builder = if self.mode().is_dark {
            cosmic::cosmic_config::Config::system(DARK_THEME_BUILDER_ID, ThemeBuilder::VERSION)
                .map_or_else(
                    |_| ThemeBuilder::dark(),
                    |config| match ThemeBuilder::get_entry(&config) {
                        Ok(t) => t,
                        Err((errs, t)) => {
                            for err in errs {
                                tracing::warn!(?err, "Error getting system theme builder");
                            }
                            t
                        }
                    },
                )
        } else {
            cosmic::cosmic_config::Config::system(LIGHT_THEME_BUILDER_ID, ThemeBuilder::VERSION)
                .map_or_else(
                    |_| ThemeBuilder::light(),
                    |config| match ThemeBuilder::get_entry(&config) {
                        Ok(t) => t,
                        Err((errs, t)) => {
                            for err in errs {
                                tracing::warn!(?err, "Error getting system theme builder");
                            }
                            t
                        }
                    },
                )
        };
    }

    pub fn update_accent_palette(&mut self) {
        let palette = self.builder.palette.as_ref();
        self.accent_palette.theme = vec![
            palette.accent_blue,
            palette.accent_indigo,
            palette.accent_purple,
            palette.accent_pink,
            palette.accent_red,
            palette.accent_orange,
            palette.accent_yellow,
            palette.accent_green,
            palette.accent_warm_grey,
        ];
    }

    fn theme_config_write<T: Serialize>(&self, name: &str, value: T) {
        let config_res = if self.mode.is_dark {
            Theme::dark_config()
        } else {
            Theme::light_config()
        };

        if let Ok(config) = config_res {
            _ = config.set(name, value);
        }
    }
}
