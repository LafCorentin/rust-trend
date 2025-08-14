use rtrend::{Client, Country, Keywords, RelatedQueries};

fn main() {
    let keywords = Keywords::new(vec!["Cinema"]).unwrap();
    let country = Country::ALL;

    let client = Client::new(keywords, country).build();

    let search_interest = RelatedQueries::new(client).get().unwrap();
    println!("{}", search_interest);
}
