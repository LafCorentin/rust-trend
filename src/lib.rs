//! # rust-trend
//!
//! **This lib is a work in progress**
//!
//! ## Overview
//! Unofficial Rust API for interacting with Google Trend
//!
//! ## Documentation
//! - [Examples Repository]("./examples")
//! - [API Documentation](https://docs.rs/rtrend)
//!
//! ## Example
//!
//! First, add the dependency to your `cargo.toml`:
//! ```toml
//! [dependencies]
//! rtrend = "0.1.3"
//! ```
//!
//! Then build a client and send the reqwest you want :
//! ```rust
//! use rtrend::{Keywords, Country, Client, RegionInterest};
//!
//! let country = Country::US;
//! let keywords = Keywords::new(vec!["Instagram","Facebook"]).unwrap();
//! let client = Client::new(keywords, country).build();
//!
//! // Then select the data you want. The interest of your keywords filtered by region for example:
//! let region_interest = RegionInterest::new(client).get().unwrap();
//! println!("{}", region_interest);
//!
//! // Result :
//! //{
//! //  "default": {
//! //    "geoMapData": [
//! //      {
//! //        "formattedValue": [
//! //          "100"
//! //        ],
//! //        "geoCode": "US-CA",
//! //        "geoName": "California",
//! //        "hasData": [
//! //          true
//! //        ],
//! //        "maxValueIndex": 0,
//! //        "value": [
//! //          100
//! //        ]
//! //      },
//! //
//! //      ...
//! //      
//! //      {
//! //        "formattedValue": [
//! //          "46"
//! //        ],
//! //        "geoCode": "US-SD",
//! //        "geoName": "South Dakota",
//! //        "hasData": [
//! //          true
//! //        ],
//! //        "maxValueIndex": 0,
//! //        "value": [
//! //          46
//! //        ]
//! //      }
//! //    ]
//! //  }
//! //}
//!
//! ```
//!
//! ### More example
//! - [Simple](./examples/simple.rs)
//! - [Region Interest](./examples/region_interest.rs)
//! - [Search Interest](./examples/search_interest.rs)
//! - [Related Queries](./examples/related_queries.rs)
//! - [Related Topics](./examples/related_topics.rs)
//! - [Use filters](./examples/filter.rs)
//! - [Get response for specific keyword](./examples/select_keyword.rs)
//!
//! ### Roadmap
//!
//! - [x] Write documentation & Doc Test
//! - [x] Release on crates.io
//! - [x] Add examples
//! - [ ] Add "TOP" and "RISING" filter
//! - [ ] Write more tests
//! - [ ] Make async feature (currently using `Reqwest::blocking`)
//!
//!
//! ### License
//!
//! Licensed under either of
//!
//!  * Apache License, Version 2.0
//!    ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
//!  * MIT license
//!    ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
//!
//! at your option.
//!
//! ### Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
//! dual licensed as above, without any additional terms or conditions.

pub mod client;

pub mod region_interest;
pub mod related_queries;
pub mod related_topics;
pub mod search_interest;

pub mod keywords;

mod cookie;
mod enums;
mod error;
mod request_handler;
mod utils;

pub use client::Client;
pub use cookie::Cookie;
pub use enums::category::Category;
pub use enums::country::Country;
pub use enums::lang::Lang;
pub use enums::period::Period;
pub use enums::property::Property;
pub use keywords::Keywords;
pub use region_interest::RegionInterest;
pub use related_queries::RelatedQueries;
pub use related_topics::RelatedTopics;
pub use search_interest::SearchInterest;
