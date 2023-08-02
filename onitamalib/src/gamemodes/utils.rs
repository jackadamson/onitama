use crate::{CardSet, CardSetDescription};
use enum_iterator::IntoEnumIterator;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = listCardSets)]
pub fn list_card_sets() -> JsValue {
    let card_sets: Vec<CardSet> = CardSet::into_enum_iter().collect();
    let card_sets: Vec<CardSetDescription> = card_sets
        .into_iter()
        .map(|card_set| CardSetDescription::from(card_set))
        .collect();
    serde_wasm_bindgen::to_value(&card_sets).unwrap()
}
