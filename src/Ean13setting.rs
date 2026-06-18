use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Ean13Settings {

    pub bar_width: f64,
    pub bar_height: f64,
    pub quiet_zone: f64,
    pub show_text: bool,
    pub foreground: String,
    pub background: String,
    pub font: String,
    pub text_size: f64,
    pub show_left_guard: bool,
    pub show_center_guard: bool,
    pub show_right_guard: bool,
    pub left_guard_extra: f64,
    pub center_guard_extra: f64,
    pub right_guard_extra: f64,
    pub text_between_guards: bool,
    pub text_between_spacing: f64,
}

impl Default for Ean13Settings {
    fn default() -> Self {
        Self {
            bar_width: 2.0,
            bar_height: 80.0,
            quiet_zone: 10.0,
            show_text: true,
            foreground: String::from("#000000"),
            background: String::from("#ffffff"),
            font: String::from("monospace"),
            text_size: 16.0,
            show_left_guard: true,
            show_center_guard: true,
            show_right_guard: true,
            left_guard_extra: 10.0,
            center_guard_extra: 10.0,
            right_guard_extra: 10.0,
            text_between_guards: false,
            text_between_spacing: 0.0,
        }
    }
}

/// Path to the persisted settings file (`<config_dir>/rust-ean13/settings.json`).
fn settings_path() -> Option<std::path::PathBuf> {
    let mut dir = dirs::config_dir()?;
    dir.push("rust-ean13");
    std::fs::create_dir_all(&dir).ok()?;
    dir.push("settings.json");
    Some(dir)
}

/// Load settings from local storage, falling back to defaults on any error.
pub fn load_settings() -> Ean13Settings {
    settings_path()
        .and_then(|path| std::fs::read_to_string(path).ok())
        .and_then(|contents| serde_json::from_str(&contents).ok())
        .unwrap_or_default()
}

/// Persist settings to local storage. Errors are ignored (best-effort save).
pub fn save_settings(settings: &Ean13Settings) {
    if let Some(path) = settings_path() {
        if let Ok(json) = serde_json::to_string_pretty(settings) {
            let _ = std::fs::write(path, json);
        }
    }
}

