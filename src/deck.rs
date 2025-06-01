use rand::{rng, prelude::SliceRandom};
#[derive(Debug)]
pub struct Deck {
  cards:Vec<String>
} 
impl Deck {
  pub fn new() -> Self {
    let mut cards = Vec::new();
    let suits = ["spades", "clubs", "hearts"];
    let values = ["one", "ace", "three"];
    for i in suits{
        for j in values{
          let card =  format!("{} of {}", j,i);
          cards.push(card) 
        }
    }
    Deck{cards}
  }
  
  pub fn shuffle(&mut self) { 
    self.cards.shuffle(&mut rng()) 
  }
  pub fn deal(&mut self, at:usize)->Vec<String>{
  self.cards.split_off(self.cards.len() - at)
  }
}
//impl is a keyword in Rust that allows you to define methods and functions for a struct.

//The new() method serves as a Deck constructor that returns a new instance of the Deck struct.
//The self parameter refers to the instance of the struct that is being created. In Rust when we have a function that returns something, we need to specify the type of the return value. (-> Self)
//The new() method initializes the cards vector and populates it with the card combinations.