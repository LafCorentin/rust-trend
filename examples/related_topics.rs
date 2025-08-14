use rtrend::{Client, Country, Keywords, RelatedTopics};

fn main() {
    let keywords = Keywords::new(vec!["Pasta"]).unwrap();
    let country = Country::IT;

    let client = Client::new(keywords, country).build();

    let search_interest = RelatedTopics::new(client).get().unwrap();
    println!("{}", search_interest);
}
