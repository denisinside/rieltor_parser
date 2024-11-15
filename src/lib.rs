mod appartment;

use crate::appartment::*;
use anyhow::{anyhow, Result};
use pest::Parser;
use pest_derive::Parser;
use reqwest::Url;
use std::fs;
use std::fs::File;
use std::io::Write;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct ApartmentParser;

pub fn parse(html_content: &str) -> Result<Apartment> {
    let mut apartment = Apartment::default();
    let parsed = ApartmentParser::parse(Rule::document, html_content)
        .map_err(|_| anyhow!("Parsing failed"))?;

    println!("{:?}", parsed);

    for pair in parsed {
        match pair.as_rule() {
            Rule::id => apartment.id = pair.as_str().to_string(),
            Rule::price => {
                let mut inner = pair.into_inner();
                let price = Price {
                    price_number : inner
                    .next()
                    .unwrap()
                    .as_str()
                    .replace(" ", "")
                    .parse::<u32>()?,
                    currency: match inner.next().unwrap().as_str() {
                    "$" => Currency::Usd,
                    "€" => Currency::Eur,
                    _ => Currency::Uah,
                    }
                };
                apartment.price = price;
            }
            Rule::address => {
                let mut inner = pair.into_inner();
                let street = inner.next().unwrap().as_str().to_string();
                let house_number = inner.next().unwrap().as_str().to_string();
                let city = inner.next().unwrap().as_str().to_string();
                let district = inner.next().unwrap().as_str().to_string();
                apartment.address = Address {
                    street,
                    house_number,
                    city,
                    district,
                };
            }
            Rule::description_text => {
                apartment.description.advert_description =
                    pair.as_str().to_string().replace("<br />", "")
            }
            Rule::characteristics => {
                let mut inner = pair.into_inner();

                apartment.characteristics.room_count = inner
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .parse::<u32>()?;

                let mut area_pairs = inner.next().unwrap().into_inner();
                apartment.characteristics.area = Area {
                    total: area_pairs.next().unwrap().as_str().parse::<f32>()?,
                    living: area_pairs.next().unwrap().as_str().parse::<f32>()?,
                    kitchen: area_pairs.next().unwrap().as_str().parse::<f32>()?,
                };

                let mut floor_info_pairs = inner.next().unwrap().into_inner();
                apartment.characteristics.floor =
                    floor_info_pairs.next().unwrap().as_str().parse::<u32>()?;
                apartment.characteristics.max_floor =
                    floor_info_pairs.next().unwrap().as_str().parse::<u32>()?;

                apartment.characteristics.statistics.renewed = inner
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .to_string();
                apartment.characteristics.statistics.published = inner
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .to_string();
                let mut views_pairs = inner.next().unwrap().into_inner();
                apartment.characteristics.statistics.views = Views {
                    total: views_pairs.next().unwrap().as_str().parse::<u32>()?,
                    today: views_pairs.next().unwrap().as_str().parse::<u32>()?,
                    yesterday: views_pairs.next().unwrap().as_str().parse::<u32>()?,
                };
            }
            Rule::label_section => {
                for pair in pair.into_inner() {
                    match pair.as_rule() {
                        Rule::premium_advert => apartment.permits.premium_advert = true,
                        Rule::short_period => apartment.permits.short_period = true,
                        Rule::allow_children => apartment.permits.allow_children = true,
                        Rule::allow_pets => apartment.permits.allow_pets = true,
                        Rule::subway_station => {
                            let mut inner = pair.into_inner();
                            apartment.infrastructure.subway_station.push(SubwayStation {
                                line: match inner.next().unwrap().as_str() {
                                    "green" => SubwayLine::Green,
                                    "blue" => SubwayLine::Blue,
                                    "red" => SubwayLine::Red,
                                    _ => {
                                        return Err(anyhow!(
                                            "Parsing error: unexpected metro line."
                                        ));
                                    }
                                },
                                name: inner.next().unwrap().as_str().to_string(),
                            })
                        }
                        Rule::landmark => apartment
                            .infrastructure
                            .landmarks
                            .push(pair.into_inner().next().unwrap().as_str().to_string()),
                        Rule::residential_complex => {
                            apartment.infrastructure.residential_complex =
                                Option::from(pair.into_inner().next().unwrap().as_str().to_string())
                        }
                        Rule::commission => {
                            let mut inner = pair.into_inner();
                            if let Some(commission) = inner.next() {
                                match commission.as_rule() {
                                    Rule::number => {
                                        apartment.permits.commission.commission_rate =
                                            commission.as_str().parse::<u32>()?
                                    }
                                    Rule::price_number => {
                                        let price_number = commission.as_str().parse::<u32>()?;
                                        let currency = match inner.next().unwrap().as_str() {
                                            "USD" => Currency::Usd,
                                            "€" => Currency::Eur,
                                            _ => Currency::Uah,
                                        };
                                        apartment.permits.commission.commission_price =
                                            Option::from(Price {
                                                price_number,
                                                currency,
                                            });
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            Rule::details_description => {
                let mut inner = pair.into_inner();
                let description = inner.next().unwrap();
                apartment.description.details_description = description.as_str().to_string();
                for pair in description.into_inner() {
                    match pair.as_rule() {
                        Rule::bargain => apartment.permits.bargain = true,
                        Rule::house_value => {
                            apartment.characteristics.house_type =
                                Option::from(pair.as_str().to_string())
                        }
                        Rule::planning_value => {
                            apartment.characteristics.room_planning =
                                Option::from(pair.as_str().to_string())
                        }
                        Rule::state_value => {
                            apartment.characteristics.state =
                                Option::from(pair.as_str().to_string())
                        }
                        _ => {}
                    }
                }
            }
            Rule::rieltor => {
                let mut inner = pair.into_inner();
                let rieltor_phone_number = inner.next().unwrap().as_str().to_string();
                let rieltor_name = inner
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .to_string();
                let rieltor_position = inner
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .to_string();
                if let Some(rieltor_agency_container) = inner.next() {
                    let rieltor_agency = rieltor_agency_container
                        .into_inner()
                        .next()
                        .unwrap()
                        .as_str()
                        .to_string();
                    apartment.rieltor = Rieltor {
                        rieltor_name,
                        rieltor_phone_number,
                        rieltor_position,
                        rieltor_agency: Some(rieltor_agency),
                    };
                } else {
                    apartment.rieltor = Rieltor {
                        rieltor_name,
                        rieltor_phone_number,
                        rieltor_position,
                        rieltor_agency: None,
                    };
                }
            }
            Rule::photo_list => {
                for photo in pair.into_inner() {
                    apartment.photo.push(photo.as_str().to_string());
                }
            }
            _ => {}
        }
    }

    Ok(apartment)
}

pub fn save_to_json(apartment: &Apartment, file_path: &str) -> Result<()> {
    let json_data = serde_json::to_string_pretty(apartment)?;
    let mut file = File::create(file_path)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}

pub fn load_html(file_path: &str) -> Result<String> {
    fs::read_to_string(file_path).map_err(|e| anyhow!("Failed to load file: {}", e))
}

pub async fn fetch_html_from_url(url: &str) -> Result<String> {
    if !url.starts_with("https://rieltor.ua/flats-rent/view/") {
        return Err(anyhow!("Invalid URL format"));
    }
    let response = reqwest::get(url).await?;
    let content = response.text().await?;

    Ok(content)
}

pub async fn parse_apartment(src: &str) -> Result<Apartment> {
    if Url::parse(src).is_ok() {
        let html_content = fetch_html_from_url(src).await?;
        let mut html_file = File::create("fetched_apartment.html")?;
        html_file.write_all(html_content.as_bytes())?;
        println!("HTML content saved to 'fetched_apartment.html'");
        parse(&html_content)
    } else {
        let html_content = load_html(src)?;
        parse(&html_content)
    }
}
