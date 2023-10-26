//! Utilities for [`bevy_egui`]` crate.

use bevy::prelude::*;

pub use bevy_egui::{egui, EguiContexts};

/// Popup window description
pub struct EguiPopup<'a> {
    /// Unique ID (unique only within current frame)
    pub name: &'a str,

    /// Draw frame and background or not
    pub background: bool,

    /// Which part of the screen to anchor to
    pub anchor: egui::Align2,

    /// In pixels, affected by UI scaling
    pub offset: Vec2,

    /// Draw order
    pub order: egui::Order,

    /// Receives input. If false, mouse events will go to UI elements with lower order
    pub interactable: bool,
}

impl Default for EguiPopup<'_> {
    fn default() -> Self {
        Self {
            name: default(),
            background: true,
            anchor: egui::Align2::CENTER_CENTER,
            offset: Vec2::ZERO,
            order: egui::Order::Middle,
            interactable: true,
        }
    }
}

/// Utilities for `EguiContext`
pub trait ExtendedBevyEguiContext {
    /// Draw popup window
    fn popup(&mut self, popup: EguiPopup, add_contents: impl FnOnce(&mut egui::Ui));
}

impl ExtendedBevyEguiContext for EguiContexts<'_, '_> {
    fn popup(&mut self, popup: EguiPopup, add_contents: impl FnOnce(&mut egui::Ui)) {
        egui::Area::new(popup.name.to_string())
            .anchor(popup.anchor, popup.offset.to_egui())
            .order(popup.order)
            .interactable(popup.interactable)
            .show(self.ctx_mut(), |ui| {
                if popup.background {
                    egui::Frame::popup(&ui.style()).show(ui, add_contents);
                } else {
                    (add_contents)(ui)
                }
            });
    }
}

/// Utilities for [`egui::Ui`]
pub trait ExtendedEguiUi {
    /// `ui.group()` which shows name as part of the frame.
    ///
    /// Name isn't required to be unique.
    fn named_group(&mut self, name: impl Into<String>, add_contents: impl FnOnce(&mut egui::Ui));

    /// Shortcut call
    fn scroll_area(&mut self, id: impl std::hash::Hash, add_contents: impl FnOnce(&mut egui::Ui));

    /// Shortcut call
    fn enabled_button(
        &mut self,
        enabled: bool,
        text: impl Into<egui::WidgetText>,
    ) -> egui::Response;

    /// Returns true if value changed.
    ///
    /// Shows options for blend and additive alpha.
    fn color_picker(&mut self, value: &mut Color) -> bool;

    /// Radio buttons for each value.
    ///
    /// Returns true if value changed
    fn radio_values<T: std::fmt::Debug + std::cmp::PartialEq>(
        &mut self,
        current: &mut T,
        values: impl Iterator<Item = T>,
    ) -> bool;

    /// Simple unbounded slider with prefix.
    ///
    /// Returns true if value changed
    fn slider<'a, Num: egui::emath::Numeric>(
        &mut self,
        name: &str,
        value: &'a mut Num,
        range: std::ops::RangeInclusive<Num>,
    ) -> bool;

    /// Barely working single-line text input for numbers.
    ///
    /// Returns true if value changed.
    fn number_edit(&mut self, value: &mut f32, precision: usize) -> bool;

    /// Shows slider-like bar with value and text
    fn number_view(&mut self, value: f32, min: f32, max: f32, text: &str);
}

impl ExtendedEguiUi for egui::Ui {
    fn named_group(&mut self, name: impl Into<String>, add_contents: impl FnOnce(&mut egui::Ui)) {
        // TODO: label should be on the frame, not inside of it
        self.group(|ui| {
            ui.label(egui::RichText::new(name).heading().strong());
            add_contents(ui);
        });
    }

    fn scroll_area(&mut self, id: impl std::hash::Hash, add_contents: impl FnOnce(&mut egui::Ui)) {
        egui::ScrollArea::both()
            .id_source(id)
            .show(self, add_contents);
    }

    fn enabled_button(
        &mut self,
        enabled: bool,
        text: impl Into<egui::WidgetText>,
    ) -> egui::Response {
        self.add_enabled(enabled, egui::Button::new(text))
    }

    fn color_picker(&mut self, value: &mut Color) -> bool {
        let mut color = value.to_egui();
        let changed = egui::color_picker::color_picker_color32(
            self,
            &mut color,
            egui::color_picker::Alpha::BlendOrAdditive,
        );

        if changed {
            *value = Color::from_egui(color);
        }
        changed
    }

    fn radio_values<T: std::fmt::Debug + std::cmp::PartialEq>(
        &mut self,
        current: &mut T,
        values: impl Iterator<Item = T>,
    ) -> bool {
        let mut changed = false;
        for value in values {
            let text = format!("{value:?}");
            if self.radio_value(current, value, text).changed() {
                changed = true;
            }
        }
        changed
    }

    fn slider<'a, Num: egui::emath::Numeric>(
        &mut self,
        name: &str,
        value: &'a mut Num,
        range: std::ops::RangeInclusive<Num>,
    ) -> bool {
        let mut slider = egui::Slider::new(value, range);
        if !name.is_empty() {
            slider = slider.prefix(format!("{name}: "))
        }
        self.add(slider).changed()
    }

    fn number_edit(&mut self, value: &mut f32, precision: usize) -> bool {
        let mut text = format!("{:.*}", precision, value);
        if self.text_edit_singleline(&mut text).changed() {
            if let Ok(new_value) = text.parse() {
                *value = new_value;
                return true;
            }
        }
        false
    }

    fn number_view(&mut self, value: f32, min: f32, max: f32, text: &str) {
        self.horizontal(|ui| {
            let text = format!("{text} {value:07.3}");
            ui.label(text);

            let mut t = ((value - min) / (max - min)).clamp(0., 1.);
            ui.add(egui::Slider::new(&mut t, 0. ..=1.).show_value(false));
        });
    }
}

/// Convert [`Color`]
pub trait BevyEguiColor {
    fn to_egui(self) -> egui::Color32;
    fn from_egui(color: egui::Color32) -> Self;
}

impl BevyEguiColor for Color {
    fn to_egui(self) -> egui::Color32 {
        let v = self.as_rgba_f32().map(|v| (v * 255.).clamp(0., 255.) as u8);
        egui::Color32::from_rgba_unmultiplied(v[0], v[1], v[2], v[3])
    }

    fn from_egui(color: egui::Color32) -> Self {
        let v = color.to_srgba_unmultiplied().map(|v| v as f32 / 255.);
        Color::rgba(v[0], v[1], v[2], v[3])
    }
}

/// Convert [`Vec2`]
pub trait BevyEguiVec2 {
    fn to_egui(self) -> egui::Vec2;
    fn to_egui_pos(self) -> egui::Pos2;
}

impl BevyEguiVec2 for Vec2 {
    fn to_egui(self) -> egui::Vec2 {
        egui::vec2(self.x, self.y)
    }
    fn to_egui_pos(self) -> egui::Pos2 {
        egui::pos2(self.x, self.y)
    }
}
