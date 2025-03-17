use std::u16;

use chrono::{DateTime, Datelike, Duration, Local};
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

#[derive(Clone, Copy)]
struct ShowDate {
    date: Signal<DateTime<Local>>,
}

#[component]
pub fn Saints() -> Element {
    let date = use_signal(|| chrono::Local::now());
    use_context_provider(|| ShowDate { date });

    let saints = use_resource(move || async move {
        let current_date = date().format("%Y-%m-%d").to_string();
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

         Header{}
        div { class: "content",
             for (i, item) in saints.cloned().unwrap_or_default().iter().enumerate() {
                Saint {id: i as u16, saint: item.clone() }
             }
        }
    }
}

#[component]
pub fn Header() -> Element {
    fn format_date_in_italian(datetime: chrono::DateTime<Local>) -> String {
        let days = [
            "domenica",
            "lunedì",
            "martedì",
            "mercoledì",
            "giovedì",
            "venerdì",
            "sabato",
        ];
        let months = [
            "gennaio",
            "febbraio",
            "marzo",
            "aprile",
            "maggio",
            "giugno",
            "luglio",
            "agosto",
            "settembre",
            "ottobre",
            "novembre",
            "dicembre",
        ];

        let week_day = days[datetime.weekday().num_days_from_sunday() as usize];
        let day = datetime.day();
        let month = months[(datetime.month() - 1) as usize];

        format!("{} {} {}", week_day, day, month)
    }

    let _current = (use_context::<ShowDate>().date)();

    let next = move |_| {
        consume_context::<ShowDate>()
            .date
            .set(_current + Duration::days(1));
    };

    let prev = move |_| {
        consume_context::<ShowDate>()
            .date
            .set(_current - Duration::days(1));
    };

    rsx! {
          div { class: "header",
                button { onclick: prev, class: "button", id: "prev", "\u{21ab}" }
                h1 { id: "date",
                    { format_date_in_italian(_current) }
                }
                button { onclick: next, class: "button", id: "next", "\u{21ac}" }
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
                class: "card-content",
                h2 { "{saint.nome}" }
                h3 { "{saint.tipologia}" }
                p { "{descr}" }
            }

        }
    }
}
