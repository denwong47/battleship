//! Possible types of ships.
//!
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use serde::{Deserialize, Serialize};

macro_rules! expand_types {
    (
        $((
            $name:ident,
            $label:literal,
            $length:literal,
            $ansi_colour:literal
        )),*$(,)?
    ) => {
        /// All possible ship types.
        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub enum ShipType {
            $($name,)*
        }

        impl ShipType {
            /// Get the label of this ship type.
            pub fn label(&self) -> &'static str {
                match self {
                    $(
                        Self::$name => $label,
                    )*
                }
            }

            /// Get the length of this ship type.
            pub fn length(&self) -> usize {
                match self {
                    $(
                        Self::$name => $length,
                    )*
                }
            }

            /// Wraps the &[`str`] with the designated colour.
            pub fn colour_wraps(&self, text: &str) -> String {
                let colour = match self {
                    $(
                        Self::$name => $ansi_colour,
                    )*
                };

                format!(
                    "\u{1b}[38:5:{colour}m{text}\u{1b}[39m"
                )
            }
        }
    };
}

expand_types!(
    (AircraftCarrier, "Aircraft Carrier", 5, 28),
    (Battleship, "Battleship", 4, 11),
    (Cruiser, "Cruiser", 3, 171),
    (Frigate, "Frigate", 2, 21),
    (Submarine, "Submarine", 3, 214),
);

impl Distribution<ShipType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ShipType {
        match rng.gen_range(0..=4) {
            0 => ShipType::AircraftCarrier,
            1 => ShipType::Battleship,
            2 => ShipType::Cruiser,
            4 => ShipType::Submarine,
            _ => ShipType::Frigate,
        }
    }
}
