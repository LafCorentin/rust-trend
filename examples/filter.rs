use rtrend::{Category, Client, Country, Keywords, Lang, Property, RelatedQueries};

fn main() {
    let keywords = Keywords::new(vec!["Pasta"]).unwrap();
    let country = Country::IT;

    // Set response lang to french and search on Google Image
    let lang = Lang::FR;
    let property = Property::Web;

    let client = Client::new(keywords, country)
        .with_lang(lang)
        .with_property(property)
        .with_category(Category::FoodAndDrink)
        .build();
    let related_queries = RelatedQueries::new(client).get().unwrap();
    println!("{}", related_queries);
}
