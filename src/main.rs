mod deck;
fn main() {
 let mut deck = deck::Deck::new();
 deck.shuffle();
 deck.deal(6);
// The deck is created using the Deck struct's new method, which initializes the cards vector with all combinations of suits and values.
    println!("Here are my cards:
{:#?}
    ", deck); 
}
// The above code creates a deck of cards with three suits and three values.
// It uses a nested loop to create all combinations of suits and values,
// and stores them in a vector. The deck is then printed using the Debug trait.
// The process of creating the deck is done in a functional style, and will be extracted into a function in the next step. using inherent implementation
//This will be done in a new file called deck.rs 