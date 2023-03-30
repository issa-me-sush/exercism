use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    HighCard(u8, u8, u8, u8, u8),
    OnePair(u8, u8, u8, u8),
    TwoPair(u8, u8, u8),
    ThreeOfAKind(u8, u8, u8),
    Straight(u8),
    Flush(u8, u8, u8, u8, u8),
    FullHouse(u8, u8, u8),
    FourOfAKind(u8, u8),
    StraightFlush(u8),
}