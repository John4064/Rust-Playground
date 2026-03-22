#![allow(unused_variables)]
#![allow(dead_code)]

use std::process::exit;
/**@Author John Parkhurst

 * @brief learning about Rust
 */

use rand::{thread_rng, Rng};
use rand::rngs::ThreadRng;
use rand_distr::{Distribution, Uniform};
use crate::CardColor::{Red,Black};
use crate::CardSuite::{Clubs,Spades,Hearts,Diamonds};

trait Blackjack{
    // fn generate_deck(self);

    /**
    Generate a brand new deck or reset the deck back to standard 52 card blackjack deck
    **/
    fn generate_deck(&mut self);
    fn print_deck(&self);

    fn shuffle_deck(&mut self, rng: ThreadRng);
}

#[derive(Debug)]
enum CardSuite {
    Diamonds,
    Clubs,
    Hearts,
    Spades
}
#[derive(Debug)]
enum CardColor{
    Black,
    Red
}
#[derive(Debug)]
struct Card{
    card_suite: CardSuite,
    card_color: CardColor,
    card_value: i32,//No rewrite, but handle game logic if >21 then convert all ace in hands to 1
    card_face: String
    
}


//Blackjack deck right now then expand in future
#[derive(Debug)]
struct Deck{
    //Define an array of 53 cards
    cards: Vec<Card>
}

impl Blackjack for Deck{
    fn generate_deck(&mut self){
        self.cards=Vec::new();
        //Standard 2->10 cards
        for m in 2..11 {
            //Second for loop for suite
            for n in 0..4{
                match n {
                    0 => {
                        self.cards.push(Card{card_suite:Clubs,card_color:Black,card_value:m,card_face:m.to_string()});
                    }
                    1 => {
                        self.cards.push(Card{card_suite:Diamonds,card_color:Red,card_value:m,card_face:m.to_string()});
                    }
                    2 => {
                        self.cards.push(Card{card_suite:Spades,card_color:Black,card_value:m,card_face:m.to_string()});
                    }
                    3 => {
                        self.cards.push(Card{card_suite:Hearts,card_color:Red,card_value:m,card_face:m.to_string()});
                    }
                    _ => {exit(49);}
                }


            }
        }
        //4 suites
        //2 colors

        for n in 0..3{
            match n {
                0 =>{//Jack
                    self.cards.push(Card{card_suite:Diamonds,card_color:Red,card_value:10,card_face: "Jack".to_string()});
                    self.cards.push(Card{card_suite:Clubs,card_color:Black,card_value:10,card_face: "Jack".to_string()});
                    self.cards.push(Card{card_suite:Spades,card_color:Red,card_value:10,card_face: "Jack".to_string()});
                    self.cards.push(Card{card_suite:Hearts,card_color:Black,card_value:10,card_face: "Jack".to_string()});
                }
                1=>{//Queen
                    self.cards.push(Card{card_suite:Diamonds,card_color:Red,card_value:10,card_face: "Queen".to_string()});
                    self.cards.push(Card{card_suite:Clubs,card_color:Black,card_value:10,card_face: "Queen".to_string()});
                    self.cards.push(Card{card_suite:Spades,card_color:Red,card_value:10,card_face: "Queen".to_string()});
                    self.cards.push(Card{card_suite:Hearts,card_color:Black,card_value:10,card_face: "Queen".to_string()});
                }
                2=>{//King
                    self.cards.push(Card{card_suite:Diamonds,card_color:Red,card_value:10,card_face: "King".to_string()});
                    self.cards.push(Card{card_suite:Clubs,card_color:Black,card_value:10,card_face: "King".to_string()});
                    self.cards.push(Card{card_suite:Spades,card_color:Red,card_value:10,card_face: "King".to_string()});
                    self.cards.push(Card{card_suite:Hearts,card_color:Black,card_value:10,card_face: "King".to_string()});
                }
                _=>{
                    exit(50);
                }
            }

        }

        //ACE
        self.cards.push(Card{card_suite:Diamonds,card_color:Red,card_value:11,card_face: "Ace".to_string()});
        self.cards.push(Card{card_suite:Clubs,card_color:Black,card_value:11,card_face: "Ace".to_string()});
        self.cards.push(Card{card_suite:Spades,card_color:Red,card_value:11,card_face: "Ace".to_string()});
        self.cards.push(Card{card_suite:Hearts,card_color:Black,card_value:11,card_face: "Ace".to_string()});

    }

    fn print_deck(&self){
        // println!("Current Deck: {:?}", self.cards);
        println!("Deck Size: {}",self.cards.len());
        for card in &self.cards{
            println!("Current Card: {:?}", card);
        }

    }

    fn shuffle_deck(&mut self, rng: ThreadRng) {
        let mut newVec = Vec::new();

        for n in 0..self.cards.len() {

        }

        self.cards=newVec;
    }



}


fn main(){
    let mut rng = thread_rng();
    // let random_number: u32 = rng.gen();
    // println!("{}",random_number);

    //define array of 52 cards and push all cards in, create shuffle function to reset and recreate array

    let mut blackjack_deck:Deck= Deck{cards:Vec::new()};
    blackjack_deck.generate_deck();
    blackjack_deck.shuffle_deck(rng);
    blackjack_deck.print_deck();



}