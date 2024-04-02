#[derive(Copy, Clone)]
use nom::{
    branch::alt, bytes::complete::{tag, take_till, take_until1}, combinator::value, IResult
};

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

#[derive(Clone)]
enum ServerResponse <'a>{
    Login,
    Bet(&'a str),
    Status(&'a str),
    Done(&'a str),
    Misc(&'a str),
}

fn till_colon(i: &str) -> IResult<&str, &str> {
    take_till(|c| c == ':')(i)
}

fn server_response(i: &str) -> IResult<ServerResponse, &str> {
    let (i, took) = till_colon(i)?;

    let (i, response) = alt(
        value(ServerResponse::Login, tag("login")),
        value(ServerResponse::Bet(i), tag("bet1")),
    )
    
    todo!()
}
