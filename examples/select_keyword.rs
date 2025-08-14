use rtrend::{Client, Country, Keywords, RegionInterest};

fn main() {
    let country = Country::US;
    let keywords = Keywords::new(vec!["Instagram", "Facebook", "Pinterest"]).unwrap();

    let client = Client::new(keywords, country).build();

    let region_interest_pinterest = RegionInterest::new(client).get_for("Pinterest").unwrap();
    println!("{}", region_interest_pinterest);
}
