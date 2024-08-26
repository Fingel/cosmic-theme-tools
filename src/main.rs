use crate::palette::{rgb::Rgb, rgb::Rgba, Srgba};
use cosmic_theme::*;
use std::io;

pub fn to_rgba(c: Srgba) -> String {
    let c_u8: Rgba<palette::encoding::Srgb, u8> = c.into_format();
    format!(
        "rgba({}, {}, {}, {:1.2})",
        c_u8.red,
        c_u8.green,
        c_u8.blue,
        c.alpha * 255.0
    )
}

pub fn rgb_to_rgba(c: Rgb) -> String {
    format!(
        "rgba({}, {}, {}, 255)",
        c.red * 255.0,
        c.green * 255.0,
        c.blue * 255.0
    )
}

pub fn component_css(prefix: &str, c: &Component) -> String {
    format!(
        r#"
--{prefix}-color: {};
--{prefix}-bg-color: {};
--{prefix}-fg-color: {};"#,
        to_rgba(c.base),
        to_rgba(c.base),
        to_rgba(c.on),
    )
}
pub fn as_css_vars(theme: &Theme) -> String {
    let active_hint = theme.active_hint;
    let bg_component_color = to_rgba(theme.bg_component_color());
    let bg_component_divider = to_rgba(theme.bg_component_divider());
    let bg_color = to_rgba(theme.bg_color());
    let bg_divider = to_rgba(theme.bg_divider());
    let button_base = to_rgba(theme.button.base);
    let button_border = to_rgba(theme.button.border);
    let button_disabled = to_rgba(theme.button.disabled);
    let button_disabled_border = to_rgba(theme.button.disabled_border);
    let button_divider = to_rgba(theme.button.divider);
    let button_focus = to_rgba(theme.button.focus);
    let button_hover = to_rgba(theme.button.hover);
    let button_on = to_rgba(theme.button.on);
    let button_on_disabled = to_rgba(theme.button.on_disabled);
    let button_pressed = to_rgba(theme.button.pressed);
    let button_selected = to_rgba(theme.button.selected);
    let button_selected_text = to_rgba(theme.button.selected_text);
    let button_focus_color = to_rgba(theme.button.focus_color());
    let corner_radius = theme.corner_radii.radius_m[0];
    let is_dark = if theme.is_dark { "1" } else { "0" };
    let is_frosted = if theme.is_frosted { "1" } else { "0" };
    let is_high_contrast = if theme.is_high_contrast { "1" } else { "0" };
    let on_bg_color = to_rgba(theme.on_bg_color());
    let on_bg_component_color = to_rgba(theme.on_bg_component_color());
    let on_primary_container_color = to_rgba(theme.on_primary_container_color());
    let on_primary_component_color = to_rgba(theme.on_primary_component_color());
    let on_secondary_container_color = to_rgba(theme.on_secondary_container_color());
    let on_secondary_component_color = to_rgba(theme.on_secondary_component_color());
    let accent_text_color = to_rgba(theme.accent_text_color());
    let success_text_color = to_rgba(theme.success_text_color());
    let warning_text_color = to_rgba(theme.warning_text_color());
    let destructive_text_color = to_rgba(theme.destructive_text_color());
    let on_accent_color = to_rgba(theme.on_accent_color());
    let on_success_color = to_rgba(theme.on_success_color());
    let on_warning_color = to_rgba(theme.on_warning_color());
    let on_destructive_color = to_rgba(theme.on_destructive_color());
    let primary_container_color = to_rgba(theme.primary_container_color());
    let primary_container_divider = to_rgba(theme.primary_container_divider());
    let primary_component_color = to_rgba(theme.primary_component_color());
    let primary_component_divider = to_rgba(theme.primary_component_divider());
    let radius_m = theme.radius_m()[0];
    let secondary_component_color = to_rgba(theme.secondary_component_color());
    let secondary_container_color = to_rgba(theme.secondary_container_color());
    let secondary_container_divider = to_rgba(theme.secondary_container_divider());
    let window_hint = rgb_to_rgba(
        theme
            .window_hint
            .unwrap_or_else(|| theme.accent_color().color),
    );
    let mut css = format! {
r#"--active-hint: {active_hint}px;
--bg-component-color: {bg_component_color};
--bg-component-divider: {bg_component_divider};
--bg-color: {bg_color};
--bg-divider: {bg_divider};
--button-base: {button_base};
--button-divider: {button_divider};
--button-border: {button_border};
--button-disabled: {button_disabled};
--button-disabled-border: {button_disabled_border};
--button-divider: {button_divider};
--button-focus: {button_focus};
--button-hover: {button_hover};
--button-on: {button_on};
--button-on-disabled: {button_on_disabled};
--button-pressed: {button_pressed};
--button-selected: {button_selected};
--button-selected-text: {button_selected_text};
--button-focus-color: {button_focus_color};
--corner-radius: {corner_radius}px;
--is-dark: {is_dark};
--is-frosted: {is_frosted};
--is-high-contrast: {is_high_contrast};
--on-bg-color: {on_bg_color};
--on-bg-component-color: {on_bg_component_color};
--on-primary-container-color: {on_primary_container_color};
--on-primary-component-color: {on_primary_component_color};
--on-secondary-container-color: {on_secondary_container_color};
--on-secondary-component-color: {on_secondary_component_color};
--accent-text-color: {accent_text_color};
--success-text-color: {success_text_color};
--warning-text-color: {warning_text_color};
--destructive-text-color: {destructive_text_color};
--on-accent-color: {on_accent_color};
--on-success-color: {on_success_color};
--on-warning-color: {on_warning_color};
--on-destructive-color: {on_destructive_color};
--primary-container-color: {primary_container_color};
--primary-container-divider: {primary_container_divider};
--primary-component-color: {primary_component_color};
--primary-component-divider: {primary_component_divider};
--radius-m: {radius_m}px;
--secondary-component-color: {secondary_component_color};
--secondary-container-color: {secondary_container_color};
--secondary-container-divider: {secondary_container_divider};
--window-hint: {window_hint};"#};

    css.push_str(&component_css("accent", &theme.accent));
    css.push_str(&component_css("destructive", &theme.destructive));
    css.push_str(&component_css("warning", &theme.warning));
    css.push_str(&component_css("success", &theme.success));
    css.push_str(&component_css("icon-button", &theme.icon_button));
    css.push_str(&component_css("link-button", &theme.link_button));
    css.push_str(&component_css("success-button", &theme.success_button));
    css.push_str(&component_css("text-button", &theme.text_button));
    css.push_str(&component_css("warning-button", &theme.warning_button));

    css
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let theme_cfg: ThemeBuilder = ron::from_str(&input).expect("Failed to parse theme");
    let theme = theme_cfg.build();
    println!("{}", as_css_vars(&theme));
}
