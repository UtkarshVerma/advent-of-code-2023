use std::{cmp::Ordering, collections::HashMap};

use crate::answer::Answer;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn new(char: char) -> Self {
        match char {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,

            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: u32,
}

impl Hand {
    fn new(cards: Vec<Card>, bid: u32) -> Self {
        let clusters = cards.iter().fold(
            HashMap::new(),
            |mut cards_by_type: HashMap<Card, u8>, card| {
                *cards_by_type.entry(card.clone()).or_default() += 1;

                cards_by_type
            },
        );

        let max_cluster_size = clusters.values().max().copied().unwrap_or_default();

        let hand_type = match max_cluster_size {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if clusters.len() == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if clusters.len() == 3 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            1 => HandType::HighCard,

            _ => unreachable!(),
        };

        Hand {
            cards,
            bid,
            hand_type,
        }
    }

    fn compare_rank(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            return self
                .cards
                .iter()
                .zip(other.cards.iter())
                .find(|(self_card, other_card)| {
                    if self_card == other_card {
                        return false;
                    }

                    true
                })
                .map(|(self_card, other_card)| self_card.cmp(other_card))
                .unwrap_or(Ordering::Equal);
        }

        self.hand_type.cmp(&other.hand_type)
    }
}

pub fn solve(input: &str) -> Answer {
    let mut hands = input
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let cards = parts
                .next()
                .unwrap()
                .chars()
                .map(Card::new)
                .collect::<Vec<Card>>();
            let bid = parts.next().unwrap().parse::<u32>().unwrap();

            Hand::new(cards, bid)
        })
        .collect::<Vec<Hand>>();

    hands.sort_by(|a, b| a.compare_rank(b));

    let answer = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| ((i + 1) as u32) * hand.bid)
        .sum();

    Answer::Number(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(solve(input), Answer::Number(6440));
    }
}
