//! A list of keywords to query on Google Trend
//! Keywords is limited to a maximum of 5 keywords.

use crate::error::{Error, Result};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Keywords {
    pub keywords: Vec<&'static str>,
}

impl Keywords {
    /// Create a new set of keywords.
    ///
    /// Keywords vector is limited to a maximum of 5 keyword.
    ///
    /// Returns a Keywords instance.
    ///
    /// # Example
    ///```rust
    /// use rtrend::Keywords;
    /// let keywords = Keywords::new(vec!["Unicorn","Labradoodle","Pikachu"]);
    /// ```
    ///
    /// # Panics
    /// A vector of length greater than 5 will panic.
    /// ```should_panic
    /// # use rtrend::Keywords;
    /// let seven_dwarf = vec!["Bashful","Doc", "Dopey","Grumpy","Happy", "Sleepy", "Sneezy"];
    /// let keywords = Keywords::new(seven_dwarf).unwrap();
    /// ```
    ///
    /// A vector without keywords will also panic.
    /// ```should_panic
    /// # use rtrend::Keywords;
    /// let keywords = Keywords::new(vec![]).unwrap();
    /// ```
    pub fn new(keywords: Vec<&'static str>) -> Result<Self> {
        check_keywords(&keywords)?;
        Ok(Self { keywords })
    }
}

impl TryFrom<&'static str> for Keywords {
    fn try_from(item: &'static str) -> Result<Self> {
        Keywords::new(item.split(',').collect())
    }

    type Error = Error;
}

fn check_keywords(keys: &Vec<&'static str>) -> Result<()> {
    if keys.is_empty() {
        Err(Error::KeywordNotSet)
    } else if keys.len() > 5 {
        Err(Error::KeywordMaxCapacity)
    } else {
        Ok(())
    }
}
