#[derive(Copy, Clone)]
struct Card {
    rank: Rank,
    suit: Suit,
    seen: bool,
}

#[derive(Copy, Clone, PartialEq)]
enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

#[derive(Copy, Clone, PartialEq)]
enum Suit {
    Spade,
    Club,
    Heart,
    Diamond,
}

#[derive(Copy, Clone)]
enum HandType {
    ThreeKind,
    TwoPair,
    HighCard,
}

#[derive(Copy, Clone)]
enum Bet {
    Bet(u16),
    Fold,
}

struct Round {
    chips: u16,
    pot: u16,
    bet: u16,
    hole: Card,
    up_first: Card,
    up_second: Option<Card>,
    players: Vec<(Card, Option<Card>)>,
}

enum ServerResponse {
    Login,
    Bet(String),
    Status(String),
    Done(String),
    Misc(String),
}

pub mod deck;
