use nom::{
    branch::alt,
    bytes::{
        complete::{is_not, tag, take_till, take_until},
        streaming::take,
    },
    character::complete::{alpha1, not_line_ending},
    combinator::value,
    IResult,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResponseVariant {
    Login,
    Bet,
    Status,
    Done,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ServerResponse<'a> {
    pub variant: ResponseVariant,
    pub contents: &'a str,
}

fn till_colon(i: &str) -> IResult<&str, &str> {
    take_till(|c| c == ':')(i)
}

pub fn parse_response(i: &str) -> IResult<&str, ServerResponse> {
    let (contents, took) = till_colon(i)?;
    let (i, variant) = alt((
        value(ResponseVariant::Login, tag("login")),
        value(ResponseVariant::Bet, tag("bet1")),
        value(ResponseVariant::Bet, tag("bet2")),
        value(ResponseVariant::Status, tag("status")),
        value(ResponseVariant::Done, tag("done")),
    ))(took)?;

    Ok((i, ServerResponse { variant, contents }))
}

#[test]
fn test_recv() {
    assert_eq!(
        parse_response("login"),
        Ok((
            "",
            ServerResponse {
                variant: ResponseVariant::Login,
                contents: ""
            }
        ))
    );

    assert_eq!(
        parse_response("bet1:208:24:12:KS:10D:up:AD:8H:10D:QD:2C"),
        Ok((
            "",
            ServerResponse {
                variant: ResponseVariant::Bet,
                contents: ":208:24:12:KS:10D:up:AD:8H:10D:QD:2C"
            }
        ))
    );

    assert_eq!(
        parse_response("bet1:188:68:5:KS:10D:up:AS:8H:10D:QD:2C"),
        Ok((
            "",
            ServerResponse {
                variant: ResponseVariant::Bet,
                contents: ":188:68:5:KS:10D:up:AS:8H:10D:QD:2C"
            }
        ))
    );

    assert_eq!(
        parse_response("bet2:183:66:0:KS:10D:10S:up:AS:AQ:8H:6D:10D:4S:QD:JC:2C:4H"),
        Ok((
            "",
            ServerResponse {
                variant: ResponseVariant::Bet,
                contents: ":183:66:0:KS:10D:10S:up:AS:AQ:8H:6D:10D:4S:QD:JC:2C:4H"
            }
        ))
    );

    assert_eq!(
        parse_response("status:win:AS:AC:AD"),
        Ok((
            "",
            ServerResponse {
                variant: ResponseVariant::Status,
                contents: ":win:AS:AC:AD"
            }
        ))
    );

    assert_eq!(
        parse_response("done:explaination"),
        Ok((
            "",
            ServerResponse {
                variant: ResponseVariant::Done,
                contents: ":explaination"
            }
        ))
    );
}
