mod card_utils;
pub mod commander;
pub mod storage;

use std::collections::HashMap;

use scryfall::Card;

use card_utils::{extract_card_colors, extract_oracle_text};

pub fn filter_commanders(cards: &[Card]) -> Vec<Card> {
    cards
        .iter()
        .filter(|card| card.type_line.contains("Legendary"))
        .filter(|card| card.type_line.contains("Creature"))
        .map(|commander| commander.to_owned())
        .collect()
}

pub fn find_compatible_cards(
    commander_keywords: Vec<(Card, Vec<String>)>,
    collection: &[Card],
) -> Vec<(Card, HashMap<String, Vec<Card>>)> {
    commander_keywords
        .iter()
        .map(|(commander, keywords)| {
            (
                commander.to_owned(),
                match_colors_and_keywords(commander, keywords, collection),
            )
        })
        .collect()
}

fn match_colors_and_keywords(
    commander: &Card,
    keywords: &[String],
    collection: &[Card],
) -> HashMap<String, Vec<Card>> {
    let mut keywords_cards = HashMap::new();
    collection
        .iter()
        .filter(|card| {
            extract_card_colors(card)
                .iter()
                .all(|color| commander.color_identity.contains(color))
        })
        .filter_map(|card| {
            let keyword = keywords.iter().find(|&keyword| {
                card.type_line.contains(keyword) || extract_oracle_text(card).contains(keyword)
            });
            keyword.map(|keyword| (keyword.to_owned(), card.to_owned()))
        })
        .for_each(|(keyword, card)| {
            keywords_cards
                .entry(keyword)
                .or_insert_with(Vec::new)
                .push(card)
        });
    keywords_cards
}
