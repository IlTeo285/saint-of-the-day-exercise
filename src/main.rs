use std::u16;

use dioxus::prelude::*;
use serde::Deserialize;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Saints {}

    }
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct SaintApi {
    nome: String,
    tipologia: String,
    urlimmagine: Option<String>,
    descrizione: Option<String>,
}

#[component]
pub fn Saints() -> Element {
    let saints = use_resource(|| async move {
        let current_date = chrono::Local::now().format("%Y-%m-%d").to_string();
        reqwest::get(format!(
            "https://www.santodelgiorno.it/santi.json?data={}",
            current_date
        ))
        .await
        .unwrap()
        .json::<Vec<SaintApi>>()
        .await
        .unwrap()
    });

    rsx! {
         for (i, item) in saints.cloned().unwrap_or_default().iter().enumerate() {
            Saint {id: i as u16, saint: item.clone() }
         }
    }
}

#[component]
pub fn Saint(id: u16, saint: SaintApi) -> Element {
    let descr = saint.descrizione.unwrap_or_default();

    rsx! {
        div {
            id: "saint-{id}",
            class: "card",

            // Immagine della card
            if saint.urlimmagine.is_some() {
                img {
                    src: saint.urlimmagine.unwrap(),
                    alt: "Immagine della card",
                }
            }

            div {
                class: "content",
                h2 { "{saint.nome}" }
                h3 { "{saint.tipologia}" }
                p { "{descr}" }
            }

        }
    }
}
