//! Represent a Google Trend Property

use strum_macros::{Display, EnumString, VariantNames};
/// Create a new Property.
///
/// The available property are :
/// - `web`, `images`, `news`, `froogle` (Google Shopping), and `youtube`
///
/// Returns a `Property` instance.
///
/// # Example
/// ```
/// # use rtrend::Property;
/// let property = Property::Web;
/// ```

#[derive(Eq, PartialEq, Display, Debug, EnumString, Clone, VariantNames)]
#[strum(serialize_all = "kebab_case")]
pub enum Property {
    #[strum(serialize = "")]
    Web,
    Images,
    News,
    Froogle,
    Youtube,
}
