use std::path::PathBuf;

use dioxus::prelude::*;

use crate::Ean13setting::{build_barcode, Ean13Settings};

/// Progress information shown in the export modal.
#[derive(Clone, Copy, PartialEq)]
pub struct ExportProgress {
    pub current: usize,
    pub total: usize,
}

/// Build a standalone SVG document for a single barcode line.
/// Returns `None` when the line does not contain a valid EAN-13 payload.
pub fn generate_svg(line: &str, s: &Ean13Settings) -> Option<String> {
    let bc = build_barcode(
        line,
        s.bar_width,
        s.quiet_zone,
        s.show_left_guard,
        s.show_center_guard,
        s.show_right_guard,
    )?;

    let guard_extra = |gt: u8| -> f64 {
        match gt {
            1 => s.left_guard_extra,
            2 => s.center_guard_extra,
            3 => s.right_guard_extra,
            _ => 0.0,
        }
    };

    // Bar/area height — mirrors the preview's `svg_height` memo.
    let mut bar_area = s.bar_height;
    if s.show_left_guard {
        bar_area = bar_area.max(s.bar_height + s.left_guard_extra);
    }
    if s.show_center_guard {
        bar_area = bar_area.max(s.bar_height + s.center_guard_extra);
    }
    if s.show_right_guard {
        bar_area = bar_area.max(s.bar_height + s.right_guard_extra);
    }
    if s.show_text && s.text_between_guards {
        bar_area = bar_area.max(s.bar_height + s.text_size + 4.0);
    }

    let below_text = s.show_text && !s.text_between_guards;
    let total_h = if below_text {
        bar_area + s.text_size + 8.0
    } else {
        bar_area
    };

    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\">",
        w = bc.total_w,
        h = total_h
    ));
    svg.push_str(&format!(
        "<rect x=\"0\" y=\"0\" width=\"{w}\" height=\"{h}\" fill=\"{bg}\"/>",
        w = bc.total_w,
        h = total_h,
        bg = s.background
    ));

    for (x, w, gt) in bc.bars.iter().cloned() {
        let bh = s.bar_height + guard_extra(gt);
        svg.push_str(&format!(
            "<rect x=\"{x}\" y=\"0\" width=\"{w}\" height=\"{bh}\" fill=\"{fg}\"/>",
            x = x,
            w = w,
            bh = bh,
            fg = s.foreground
        ));
    }

    if s.show_text {
        if s.text_between_guards {
            let chars: Vec<char> = bc.display.chars().collect();
            let y = s.bar_height + s.text_size;
            // System digit, left of the left guard.
            let sx = (s.quiet_zone / 2.0) * s.bar_width;
            svg.push_str(&text_el(sx, y, chars.first().copied().unwrap_or(' '), s));
            // Left group: digits 1-6.
            for k in 0..6usize {
                let x = (s.quiet_zone + 6.0 + k as f64 * 7.0 - s.text_between_spacing / 2.0) * s.bar_width;
                svg.push_str(&text_el(x, y, chars.get(k + 1).copied().unwrap_or(' '), s));
            }
            // Right group: digits 7-12.
            for k in 0..6usize {
                let x = (s.quiet_zone + 53.0 + k as f64 * 7.0 + s.text_between_spacing / 2.0) * s.bar_width;
                svg.push_str(&text_el(x, y, chars.get(k + 7).copied().unwrap_or(' '), s));
            }
        } else {
            let y = bar_area + s.text_size;
            let x = bc.total_w / 2.0;
            svg.push_str(&format!(
                "<text x=\"{x}\" y=\"{y}\" font-family='{font}' font-size=\"{fs}\" fill=\"{fg}\" text-anchor=\"middle\" letter-spacing=\"4\">{txt}</text>",
                x = x,
                y = y,
                font = s.font,
                fs = s.text_size,
                fg = s.foreground,
                txt = bc.display
            ));
        }
    }

    svg.push_str("</svg>");
    Some(svg)
}

fn text_el(x: f64, y: f64, ch: char, s: &Ean13Settings) -> String {
    format!(
        "<text x=\"{x}\" y=\"{y}\" font-family='{font}' font-size=\"{fs}\" fill=\"{fg}\" text-anchor=\"middle\">{ch}</text>",
        x = x,
        y = y,
        font = s.font,
        fs = s.text_size,
        fg = s.foreground,
        ch = ch
    )
}

/// Export every non-empty line of `input_text` as an SVG file.
///
/// 1. Opens a native folder picker (folders only).
/// 2. Shows a progress modal (driven by `progress`) with cancel support (`cancel`).
/// 3. Opens the destination folder in the OS file browser when finished.
pub async fn export_ean13(
    input_text: String,
    settings: Ean13Settings,
    mut progress: Signal<Option<ExportProgress>>,
    mut cancel: Signal<bool>,
) {
    let lines: Vec<String> = input_text
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();

    if lines.is_empty() {
        return;
    }

    // 1. Folder picker — folders only.
    let folder = rfd::AsyncFileDialog::new()
        .set_title("Select export folder")
        .pick_folder()
        .await;
    let Some(folder) = folder else {
        return;
    };
    let dest: PathBuf = folder.path().to_path_buf();

    // 2. Show the progress modal.
    cancel.set(false);
    let total = lines.len();
    progress.set(Some(ExportProgress { current: 0, total }));

    for (idx, line) in lines.iter().enumerate() {
        if cancel() {
            break;
        }
        if let Some(svg) = generate_svg(line, &settings) {
            let digits: String = line.chars().filter(|c| c.is_ascii_digit()).collect();
            let stem = if digits.is_empty() { "barcode".to_string() } else { digits };
            let name = format!("{:03}_{}.svg", idx + 1, stem);
            let _ = std::fs::write(dest.join(name), svg);
        }
        progress.set(Some(ExportProgress { current: idx + 1, total }));
        // Yield so the UI can repaint the progress bar between files.
        tokio::time::sleep(std::time::Duration::from_millis(30)).await;
    }

    let was_cancelled = cancel();
    progress.set(None);

    // 3. Reveal the destination folder when the export completed.
    if !was_cancelled {
        let _ = open::that(&dest);
    }
}
