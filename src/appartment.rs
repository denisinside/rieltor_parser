use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Currency {
    UAH,
    USD,
    EUR,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SubwayLine {
    Red,
    Green,
    Blue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Apartment {
    pub id: String,
    pub price: Price,
    pub address: Address,
    pub characteristics: Characteristics,
    pub description: Description,
    pub permits: Permits,
    pub infrastructure: Infrastructure,
    pub rieltor: Rieltor,
    pub photo: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Price {
    pub price_number: u32,
    pub currency: Currency,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    pub street: String,
    pub house_number: String,
    pub city: String,
    pub district: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Permits {
    pub premium_advert: bool,
    pub short_period: bool,
    pub commission: Commission,
    pub allow_children: bool,
    pub allow_pets: bool,
    pub bargain: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Commission {
    pub commission_rate: u32,
    pub commission_price: Option<Price>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Infrastructure {
    pub subway_station: Vec<SubwayStation>,
    pub landmarks: Vec<String>,
    pub residential_complex: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubwayStation {
    pub name: String,
    pub line: SubwayLine,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Characteristics {
    pub room_count: u32,
    pub area: Area,
    pub floor: u32,
    pub max_floor: u32,
    pub house_type: Option<String>,
    pub room_planning: Option<String>,
    pub state: Option<String>,
    pub statistics: AdvertStatistics,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Area {
    pub total: f32,
    pub living: f32,
    pub kitchen: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdvertStatistics {
    pub renewed: String,
    pub published: String,
    pub views: Views,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Views {
    pub total: u32,
    pub today: u32,
    pub yesterday: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Description {
    pub advert_description: String,
    pub details_description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rieltor {
    pub rieltor_name: String,
    pub rieltor_phone_number: String,
    pub rieltor_position: String,
    pub rieltor_agency: Option<String>,
}

impl Apartment {
    pub fn new() -> Self {
        Self {
            id: String::new(),
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

impl Price {
    pub fn new() -> Self {
        Self {
            price_number: 0,
            currency: Currency::UAH,
        }
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

impl Description {
    pub fn new() -> Self {
        Self {
            advert_description: String::new(),
            details_description: String::new(),
        }
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

impl Commission {
    pub fn new() -> Self {
        Self {
            commission_rate: 0,
            commission_price: None,
        }
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

impl Area {
    pub fn new() -> Self {
        Self {
            total: 0.,
            living: 0.,
            kitchen: 0.,
        }
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

impl Views {
    pub fn new() -> Self {
        Self {
            total: 0,
            today: 0,
            yesterday: 0,
        }
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
