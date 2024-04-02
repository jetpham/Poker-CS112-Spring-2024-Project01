use crate::*;

/// Represents a deck of cards.
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Initializes a new deck or decks of cards.
    ///
    /// # Arguments
    ///
    /// * `num_decks` - The number of decks to initialize in the big deck
    ///
    /// # Returns
    ///
    /// A new `Deck` instance.
    fn new(num_decks: usize) -> Deck {
        let mut cards = Vec::new();
        for _ in 0..num_decks {
            for suit in &[Suit::Spade, Suit::Club, Suit::Heart, Suit::Diamond] {
                for rank in &[
                    Rank::Ace,
                    Rank::King,
                    Rank::Queen,
                    Rank::Jack,
                    Rank::Ten,
                    Rank::Nine,
                    Rank::Eight,
                    Rank::Seven,
                    Rank::Six,
                    Rank::Five,
                    Rank::Four,
                    Rank::Three,
                    Rank::Two,
                ] {
                    cards.push(Card {
                        rank: *rank,
                        suit: *suit,
                        seen: false,
                    });
                }
            }
        }
        Deck { cards }
    }

    /// Calculates the probability of drawing a specific card from the deck.
    ///
    /// # Arguments
    ///
    /// * `rank` - The rank of the card.
    /// * `suit` - An optional suit of the card. If specified, only cards with the specified suit will be considered.
    ///
    /// # Returns
    ///
    /// The probability of drawing the specified card from the deck as a floating-point number between 0.0 and 1.0.
    fn probability_of_drawing(&self, rank: Rank, suit: Option<Suit>) -> f64 {
        let mut matching_cards = 0;

        for card in &self.cards {
            if card.rank == rank && !card.seen {
                if let Some(suit_specified) = suit {
                    if card.suit == suit_specified {
                        matching_cards += 1;
                    }
                } else {
                    matching_cards += 1;
                }
            }
        }

        let total_unseen_cards: usize = self.cards.iter().filter(|c| !c.seen).count();

        if total_unseen_cards == 0 {
            return 0.0; // To avoid division by zero.
        }

        matching_cards as f64 / total_unseen_cards as f64
    }

    /// Marks a specific card as seen in the deck.
    ///
    /// # Arguments
    ///
    /// * `target_card` - The card to mark as seen.
    ///
    /// # Returns
    ///
    /// An `Ok` result if the card was found and marked as seen, or an `Err` result with an error message if the card was not found.
    fn mark_card_as_seen(&mut self, target_card: &Card) -> Result<(), &'static str> {
        let mut card_found_and_marked = false;

        for card in &mut self.cards {
            if card.rank == target_card.rank && card.suit == target_card.suit && !card.seen {
                card.seen = true;
                card_found_and_marked = true;
                // Don't break here, as we want to mark all unseen instances of the card.
            }
        }

        if card_found_and_marked {
            Ok(())
        } else {
            Err("No unseen instance of the specified card found in the deck.")
        }
    }

    /// Resets the seen status of all cards in the deck.
    fn shuffle(&mut self) {
        for card in &mut self.cards {
            card.seen = false;
        }
    }
}
