use std::{collections::HashMap, hash::Hash, vec};

use web_sys::wasm_bindgen::JsValue;
use yew::AttrValue;

use crate::model::{Gebaeude, Raum, Schrank};

pub async fn fetch_get<T: serde::de::DeserializeOwned>(url: &str) -> Option<T> {
    let response = match gloo_net::http::Request::get(url).send().await {
        Ok(res) => res,
        Err(err) => {
            alert(&format!("{:?}", err));
            return None;
        }
    };

    let json = match response.json::<T>().await {
        Ok(json) => json,
        Err(err) => {
            alert(&format!("{:?}", err));
            return None;
        }
    };

    Some(json)
}
pub async fn fetch_post_with_body(url: &str, body: impl Into<JsValue>) {
    let request = match gloo_net::http::Request::post(url)
        .header("Content-Type", "application/json")
        .body(body)
    {
        Ok(req) => req,
        Err(err) => {
            alert(&format!("{:?}", err));
            return;
        }
    };

    if let Err(err) = request.send().await {
        alert(&format!("{:?}", err));
    }
}
pub async fn fetch_put_with_body(url: &str, body: impl Into<JsValue>) {
    let request = match gloo_net::http::Request::put(url)
        .header("Content-Type", "application/json")
        .body(body)
    {
        Ok(req) => req,
        Err(err) => {
            alert(&format!("{:?}", err));
            return;
        }
    };

    if let Err(err) = request.send().await {
        alert(&format!("{:?}", err));
    }
}
pub async fn fetch_delete_with_body(url: &str, body: impl Into<JsValue>) {
    let request = match gloo_net::http::Request::delete(url)
        .header("Content-Type", "application/json")
        .body(body)
    {
        Ok(req) => req,
        Err(err) => {
            alert(&format!("{:?}", err));
            return;
        }
    };

    if let Err(err) = request.send().await {
        alert(&format!("{:?}", err));
    }
}
pub fn alert(message: &str) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let _ = window.alert_with_message(message);
}

type FullMap = HashMap<String, HashMap<i32, (Vec<Schrank>, Vec<Raum>)>>;
type FullVec = Vec<(AttrValue, Vec<(i32, Vec<Schrank>, Vec<Raum>)>)>;

pub fn map_schraenke_raeume(
    gebaeude: Vec<Gebaeude>,
    schraenke: Vec<Schrank>,
    raeume: Vec<Raum>,
) -> FullVec {
    let mut gebaeude_map: FullMap = gebaeude
        .into_iter()
        .map(|g| (g.ge_name, HashMap::new()))
        .collect();

    for schrank in schraenke.into_iter() {
        let stockwerk_map = match gebaeude_map.get_mut(&schrank.sc_ge_name) {
            Some(smap) => smap,
            None => {
                let _ = gebaeude_map.insert(
                    schrank.sc_ge_name.clone(),
                    [(schrank.sc_stockwerk, (vec![schrank], Vec::new()))].into(),
                );
                continue;
            }
        };

        match stockwerk_map.get_mut(&schrank.sc_stockwerk) {
            Some((vec, _)) => vec.push(schrank),
            None => {
                let _ = stockwerk_map.insert(schrank.sc_stockwerk, (vec![schrank], Vec::new()));
            }
        };
    }

    for raum in raeume.into_iter() {
        let stockwerk_map = match gebaeude_map.get_mut(&raum.ra_ge_name) {
            Some(smap) => smap,
            None => {
                let _ = gebaeude_map.insert(
                    raum.ra_ge_name.clone(),
                    [(raum.ra_stockwerk, (Vec::new(), vec![raum]))].into(),
                );
                continue;
            }
        };

        match stockwerk_map.get_mut(&raum.ra_stockwerk) {
            Some((_, vec)) => vec.push(raum),
            None => {
                let _ = stockwerk_map.insert(raum.ra_stockwerk, (Vec::new(), vec![raum]));
            }
        };
    }

    let mut gebaeude_vec: FullVec = gebaeude_map
        .into_iter()
        .map(|(ge_name, stockwerk_map)| {
            let mut stockwerk_vec: Vec<_> = stockwerk_map
                .into_iter()
                .map(|(stockwerk, (mut schraenke, mut raeume))| {
                    schraenke.sort_by(|a, b| a.sc_nummer.cmp(&b.sc_nummer));
                    raeume.sort_by(|a, b| a.ra_nummer.cmp(&b.ra_nummer));

                    (stockwerk, schraenke, raeume)
                })
                .collect();

            stockwerk_vec.sort_by(|(s1, ..), (s2, ..)| s1.cmp(s2));

            (AttrValue::from(ge_name), stockwerk_vec)
        })
        .collect();
    gebaeude_vec.sort_by(|a, b| a.0.cmp(&b.0));
    gebaeude_vec
}

pub fn pretty_stockwerk_number(stockwerk: i32) -> String {
    match stockwerk {
        0 => String::from("EG"),
        s @ ..=-1 => format!("{}UG", s.abs()),
        s @ 1.. => format!("{}OG", s),
    }
}
