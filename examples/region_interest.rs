use rtrend::{Client, Country, Keywords, RegionInterest};

fn main() {
    let country = Country::US;
    let keywords = Keywords::new(vec!["Instagram", "Facebook"]).unwrap();

    let client = Client::new(keywords, country).build();

    let region_interest = RegionInterest::new(client).get().unwrap();
    println!("{}", region_interest);
}
