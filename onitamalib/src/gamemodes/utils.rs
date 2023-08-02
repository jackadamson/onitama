use crate::{CardSet, CardSetDescription};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = listCardSets)]
pub fn list_card_sets() -> JsValue {
    let card_sets = vec![CardSet::Base, CardSet::SenseiPath];
    let card_sets: Vec<CardSetDescription> = card_sets
        .into_iter()
        .map(|card_set| CardSetDescription::from(card_set))
        .collect();
    serde_wasm_bindgen::to_value(&card_sets).unwrap()
}