#[component]
pub fn Ean13Setting(on_setting_changed: EventHandler<Ean13Settings>) -> Element {
    // Load persisted settings once when the component mounts.
    let loaded = use_hook(load_settings);
    let mut data = use_signal(|| String::from("590123412345"));
    let mut bar_width = use_signal(|| loaded.bar_width);
    let mut bar_height = use_signal(|| loaded.bar_height);
    let mut quiet_zone = use_signal(|| loaded.quiet_zone);
    let mut show_text = use_signal(|| loaded.show_text);
    let mut foreground = use_signal(|| loaded.foreground.clone());
    let mut background = use_signal(|| loaded.background.clone());
    let mut font = use_signal(|| loaded.font.clone());
    let mut text_size = use_signal(|| loaded.text_size);
    let mut show_left_guard = use_signal(|| loaded.show_left_guard);
    let mut show_center_guard = use_signal(|| loaded.show_center_guard);
    let mut show_right_guard = use_signal(|| loaded.show_right_guard);
    let mut left_guard_extra = use_signal(|| loaded.left_guard_extra);
    let mut center_guard_extra = use_signal(|| loaded.center_guard_extra);
    let mut right_guard_extra = use_signal(|| loaded.right_guard_extra);
    let mut text_between_guards = use_signal(|| loaded.text_between_guards);
    let mut text_between_spacing = use_signal(|| loaded.text_between_spacing);

    let svg_height = use_memo(move || {
        let mut h = bar_height();
        if show_left_guard() { h = h.max(bar_height() + left_guard_extra()); }
        if show_center_guard() { h = h.max(bar_height() + center_guard_extra()); }
        if show_right_guard() { h = h.max(bar_height() + right_guard_extra()); }
        if show_text() && text_between_guards() { h = h.max(bar_height() + text_size() + 4.0); }
        h
    });

    let guard_extra = move |gt: u8| -> f64 {
        match gt {
            1 => left_guard_extra(),
            2 => center_guard_extra(),
            3 => right_guard_extra(),
            _ => 0.0,
        }
    };

    let settings = use_memo(move || Ean13Settings {
        bar_width: bar_width(),
        bar_height: bar_height(),
        quiet_zone: quiet_zone(),
        show_text: show_text(),
        foreground: foreground(),
        background: background(),
        font: font(),
        text_size: text_size(),
        show_left_guard: show_left_guard(),
        show_center_guard: show_center_guard(),
        show_right_guard: show_right_guard(),
        left_guard_extra: left_guard_extra(),
        center_guard_extra: center_guard_extra(),
        right_guard_extra: right_guard_extra(),
        text_between_guards: text_between_guards(),
        text_between_spacing: text_between_spacing(),
    });

    use_effect(move || {
        let current = settings();
        save_settings(&current);
        on_setting_changed.call(current);
    });

    // Recompute the rendered barcode whenever any setting changes.
    let render = use_memo(move || {
        build_barcode(
            &data(),
            bar_width(),
            quiet_zone(),
            show_left_guard(),
            show_center_guard(),
            show_right_guard(),
        )
    });

    rsx! {
        div {
            class: "ean13-setting",
            style: "display: flex; flex-direction: column; gap: 10px; min-height: 0;flex:9;",
            label { style: "font-size: large; font-weight: bold;", "EAN-13 Setting:" }

            // Preview
            div {
                class: "ean13-preview",
                style: "display: flex; flex-direction: column; align-items: center; gap: 4px; padding: 10px; border: 1px solid #444; border-radius: 5px; background: {background};",
                match render() {
                    Some(bc) => rsx! {
                        svg {
                            width: "{bc.total_w}",
                            height: "{svg_height()}",
                            view_box: "0 0 {bc.total_w} {svg_height()}",
                            rect {
                                x: 0,
                                y: 0,
                                width: "{bc.total_w}",
                                height: "{svg_height()}",
                                fill: "{background}",
                            }
                            for (x , w , guard_type) in bc.bars.iter().cloned() {
                                rect {
                                    x: "{x}",
                                    y: 0,
                                    width: "{w}",
                                    height: "{bar_height() + guard_extra(guard_type)}",
                                    fill: "{foreground}",
                                }
                            }
                            if show_text() && text_between_guards() {
                                // system digit — left of left guard
                                text {
                                    x: "{(quiet_zone() / 2.0) * bar_width()}",
                                    y: "{bar_height() + text_size()}",
                                    font_family: "{font()}",
                                    font_size: "{text_size()}",
                                    fill: "{foreground}",
                                    text_anchor: "middle",
                                    "{bc.display.chars().nth(0).unwrap_or(' ')}"
                                }
                                // left group: digits 1-6
                                for k in 0..6usize {
                                    text {
                                        x: "{(quiet_zone() + 6.0 + k as f64 * 7.0 - text_between_spacing() / 2.0) * bar_width()}",
                                        y: "{bar_height() + text_size()}",
                                        font_family: "{font()}",
                                        font_size: "{text_size()}",
                                        fill: "{foreground}",
                                        text_anchor: "middle",
                                        "{bc.display.chars().nth(k + 1).unwrap_or(' ')}"
                                    }
                                }
                                // right group: digits 7-12
                                for k in 0..6usize {
                                    text {
                                        x: "{(quiet_zone() + 53.0 + k as f64 * 7.0 + text_between_spacing() / 2.0) * bar_width()}",
                                        y: "{bar_height() + text_size()}",
                                        font_family: "{font()}",
                                        font_size: "{text_size()}",
                                        fill: "{foreground}",
                                        text_anchor: "middle",
                                        "{bc.display.chars().nth(k + 7).unwrap_or(' ')}"
                                    }
                                }
                            }
                        }
                        if show_text() && !text_between_guards() {
                            div { style: "font-family: {font()}; letter-spacing: 4px; color: {foreground}; font-size: {text_size()}px;",
                                "{bc.display}"
                            }
                        }
                    },
                    None => rsx! {
                        div { style: "color: #c0392b; padding: 20px; font-size: medium;",
                            "Enter at least 12 digits to preview an EAN-13 barcode."
                        }
                    },
                }
            }

            // Scrollable settings
            div {
                class: "ean13-settings-scroll",
                style: "display: flex; flex-direction: column; gap: 10px; overflow-y: auto; flex: 1; min-height: 0; padding-right: 6px;",

                div {
                    class: "setting-row",
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    label { style: "font-size: medium;", "Test Barcode data (digits)" }
                    input {
                        r#type: "text",
                        value: "{data}",
                        maxlength: 13,
                        oninput: move |e| data.set(e.value()),
                    }
                }

                div {
                    class: "setting-row",
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    label { style: "font-size: medium;", "Bar width (px)" }
                    input {
                        r#type: "number",
                        min: "1",
                        max: "10",
                        step: "0.5",
                        value: "{bar_width()}",
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<f64>() {
                                bar_width.set(v);
                            }
                        },
                    }
                }

                div {
                    class: "setting-row",
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    label { style: "font-size: medium;", "Bar height (px)" }
                    input {
                        r#type: "number",
                        min: "10",
                        max: "500",
                        step: "1",
                        value: "{bar_height()}",
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<f64>() {
                                bar_height.set(v);
                            }
                        },
                    }
                }

                div {
                    class: "setting-row",
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    label { style: "font-size: medium;", "Quiet zone (modules)" }
                    input {
                        r#type: "number",
                        min: "0",
                        max: "50",
                        step: "1",
                        value: "{quiet_zone()}",
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<f64>() {
                                quiet_zone.set(v);
                            }
                        },
                    }
                }

                div {
                    class: "setting-row",
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    label { style: "font-size: medium;", "Font" }
                    select {
                        value: "{font}",
                        oninput: move |e| font.set(e.value()),
                        option { value: "monospace", "Monospace" }
                        option { value: "Arial, sans-serif", "Arial" }
                        option { value: "\"Courier New\", monospace", "Courier New" }
                        option { value: "\"Times New Roman\", serif", "Times New Roman" }
                        option { value: "\"OCR-B\", monospace", "OCR-B" }
                        option { value: "Georgia, serif", "Georgia" }
                    }
                }

                div {
                    class: "setting-row",
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    label { style: "font-size: medium;", "Text size (px)" }
                    input {
                        r#type: "number",
                        min: "8",
                        max: "72",
                        step: "1",
                        value: "{text_size()}",
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<f64>() {
                                text_size.set(v);
                            }
                        },
                    }
                }

                div {
                    class: "setting-row",
                    style: "display: flex; flex-direction: row; align-items: center; gap: 8px;",
                    input {
                        r#type: "checkbox",
                        checked: show_text(),
                        oninput: move |e| show_text.set(e.checked()),
                    }
                    label { style: "font-size: medium;", "Show human-readable text" }
                }

                div {
                    class: "setting-row",
                    style: "display: flex; flex-direction: row; align-items: center; gap: 8px;",
                    input {
                        r#type: "checkbox",
                        checked: text_between_guards(),
                        oninput: move |e| text_between_guards.set(e.checked()),
                    }
                    label { style: "font-size: medium;", "Text between guards" }
                }

                if text_between_guards() {
                    div {
                        class: "setting-row",
                        style: "display: flex; flex-direction: column; gap: 4px;",
                        label { style: "font-size: medium;", "Text space between guards (modules)" }
                        input {
                            r#type: "number",
                            min: "-20",
                            max: "20",
                            step: "0.5",
                            value: "{text_between_spacing()}",
                            oninput: move |e| {
                                if let Ok(v) = e.value().parse::<f64>() {
                                    text_between_spacing.set(v);
                                }
                            },
                        }
                    }
                }

                // Guard toggles
                div {
                    class: "setting-row",
                    style: "display: flex; flex-direction: column; gap: 6px;",
                    label { style: "font-size: medium;", "Guards" }
                    // Left guard
                    div { style: "display: flex; flex-direction: row; align-items: center; gap: 8px;",
                        input {
                            r#type: "checkbox",
                            checked: show_left_guard(),
                            oninput: move |e| show_left_guard.set(e.checked()),
                        }
                        label { style: "font-size: medium; min-width: 52px;", "Left" }
                        if show_left_guard() {
                            label { style: "font-size: small;", "extra height" }
                            input {
                                r#type: "number",
                                min: "0",
                                max: "200",
                                step: "1",
                                value: "{left_guard_extra()}",
                                style: "width: 70px;",
                                oninput: move |e| {
                                    if let Ok(v) = e.value().parse::<f64>() {
                                        left_guard_extra.set(v);
                                    }
                                },
                            }
                        }
                    }
                    // Center guard
                    div { style: "display: flex; flex-direction: row; align-items: center; gap: 8px;",
                        input {
                            r#type: "checkbox",
                            checked: show_center_guard(),
                            oninput: move |e| show_center_guard.set(e.checked()),
                        }
                        label { style: "font-size: medium; min-width: 52px;", "Center" }
                        if show_center_guard() {
                            label { style: "font-size: small;", "extra height" }
                            input {
                                r#type: "number",
                                min: "0",
                                max: "200",
                                step: "1",
                                value: "{center_guard_extra()}",
                                style: "width: 70px;",
                                oninput: move |e| {
                                    if let Ok(v) = e.value().parse::<f64>() {
                                        center_guard_extra.set(v);
                                    }
                                },
                            }
                        }
                    }
                    // Right guard
                    div { style: "display: flex; flex-direction: row; align-items: center; gap: 8px;",
                        input {
                            r#type: "checkbox",
                            checked: show_right_guard(),
                            oninput: move |e| show_right_guard.set(e.checked()),
                        }
                        label { style: "font-size: medium; min-width: 52px;", "Right" }
                        if show_right_guard() {
                            label { style: "font-size: small;", "extra height" }
                            input {
                                r#type: "number",
                                min: "0",
                                max: "200",
                                step: "1",
                                value: "{right_guard_extra()}",
                                style: "width: 70px;",
                                oninput: move |e| {
                                    if let Ok(v) = e.value().parse::<f64>() {
                                        right_guard_extra.set(v);
                                    }
                                },
                            }
                        }
                    }
                }

                div {
                    class: "setting-row",
                    style: "display: flex; flex-direction: row; align-items: center; justify-content: space-between; gap: 8px;",
                    label { style: "font-size: medium;", "Bar color" }
                    input {
                        r#type: "color",
                        value: "{foreground}",
                        oninput: move |e| foreground.set(e.value()),
                    }
                }

                div {
                    class: "setting-row",
                    style: "display: flex; flex-direction: row; align-items: center; justify-content: space-between; gap: 8px;",
                    label { style: "font-size: medium;", "Background color" }
                    input {
                        r#type: "color",
                        value: "{background}",
                        oninput: move |e| background.set(e.value()),
                    }
                }
            }
        }
    }
}

