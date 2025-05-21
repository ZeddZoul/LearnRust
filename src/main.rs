
#[derive(Debug)]
struct Deck{
    cards:Vec<String>
}
fn main() {
    let mut cards = Vec::new();
    let suits = ["spades", "clubs", "hearts"];
    let values = ["one", "ace", "three"];
    for i in suits{
        for j in values{
          let card =  format!("{} of {}", j,i);
          cards.push(card) 
        }
    }
    let deck = Deck{cards};
    println!("Here are my cards:
{:#?}
    ", deck); 
}
// stopped at folder 2 vid 4
