use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

mod Ean13setting;
use Ean13setting::Ean13Setting;

mod Ean13Export;
use Ean13Export::{export_ean13, ExportFormat, ExportProgress};

use crate::Ean13setting::load_settings;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Title { "EAN13 Barcode Generator" }
        MainApp {}

    }
}

#[component]
pub fn MainApp() -> Element {
    let mut input_text = use_signal(|| String::new());
    let mut ean13_settings = use_signal(|| load_settings());
    let mut export_format = use_signal(|| ExportFormat::Svg);
    let export_progress = use_signal(|| None::<ExportProgress>);
    let mut export_cancel = use_signal(|| false);
    rsx! {
        div { id: "hero", style: "display: flex; gap: 20px; height: 100vh;",
            div { "EAN13" }
            div {
                id: "links",
                style: "display: flex; flex-direction: row; gap: 10px; flex: 1; min-height: 0;margin-bottom: 30px;",
                TextAreaBox { on_submit: move |text| input_text.set(text) }
                div { style: "display: flex; flex-direction: column; gap: 10px;flex:1;",
                    Ean13Setting { on_setting_changed: move |settings| ean13_settings.set(settings) }
                    div { style: "display: flex; flex-direction: row; align-items: center; gap: 8px;",
                        label { style: "font-size: medium;", "Export format" }
                        select {
                            style: "flex: 1; height: 30px;",
                            value: match export_format() {
                                ExportFormat::Svg => "svg",
                                ExportFormat::Pdf => "pdf",
                            },
                            oninput: move |e| {
                                let fmt = if e.value() == "pdf" { ExportFormat::Pdf } else { ExportFormat::Svg };
                                export_format.set(fmt);
                            },
                            option { value: "svg", "SVG" }
                            option { value: "pdf", "PDF" }
                        }
                    }
                    button {
                        style: "height:40px;;",
                        onclick: move |_| {
                            let input = input_text();
                            let settings = ean13_settings();
                            let format = export_format();
                            spawn(async move {
                                export_ean13(input, settings, format, export_progress, export_cancel).await;
                            });
                        },
                        "Export"
                    }
                }
            
            }
        }

        {
            if let Some(p) = export_progress() {
                let percent = (p.current as f64 / p.total.max(1) as f64) * 100.0;
                rsx! {
                    div { style: "position: fixed; inset: 0; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 1000;",
                        div { style: "background: #ffffff; color: #000000; padding: 24px; border-radius: 8px; min-width: 320px; display: flex; flex-direction: column; gap: 16px; box-shadow: 0 8px 24px rgba(0,0,0,0.3);",
                            div { style: "font-weight: bold;", "Exporting barcodes…" }
                            div { style: "width: 100%; height: 18px; background: #e0e0e0; border-radius: 9px; overflow: hidden;",
                                div { style: "height: 100%; background: #2ecc71; width: {percent}%; transition: width 0.1s;" }
                            }
                            div { style: "font-size: small;", "{p.current} / {p.total}" }
                            button {
                                style: "align-self: flex-end; padding: 6px 16px;",
                                onclick: move |_| export_cancel.set(true),
                                "Cancel"
                            }
                        }
                    }
                }
            } else {
                rsx! {}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct TextInputProps {
    on_submit: EventHandler<String>,
}
#[component]
fn TextAreaBox(props: TextInputProps) -> Element {
    let mut long_text = use_signal(|| String::from(""));

    rsx! {
        div {
            class: "text-panel",
            style: "display: flex; flex: 1; min-height: 0;",
            textarea {
                style: "width: 100%; height: 100%;resize: none;",
                rows: 5,
                cols: 40,
                value: "{long_text}",
                oninput: move |event| {
                    long_text.set(event.value().clone());
                    props.on_submit.call(event.value().clone());
                },
                placeholder: "Enter multiple lines of text...",
            }
        }
    }
}