/// Bars to draw for the rendered barcode, in pixels.
/// `guard_type`: 0 = data, 1 = left guard, 2 = center guard, 3 = right guard.
#[derive(Clone, PartialEq)]
pub struct BarcodeRender {
    pub bars: Vec<(f64, f64, u8)>,
    pub total_w: f64,
    pub display: String,
}

const L_CODES: [&str; 10] = [
    "0001101", "0011001", "0010011", "0111101", "0100011", "0110001", "0101111", "0111011",
    "0110111", "0001011",
];
const G_CODES: [&str; 10] = [
    "0100111", "0110011", "0011011", "0100001", "0011101", "0111001", "0000101", "0010001",
    "0001001", "0010111",
];
const R_CODES: [&str; 10] = [
    "1110010", "1100110", "1101100", "1000010", "1011100", "1001110", "1010000", "1000100",
    "1001000", "1110100",
];
const PARITY: [&str; 10] = [
    "LLLLLL", "LLGLGG", "LLGGLG", "LLGGGL", "LGLLGG", "LGGLLG", "LGGGLL", "LGLGLG", "LGLGGL",
    "LGGLGL",
];

fn ean13_check_digit(first12: &[u8]) -> u8 {
    let sum: u32 = first12
        .iter()
        .enumerate()
        .map(|(i, &d)| if i % 2 == 0 { d as u32 } else { d as u32 * 3 })
        .sum();
    ((10 - (sum % 10)) % 10) as u8
}

