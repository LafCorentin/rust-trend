use rtrend::{Client, Country, Keywords, SearchInterest};

fn main() {
    let keywords = Keywords::new(vec!["Cinema"]).unwrap();
    let country = Country::ALL;

    let client = Client::new(keywords, country).build();

    let search_interest = SearchInterest::new(client).get().unwrap();
    println!("{}", search_interest);
}
