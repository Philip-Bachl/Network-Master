use std::collections::HashMap;

use crate::model::Schrank;

pub async fn fetch<T: serde::de::DeserializeOwned>(url: &str) -> Option<T> {
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
pub fn alert(message: &str) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let _ = window.alert_with_message(message);
}

// Gebäude => Stockwerk => Schrank
pub fn map_schraenke_to_ge_name(
    schraenke: Vec<Schrank>,
) -> HashMap<String, HashMap<i32, Vec<Schrank>>> {
    let mut gebaeude_map: HashMap<String, HashMap<i32, Vec<Schrank>>> = HashMap::new();

    for schrank in schraenke.into_iter() {
        let stockwerk_map = match gebaeude_map.get_mut(&schrank.sc_ge_name) {
            Some(smap) => smap,
            None => {
                let _ = gebaeude_map.insert(
                    schrank.sc_ge_name.clone(),
                    [(schrank.sc_stockwerk, vec![schrank])].into(),
                );
                continue;
            }
        };

        match stockwerk_map.get_mut(&schrank.sc_stockwerk) {
            Some(vec) => vec.push(schrank),
            None => {
                let _ = stockwerk_map.insert(schrank.sc_stockwerk, vec![schrank]);
            }
        };
    }

    gebaeude_map
}

pub fn pretty_stockwerk_number(stockwerk: i32) -> String {
    match stockwerk {
        0 => String::from("EG"),
        s @ ..=-1 => format!("{} UG", s.abs()),
        s @ 1.. => format!("{} OG", s),
    }
}
/*pub fn map_schraenke_to_ge_name(schraenke: &[Schrank]) -> HashMap<String, Vec<Schrank>> {
    let mut map: HashMap<String, Vec<Schrank>> = HashMap::new();
    for schrank in schraenke {
        let schrank = schrank.clone();

        if let Some(vec) = map.get_mut(&schrank.sc_ge_name) {
            vec.push(schrank);
        } else {
            map.insert(schrank.sc_ge_name.clone(), vec![schrank]);
        }
    }

    map
}*/
