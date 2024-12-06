use serde::{Deserialize, Serialize};

/// Enum representing supported currencies for apartment prices.
#[derive(Serialize, Deserialize, Debug)]
pub enum Currency {
    /// Ukrainian Hryvnia
    Uah,
    /// United States Dollar
    Usd,
    /// Euro
    Eur,
}

/// Enum representing subway lines associated with apartment locations.
#[derive(Serialize, Deserialize, Debug)]
pub enum SubwayLine {
    /// Red subway line
    Red,
    /// Green subway line
    Green,
    /// Blue subway line
    Blue,
}

/// The main structure representing an apartment.
#[derive(Serialize, Deserialize, Debug)]
pub struct Apartment {
    /// Unique identifier of the apartment.
    pub _id: String,
    /// Link to the apartment's webpage.
    pub link: String,
    /// The price of the apartment, including currency.
    pub price: Price,
    /// Address details of the apartment.
    pub address: Address,
    /// Various characteristics of the apartment, such as rooms, area, and floor.
    pub characteristics: Characteristics,
    /// Description of the apartment.
    pub description: Description,
    /// Permits or special tags associated with the apartment.
    pub permits: Permits,
    /// Information about nearby infrastructure.
    pub infrastructure: Infrastructure,
    /// Details about the realtor managing the apartment.
    pub rieltor: Rieltor,
    /// List of photo URLs for the apartment.
    pub photo: Vec<String>,
}
/// Structure representing the price of an apartment.
#[derive(Serialize, Deserialize, Debug)]
pub struct Price {
    /// Numeric value of the price.
    pub price_number: u32,
    /// Currency of the price.
    pub currency: Currency,
}

/// Structure representing the address of an apartment.
#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    /// Street name.
    pub street: String,
    /// House number, potentially including letters or fractions.
    pub house_number: String,
    /// City name.
    pub city: String,
    /// District name.
    pub district: String,
}

/// Structure representing permissions or tags associated with an apartment.
#[derive(Serialize, Deserialize, Debug)]
pub struct Permits {
    /// Indicates if the apartment is advertised as premium.
    pub premium_advert: bool,
    /// Indicates if the apartment is available for short-term rentals.
    pub short_period: bool,
    /// Commission details for the apartment.
    pub commission: Commission,
    /// Indicates if children are allowed.
    pub allow_children: bool,
    /// Indicates if pets are allowed.
    pub allow_pets: bool,
    /// Indicates if bargaining is allowed.
    pub bargain: bool,
}

/// Structure representing commission details for an apartment.
#[derive(Serialize, Deserialize, Debug)]
pub struct Commission {
    /// Commission rate in percentage.
    pub commission_rate: u32,
    /// Optional fixed price for the commission.
    pub commission_price: Option<Price>,
}

/// Structure representing nearby infrastructure details.
#[derive(Serialize, Deserialize, Debug)]
pub struct Infrastructure {
    /// List of nearby subway stations.
    pub subway_station: Vec<SubwayStation>,
    /// List of notable landmarks near the apartment.
    pub landmarks: Vec<String>,
    /// Optional name of the residential complex.
    pub residential_complex: Option<String>,
}

/// Structure representing a subway station.
#[derive(Serialize, Deserialize, Debug)]
pub struct SubwayStation {
    /// Name of the subway station.
    pub name: String,
    /// The subway line associated with the station.
    pub line: SubwayLine,
}

/// Structure representing the main characteristics of an apartment.
#[derive(Serialize, Deserialize, Debug)]
pub struct Characteristics {
    /// Number of rooms in the apartment.
    pub room_count: u32,
    /// Area of the apartment, including total, living, and kitchen areas.
    pub area: Area,
    /// Floor number of the apartment.
    pub floor: u32,
    /// Total number of floors in the building.
    pub max_floor: u32,
    /// Optional type of the building (e.g., brick, panel).
    pub house_type: Option<String>,
    /// Optional room planning information.
    pub room_planning: Option<String>,
    /// Optional state of the apartment (e.g., renovated, needs repairs).
    pub state: Option<String>,
    /// Advertisement statistics for the listing.
    pub statistics: AdvertStatistics,
}

/// Structure representing the area details of an apartment.
#[derive(Serialize, Deserialize, Debug)]
pub struct Area {
    /// Total area of the apartment in square meters.
    pub total: f32,
    /// Living area of the apartment in square meters.
    pub living: f32,
    /// Kitchen area of the apartment in square meters.
    pub kitchen: f32,
}

