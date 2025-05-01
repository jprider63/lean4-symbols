use dioxus::prelude::*;
use dioxus_clipboard::prelude::use_clipboard;
use dioxus_free_icons::icons::bs_icons::*;
use dioxus_free_icons::icons::fa_brands_icons::*;
use dioxus_free_icons::Icon;
use std::sync::LazyLock;
use std::collections::BTreeMap;
use tracing;

// const FAVICON: Asset = asset!("/assets/favicon.ico");
const BOOTSTRAP_CSS: Asset = asset!("/assets/css/bootstrap.min.css");
const MAIN_CSS: Asset = asset!("/assets/css/main.css");

static MAPPING : LazyLock<(BTreeMap<String,String>, BTreeMap<String,String>)> = LazyLock::new(|| {
    let abbreviations = include_bytes!("../vscode-lean4/lean4-unicode-input/src/abbreviations.json");
    let abbreviations: serde_json::Value = serde_json::from_slice(abbreviations).expect("JSON was not well-formatted");
    let m = abbreviations.as_object().expect("Invalid JSON. Expected a dictionary.");

    let mut symbol_mapping = BTreeMap::<String, String>::new();
    let mut abbreviation_mapping = BTreeMap::<String, String>::new();
    for (abbr, symb) in m {
        let symb = symb.as_str().expect("Invalid JSON. Expected a string.").to_string();
        // Prioritize shorter abbreviations
        let should_insert = {
            if let Some(old_abbr) = symbol_mapping.get(&symb) {
                abbr.len() < old_abbr.len()
            } else {
                true
            }
        };
        if should_insert {
            let _ = symbol_mapping.insert(symb.clone(), abbr.clone());
        }
        let _ = abbreviation_mapping.insert(abbr.clone(), symb);
    }
    // tracing::debug!("{abbreviation_mapping:?}");
    (symbol_mapping, abbreviation_mapping)
});

fn main() {
    dioxus::launch(App);
}

/*
How do I write this symbol in Lean?

Symbol                 Abbreviation

Github icon
*/

#[component]
fn App() -> Element {
    rsx! {
        // document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: BOOTSTRAP_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div {
            class: "d-flex flex-column",
            style: "min-height: 100dvh;",
            main {
                class: "flex-shrink-0",
                div {
                    class: "container",
                    div {
                        class: "row",
                        div {
                            class: "col-md-12",
                            h1 {
                                // id: "title",
                                class: "display-1 text-center py-5",
                                "How do I write this symbol in Lean?"
                            }
                        }
                    }
                    Symbols {}
                }
            }
            footer {
                class: "footer mt-auto py-5",
                div {
                    class: "container",
                    div {
                        class: "row",
                        div {
                            class: "col-md-12 text-center",
                            a {
                                href: "https://github.com/jprider63/lean4-symbols",
                                Icon {
                                    class: "footer-icon",
                                    width: 48,
                                    height: 48,
                                    fill: "#212529",
                                    icon: FaGithub,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Symbols() -> Element {
    let mut symbol = use_signal(|| "".to_string());
    let mut abbreviation: Signal<String> = use_signal(|| "".to_string());
    let mut last_edit = use_signal(|| true); // true is symbol, false is abbreviation
    let mut clipboard = use_clipboard();

    let mut handle_symbol = move |e: Event<FormData>| {
        // tracing::debug!("{e:?}");
        let v = e.value();
        if let Some(abbr) = (*MAPPING).0.get(&v) {
            let mut r = "\\".to_string();
            r.push_str(abbr);
            // tracing::debug!("Some: {abbr:?}");
            abbreviation.set(r);
        } else {
            // tracing::debug!("None");
            abbreviation.set("".to_string());
        }
        symbol.set(v);
        last_edit.set(true);
    };
    let normalize_abbreviation = |mut s: String| {
        if s.chars().next() == Some('\\') {
            s.remove(0);
        }
        s
    };
    let mut handle_abbreviation = move |e: Event<FormData>| {
        // tracing::debug!("{e:?}");
        let v = normalize_abbreviation(e.value());

        if let Some(symb) = (*MAPPING).1.get(&v) {
            // tracing::debug!("Some: {symb:?}");
            symbol.set(symb.clone());
        } else {
            // tracing::debug!("None");
            symbol.set("".to_string());
        }
        abbreviation.set(e.value());
        last_edit.set(false);
    };



    let both_empty = use_memo(move || symbol() == "" && abbreviation() == "");

    let left_error = {
        use_memo(move || last_edit() && symbol() != "" && (*MAPPING).0.get(&symbol()).is_none())
    };
    let right_error = {
        use_memo(move || !last_edit() && abbreviation() != "" && (*MAPPING).1.get(&normalize_abbreviation(abbreviation())).is_none())
    };

    rsx! {
        div {
            class: "row",
            div {
                class: "col-md-4 offset-md-1 py-3 text-center",
                label {
                    for: "form-input-symbol",
                    class: "form-label h3",
                    "Symbol"
                }
                input {
                    type: "text",
                    id: "form-input-symbol",
                    class: format_args!("form-control form-control-lg {}", if left_error() {"is-invalid"} else {""}),
                    "aria-describedby": "form-input-symbol-feedback",
                    placeholder: if both_empty() {"→"} else {""},
                    oninput: move |evt| handle_symbol(evt),
                    // onkeyup: move |evt| handle_symbol(evt),
                    value: "{symbol}"
                }
                if left_error() {
                    div {
                        id: "form-input-symbol-feedback",
                        class: "invalid-feedback",
                        style: "text-align: left;",
                        "Unknown symbol"
                    }
                }
            }
            div {
                class: "col-md-4 offset-md-2 py-3 text-center",
                label {
                    for: "form-abbreviation-symbol",
                    class: "form-label h3",
                    "Abbreviation"
                }
                div {
                    class: format_args!("input-group {}", if right_error() {"has-validation"} else {""}),
                    input {
                        type: "text",
                        class: format_args!("form-control form-control-lg {}", if right_error() {"is-invalid"} else {""}),
                    "aria-describedby": "form-abbreviation-symbol-feedback",
                        placeholder: if both_empty() {"\\r"} else {""},
                        oninput: move |evt| handle_abbreviation(evt),
                        // onkeyup: move |evt| handle_abbreviation(evt),
                        value: "{abbreviation}"
                    }
                    button {
                        class: "btn btn-outline-secondary",
                        onclick: move |_| {
                            tracing::info!("HERE!");
                            if let Err(_) = clipboard.set(abbreviation()) {
                                tracing::info!("Failed to set clipboard");
                            }
                        },
                        Icon {
                            class: "copy-icon",
                            width: 24,
                            height: 24,
                            fill: "#212529",
                            icon: BsClipboard,
                        }
                    }
                    if right_error() {
                        div {
                            id: "form-input-abbreviation-feedback",
                            class: "invalid-feedback",
                            style: "text-align: left;",
                            "Unknown abbreviation"
                        }
                    }
                }
            }
        }
    }
}

