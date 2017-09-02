pub mod deck {

    extern crate rand;
    use self::rand::Rng;
    use self::card::*;

    #[derive(Debug)]
    pub struct Deck {
        cards: Vec<Card>,
    }

    impl Deck {
        pub fn new() -> Deck {
            let mut new_deck = Deck { cards: Vec::new() };

            for i in 0..52 {
                let card_value = (i % 13) + 1;
                let suit_num = i / 13;

                let card_suit = match suit_num {
                    0 => CardSuit::Spades,
                    1 => CardSuit::Hearts,
                    2 => CardSuit::Diamonds,
                    3 => CardSuit::Clubs,
                    _ => panic!("Invalid card value"),
                };
                new_deck.cards.push(Card::new(card_suit, card_value));
            }

            new_deck
        }

        pub fn shuffle(&mut self) {
            rand::thread_rng().shuffle(&mut self.cards);
        }

        pub fn split_into(&mut self, number: usize) -> Vec<Deck> {
            let splits: Vec<Deck> = self.cards
                .chunks(number)
                .map(|chunk| Deck { cards: chunk.to_vec() })
                .collect();
            splits
        }

        pub fn lowest_card(&self, suit: CardSuit) -> Option<Card> {
            self.cards
                .iter()
                .filter(|card| card.suit == suit)
                .min_by(|a, b| a.value.cmp(&b.value))
                .map(|card| card.clone())
        }
    }

    pub mod card {

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum CardSuit {
            Spades,
            Hearts,
            Diamonds,
            Clubs,
            Dummy,
        }

        #[derive(Debug, Copy)]
        pub struct Card {
            pub suit: CardSuit,
            pub value: u32,
        }

        impl Card {
            pub fn new(suit: CardSuit, value: u32) -> Card {
                Card {
                    suit: suit,
                    value: value,
                }
            }
        }

        impl Clone for Card {
            fn clone(&self) -> Card {
                Card {
                    suit: self.suit,
                    value: self.value,
                }
            }
        }


    }
}
