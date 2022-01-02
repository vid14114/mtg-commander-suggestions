//! Enum defining the exiting platforms on with a magic card can exist.
use serde::{Deserialize, Serialize};

/// Enum defining the exiting platforms on with a magic card can exist.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
#[non_exhaustive]
pub enum Game {
    Paper,
    Arena,
    Mtgo,
    Astral,
    Sega,
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Game::Paper => "paper",
                Game::Arena => "arena",
                Game::Mtgo => "mtgo",
                Game::Astral => "astral",
                Game::Sega => "sega",
            }
        )
    }
}
