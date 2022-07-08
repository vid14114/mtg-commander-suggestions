use scryfall::{
    card::{CardFace, Color},
    Card,
};
use std::fmt::Write;

pub fn extract_oracle_text(card: &Card) -> String {
    match &card.oracle_text {
        Some(card_text) => card_text.to_owned(),
        None => extract_multiple_faces(card)
            .iter()
            .map(|face| {
                face.oracle_text
                    .as_ref()
                    .expect("Multiple faces oracle text")
            })
            .reduce(|combined_text, face_text| {
                write!(combined_text.to_owned(), " {}", face_text).expect("Appending multiple faces text");
                combined_text
            })
            .expect("Multiple faces reduce")
            .to_string(),
    }
}

pub fn extract_card_colors(card: &Card) -> Vec<Color> {
    match &card.colors {
        Some(colors) => colors.to_owned(),
        None => extract_multiple_faces(card)
            .iter()
            .flat_map(|face| {
                face.colors
                    .as_ref()
                    .expect("Multiple faces colors")
                    .to_owned()
            })
            .collect(),
    }
}

fn extract_multiple_faces(card: &Card) -> &Vec<CardFace> {
    card.card_faces.as_ref().expect("Card with multiple faces")
}
