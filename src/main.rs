use bimap::BiMap;
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::*;
use dioxus_free_icons::Icon;
use tracing;

// const FAVICON: Asset = asset!("/assets/favicon.ico");
const BOOTSTRAP_CSS: Asset = asset!("/assets/css/bootstrap.min.css");
const MAIN_CSS: Asset = asset!("/assets/css/main.css");

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
                class: "footer mt-auto py-3",
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

    let mut symbol_mapping = BiMap::<String, String>::new();
    symbol_mapping.insert("r".into(), "→".into());

    let mut handle_symbol = {
        let symbol_mapping = symbol_mapping.clone(); // TODO: get rid of this clone.
        move |e: Event<FormData>| {
            tracing::debug!("{e:?}");
            let v = e.value();
            if let Some(abbr) = symbol_mapping.clone().get_by_right(&v) {
                let mut r = "\\".to_string();
                r.push_str(abbr);
                tracing::debug!("Some: {abbr:?}");
                abbreviation.set(r);
            } else {
                tracing::debug!("None");
                abbreviation.set("".to_string());
            }
            symbol.set(v);
            last_edit.set(true);
        }
    };
    let normalize_abbreviation = |mut s: String| {
        if s.chars().next() == Some('\\') {
            s.remove(0);
        }
        s
    };
    let mut handle_abbreviation = {
        let symbol_mapping = symbol_mapping.clone(); // TODO: get rid of this clone.
        move |e: Event<FormData>| {
            tracing::debug!("{e:?}");
            let v = normalize_abbreviation(e.value());

            if let Some(symb) = symbol_mapping.clone().get_by_left(&v) {
                tracing::debug!("Some: {symb:?}");
                symbol.set(symb.clone());
            } else {
                tracing::debug!("None");
                symbol.set("".to_string());
            }
            abbreviation.set(e.value());
            last_edit.set(false);
        }
    };



    let both_empty = use_memo(move || symbol() == "" && abbreviation() == "");

    let left_error = {
        let symbol_mapping = symbol_mapping.clone(); // TODO: get rid of this clone.
        use_memo(move || last_edit() && symbol() != "" && symbol_mapping.get_by_right(&symbol()).is_none())
    };
    let right_error = {
        let symbol_mapping = symbol_mapping.clone(); // TODO: get rid of this clone.
        use_memo(move || !last_edit() && abbreviation() != "" && symbol_mapping.get_by_left(&normalize_abbreviation(abbreviation())).is_none())
    };

    rsx! {
        div {
            class: "row",
            div {
                class: "col-md-4 offset-md-1 py-5 text-center",
                label {
                    for: "form-input-symbol",
                    class: "form-label h5",
                    "Symbol"
                }
                input {
                    type: "text",
                    id: "form-input-symbol",
                    class: format_args!("form-control {}", if left_error() {"is-invalid"} else {""}),
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
                class: "col-md-4 offset-md-2 py-5 text-center",
                label {
                    for: "form-abbreviation-symbol",
                    class: "form-label h5",
                    "Abbreviation"
                }
                input {
                    type: "text",
                    class: format_args!("form-control py-1 px-2 {}", if right_error() {"is-invalid"} else {""}),
                "aria-describedby": "form-abbreviation-symbol-feedback",
                    placeholder: if both_empty() {"\\r"} else {""},
                    oninput: move |evt| handle_abbreviation(evt),
                    // onkeyup: move |evt| handle_abbreviation(evt),
                    value: "{abbreviation}"
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

