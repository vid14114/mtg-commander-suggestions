use scryfall::{
    card::{CardFace, Color},
    Card,
};

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
                combined_text
                    .to_owned()
                    .push_str(&(format!(" {}", face_text)));
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
            .map(|face| {
                face.colors
                    .as_ref()
                    .expect("Multiple faces colors")
                    .to_owned()
            })
            .flatten()
            .collect(),
    }
}

fn extract_multiple_faces(card: &Card) -> &Vec<CardFace> {
    card.card_faces.as_ref().expect("Card with multiple faces")
}
