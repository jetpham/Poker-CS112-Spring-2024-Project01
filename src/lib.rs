struct Card {
    rank: Rank,
    suit: Suit,
    seen: bool,
}

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

enum Suit {
    Spade,
    Club,
    Heart,
    Diamond,
}

enum HandType {
    ThreeKind,
    TwoPair,
    HighCard,
}

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
