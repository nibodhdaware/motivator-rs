use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
struct Quotes {
    pub id: Option<String>,
    pub author: String,
    pub content: String,
    pub tags: Vec<String>,
    pub author_slug: String,
    pub length: i32,
    pub date_added: String,
    pub date_modified: String,
}

#[function_component(App)]
fn app() -> Html {
    let quotes = use_state(|| None);
    {
        let quotes = quotes.clone();
        use_effect_with((), move |_| {
            let quotes = quotes.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let quotes_endpoint = "https://api.quotable.io/quotes/random";
                let fetched_quotes: Vec<Quotes> = Request::get(&quotes_endpoint)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                quotes.set(Some(fetched_quotes));
            });
            || ()
        });
    }

    match quotes.as_ref() {
        Some(q) => {
            html! {
            <div class="quote-container">
                <div id="quote">
                    <h2><i>{serde_json::to_string(&q[0].content).unwrap()}</i></h2>
                </div>
                <div id="author">
                    <h3>{serde_json::to_string(&q[0].author).unwrap()}</h3>
                </div>
            </div>
            }
        }
        None => html! {"No data yet"},
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