/// Structure representing advertisement statistics.
#[derive(Serialize, Deserialize, Debug)]
pub struct AdvertStatistics {
    /// Date the listing was last renewed.
    pub renewed: String,
    /// Date the listing was published.
    pub published: String,
    /// Number of views for the listing.
    pub views: Views,
}

/// Structure representing view statistics for a listing.
#[derive(Serialize, Deserialize, Debug)]
pub struct Views {
    /// Total number of views.
    pub total: u32,
    /// Number of views today.
    pub today: u32,
    /// Number of views yesterday.
    pub yesterday: u32,
}

/// Structure representing the description of an apartment.
#[derive(Serialize, Deserialize, Debug)]
pub struct Description {
    /// Main description of the apartment.
    pub advert_description: String,
    /// Additional details about the apartment.
    pub details_description: String,
}

/// Structure representing the realtor responsible for the listing.
#[derive(Serialize, Deserialize, Debug)]
pub struct Rieltor {
    /// Name of the realtor.
    pub rieltor_name: String,
    /// Phone number of the realtor.
    pub rieltor_phone_number: String,
    /// Position of the realtor in the agency.
    pub rieltor_position: String,
    /// Optional name of the realtor's agency.
    pub rieltor_agency: Option<String>,
}

impl Apartment {
    pub fn new() -> Self {
        Self {
            _id: String::new(),
            link: String::new(),
            price: Price::new(),
            address: Address::new(),
            characteristics: Characteristics::new(),
            description: Description::new(),
            infrastructure: Infrastructure::new(),
            permits: Permits::new(),
            rieltor: Rieltor::new(),
            photo: Vec::new(),
        }
    }
}
impl Default for Apartment {
    fn default() -> Self {
        Self::new()
    }
}

impl Price {
    pub fn new() -> Self {
        Self {
            price_number: 0,
            currency: Currency::Uah,
        }
    }
}
impl Default for Price {
    fn default() -> Self {
        Self::new()
    }
}

impl Address {
    pub fn new() -> Self {
        Self {
            street: String::new(),
            house_number: String::new(),
            city: String::new(),
            district: String::new(),
        }
    }
}

impl Default for Address {
    fn default() -> Self {
        Self::new()
    }
}

impl Description {
    pub fn new() -> Self {
        Self {
            advert_description: String::new(),
            details_description: String::new(),
        }
    }
}

impl Default for Description {
    fn default() -> Self {
        Self::new()
    }
}

impl Permits {
    pub fn new() -> Self {
        Self {
            premium_advert: false,
            short_period: false,
            bargain: false,
            allow_pets: false,
            allow_children: false,
            commission: Commission::new(),
        }
    }
}

impl Default for Permits {
    fn default() -> Self {
        Self::new()
    }
}

impl Commission {
    pub fn new() -> Self {
        Self {
            commission_rate: 0,
            commission_price: None,
        }
    }
}

impl Default for Commission {
    fn default() -> Self {
        Self::new()
    }
}

impl Infrastructure {
    pub fn new() -> Self {
        Self {
            subway_station: Vec::new(),
            landmarks: Vec::new(),
            residential_complex: None,
        }
    }
}

impl Default for Infrastructure {
    fn default() -> Self {
        Self::new()
    }
}

impl Characteristics {
    pub fn new() -> Self {
        Self {
            room_count: 1,
            area: Area::new(),
            floor: 1,
            max_floor: 1,
            house_type: None,
            room_planning: None,
            state: None,
            statistics: AdvertStatistics::new(),
        }
    }
}

impl Default for Characteristics {
    fn default() -> Self {
        Self::new()
    }
}

impl Area {
    pub fn new() -> Self {
        Self {
            total: 0.,
            living: 0.,
            kitchen: 0.,
        }
    }
}

impl Default for Area {
    fn default() -> Self {
        Self::new()
    }
}

impl AdvertStatistics {
    pub fn new() -> Self {
        Self {
            renewed: String::new(),
            published: String::new(),
            views: Views::new(),
        }
    }
}
impl Default for AdvertStatistics {
    fn default() -> Self {
        Self::new()
    }
}

impl Views {
    pub fn new() -> Self {
        Self {
            total: 0,
            today: 0,
            yesterday: 0,
        }
    }
}

impl Default for Views {
    fn default() -> Self {
        Self::new()
    }
}

impl Rieltor {
    pub fn new() -> Self {
        Self {
            rieltor_name: String::new(),
            rieltor_phone_number: String::new(),
            rieltor_position: String::new(),
            rieltor_agency: None,
        }
    }
}

impl Default for Rieltor {
    fn default() -> Self {
        Self::new()
    }
}