/// Encode the input into an EAN-13 module pattern and compute pixel bars.
///
/// Module layout (95 modules total):
///   0–2   : left guard  "101"
///   3–44  : left 6 digits (6 × 7 modules)
///   45–49 : center guard "01010"
///   50–91 : right 6 digits (6 × 7 modules)
///   92–94 : right guard "101"
pub fn build_barcode(
    input: &str,
    bar_width: f64,
    quiet_zone: f64,
    show_left_guard: bool,
    show_center_guard: bool,
    show_right_guard: bool,
) -> Option<BarcodeRender> {
    let digits: Vec<u8> = input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c as u8 - b'0')
        .collect();
    if digits.len() < 12 {
        return None;
    }

    let mut all: Vec<u8> = digits.into_iter().take(12).collect();
    let check = ean13_check_digit(&all);
    all.push(check);

    let first = all[0] as usize;
    let parity = PARITY[first].as_bytes();

    let mut bin = String::from("101"); // modules 0-2   : left guard
    for i in 0..6 {
        let d = all[1 + i] as usize;
        if parity[i] == b'L' {
            bin.push_str(L_CODES[d]);
        } else {
            bin.push_str(G_CODES[d]);
        }
    }
    bin.push_str("01010"); // modules 45-49 : center guard
    for i in 0..6 {
        let d = all[7 + i] as usize;
        bin.push_str(R_CODES[d]);
    }
    bin.push_str("101"); // modules 92-94 : right guard

    let bytes = bin.as_bytes();
    let n = bytes.len();
    let mut bars = Vec::new();
    let mut i = 0;
    while i < n {
        if bytes[i] == b'1' {
            let start = i;
            while i < n && bytes[i] == b'1' {
                i += 1;
            }
            let in_left_guard = start < 3;
            let in_center_guard = start >= 45 && start < 50;
            let in_right_guard = start >= 92;
            let guard_type: u8 = if in_left_guard { 1 } else if in_center_guard { 2 } else if in_right_guard { 3 } else { 0 };
            let skip = (in_left_guard && !show_left_guard)
                || (in_center_guard && !show_center_guard)
                || (in_right_guard && !show_right_guard);
            if !skip {
                let x = (quiet_zone + start as f64) * bar_width;
                let w = (i - start) as f64 * bar_width;
                bars.push((x, w, guard_type));
            }
        } else {
            i += 1;
        }
    }

    let total_w = (n as f64 + 2.0 * quiet_zone) * bar_width;
    let display: String = all.iter().map(|d| (d + b'0') as char).collect();

    Some(BarcodeRender {
        bars,
        total_w,
        display,
    })
}
