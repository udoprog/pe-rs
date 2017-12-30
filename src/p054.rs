/// Keywords: none

use std::io::{Read, BufRead, BufReader};
use std::collections::HashMap;

use self::Suit::*;
use self::Value::*;
use self::Hand::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Hand {
    HighCard(Value),
    OnePair(Value),
    TwoPairs(Value, Value),
    ThreeOfAKind(Value),
    Straight,
    Flush,
    FullHouse{
        three_of_a_kind: Value,
        pair: Value
    },
    FourOfAKind(Value),
    StraightFlush(Suit),
    RoyalFlush,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Card(Value, Suit);

pub struct Deal(Vec<Card>, Vec<Card>);

fn run(deals: Vec<Deal>) -> u64 {
    let mut wins = 0;

    for Deal(player1, player2) in deals {
        let hand1 = identify(player1.clone());
        let hand2 = identify(player2.clone());

        if hand1 > hand2 {
            wins += 1;
            continue;
        }

        // identical hands, compare by values
        if hand1 == hand2 {
            let mut values1: Vec<Value> = player1.iter().map(|c| c.0).collect();
            let mut values2: Vec<Value> = player2.iter().map(|c| c.0).collect();

            values1.sort();
            values1.reverse();

            values2.sort();
            values2.reverse();

            if values1 > values2 {
                println!("player1 wins");
                wins += 1;
            }

            continue;
        }
    }

    return wins;

    fn identify(mut hand: Vec<Card>) -> Hand {
        hand.sort();

        let lowest = hand[0].0 as usize;
        let straight = hand.iter().map(|v| v.0 as usize - lowest).collect::<Vec<_>>();
        let is_straight = &straight == &[0, 1, 2, 3, 4];

        let values = {
            let mut values: HashMap<Value, u32> = HashMap::new();

            for c in hand.iter().cloned() {
                *values.entry(c.0).or_insert_with(Default::default) += 1;
            }

            values
        };

        let pairs = {
            let mut pairs: Vec<Value> = values.iter().filter(|e| *e.1 == 2).map(|e| *e.0).collect();
            pairs.sort();
            pairs
        };

        if same_suit(&hand) {
            if hand.iter().map(|c| c.0).collect::<Vec<_>>() == vec![Ten, Jack, King, Queen, Ace] {
                return RoyalFlush;
            }

            if is_straight {
                return StraightFlush(hand[0].1);
            }

            return Flush;
        }

        if is_straight {
            return Straight;
        }

        if pairs.len() == 2 {
            return TwoPairs(pairs[0], pairs[1]);
        }

        if let Some(value) = values.iter().find(|e| *e.1 == 3) {
            if pairs.len() == 1 {
                return FullHouse {
                    three_of_a_kind: *value.0,
                    pair: pairs[0]
                };
            }

            return ThreeOfAKind(*value.0);
        }

        if pairs.len() == 1 {
            return OnePair(pairs[0]);
        }

        if let Some(value) = values.iter().find(|e| *e.1 == 4) {
            return FourOfAKind(*value.0);
        }

        HighCard(hand[hand.len() - 1].0)
    }

    fn same_suit(hand: &[Card]) -> bool {
        let suit = hand[0].1;
        hand.iter().all(|c| c.1 == suit)
    }
}

fn parse<R: Read>(input: R) -> Vec<Deal> {
    let mut deals = Vec::new();

    for line in BufReader::new(input).lines() {
        let line = line.expect("bad line");

        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        let hand1 = parse_hand(&parts[0..5]);
        let hand2 = parse_hand(&parts[5..10]);

        deals.push(Deal(hand1, hand2));
    }

    return deals;

    fn parse_hand(hand: &[&str]) -> Vec<Card> {
        let mut out = Vec::new();

        for card in hand.iter() {
            let mut c = card.chars();

            let value = match c.next().expect("value") {
                '2' => Two,
                '3' => Three,
                '4' => Four,
                '5' => Five,
                '6' => Six,
                '7' => Seven,
                '8' => Eight,
                '9' => Nine,
                'T' => Ten,
                'J' => Jack,
                'Q' => Queen,
                'K' => King,
                'A' => Ace,
                c => panic!("bad value: {}", c),
            };

            let suit = match c.next().expect("suit") {
                'C' => Clubs,
                'S' => Spades,
                'H' => Hearts,
                'D' => Diamonds,
                c => panic!("bad suite: {}", c),
            };

            out.push(Card(value, suit));
        }

        out
    }
}

problem!{
    tests => [
        q => {run(parse(::std::io::Cursor::new(include_str!("p054_poker.txt")))), "12e2c8df501501b2bb531e941a737ffa7a2a491e849c5c5841e3b6132291bc35"},
    ];
}
