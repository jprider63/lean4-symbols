use bimap::BiMap;
use dioxus::prelude::*;
use tracing;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const FONT_AWESOME_CSS: Asset = asset!("/assets/font-awesome.min.css");
const FONT_AWESOME_WOFF2: Asset = asset!("/assets/fonts/fontawesome-webfont.woff2");
const FONT_AWESOME_TTF: Asset = asset!("/assets/fonts/fontawesome-webfont.ttf");

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
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: FONT_AWESOME_CSS }
        h1 {
            id: "title",
            "How do I write this symbol in Lean?"
        }
        Symbols {}
        a {
            href: "https://github.com/jprider63/lean4-symbols",
            span {
                class: "fa fa-fw fa-2x fa-github-square"
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
    }
    let mut handle_abbreviation = {
        let symbol_mapping = symbol_mapping.clone(); // TODO: get rid of this clone.
        move |e: Event<FormData>| {
            tracing::debug!("{e:?}");
            let mut v = normalize_abbreviation(e.value());

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
            id: "symbols",
            div {
                "Symbol"
                input {
                    class: format_args!("appearance-none border rounded py-1 px-2 {}", if left_error() {"border-red-500"} else {""}),
                    placeholder: if both_empty() {"→"} else {""},
                    oninput: move |evt| handle_symbol(evt),
                    // onkeyup: move |evt| handle_symbol(evt),
                    value: "{symbol}"
                }
                if left_error() {
                    span {
                        "Unknown symbol"
                    }
                }
            }
            div {
                "Abbreviation"
                input {
                    class: format_args!("appearance-none border rounded py-1 px-2 {}", if left_error() {"border-red-500"} else {""}),
                    placeholder: if both_empty() {"\\r"} else {""},
                    oninput: move |evt| handle_abbreviation(evt),
                    // onkeyup: move |evt| handle_abbreviation(evt),
                    value: "{abbreviation}"
                }
                if right_error() {
                    span {
                        "Unknown symbol"
                    }
                }
            }
        }
    }
}


#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}
