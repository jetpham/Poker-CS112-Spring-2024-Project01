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

fn evaluate_hand(cards: [Card; 3]) -> u8 {
    // Array to count occurrences of each rank in the hand
    let mut ranks_count = [0; 13];
    
    // Count occurrences of each rank in the hand
    for card in cards.iter() {
        let rank_index = card.rank as usize;
        ranks_count[rank_index] += 1;
    }

    // Check for a pair
    for (i, &count) in ranks_count.iter().enumerate() {
        if count == 2 {
            // Pair found, calculate hand value
            let pair_value = i as u8;
            let mut hand_value = (pair_value * 13) + 13;
            
            // Find the value of the remaining high card
            for (j, &count) in ranks_count.iter().enumerate() {
                if count == 1 {
                    hand_value += j as u8;
                    break;
                }
            }
            
            return hand_value;
        }
    }

    // Check for three of a kind
    for (i, &count) in ranks_count.iter().enumerate() {
        if count == 3 {
            // Three of a kind found, calculate hand value
            return 13 + (13 * 13) + i as u8;
        }
    }

    // No pair or three of a kind found, evaluate as high card
    let mut hand_value = 0;
    for (i, &count) in ranks_count.iter().enumerate().rev() {
        if count == 1 {
            hand_value = i as u8;
            break;
        }
    }
    hand_value
}
// Define the Hand struct
struct Hand {
    cards: [Card; 3],
}

impl Hand {
    // Constructor for creating a new Hand instance
    fn new(cards: [Card; 3]) -> Self {
        Hand { cards }
    }
}

// Implement PartialOrd and Ord traits for Hand
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Compare the hand values using evaluate_hand function
        evaluate_hand(self.cards).cmp(&evaluate_hand(other.cards))
    }
}
}
