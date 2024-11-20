/// Module containing types for parsing apartments.
pub mod apartment;

use crate::apartment::*;
use anyhow::{anyhow, Result};
use chrono::Local;
use futures::future::join_all;
use pest::Parser;
use pest_derive::Parser;
use reqwest::Url;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{env, fs};

/// Parser struct for handling the grammar defined in `grammar.pest`.
#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct ApartmentParser;

/// Parses the HTML content of a single apartment page into an `Apartment` struct.
///
/// # Arguments
/// * `html_content` - The HTML source as a string.
///
/// # Returns
/// A `Result` containing the parsed `Apartment` struct or an error if parsing fails.
pub fn parse(html_content: &str) -> Result<Apartment> {
    let mut apartment = Apartment::default();
    let parsed = ApartmentParser::parse(Rule::document, html_content)
        .map_err(|_| anyhow!("Parsing failed"))?;

    for pair in parsed {
        match pair.as_rule() {
            Rule::id => {
                apartment.id = pair.into_inner().next().unwrap().as_str().to_string();
                apartment.link = format!("https://rieltor.ua/flats-rent/view/{}/", apartment.id);
            }
            Rule::price => {
                let mut inner = pair.into_inner();
                let price = Price {
                    price_number: inner
                        .next()
                        .unwrap()
                        .as_str()
                        .replace(" ", "")
                        .parse::<u32>()?,
                    currency: match inner.next().unwrap().as_str() {
                        "$" => Currency::Usd,
                        "€" => Currency::Eur,
                        _ => Currency::Uah,
                    },
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

/// Saves an apartment to a JSON file.
///
/// # Arguments
/// * `apartment` - The apartment to save.
/// * `file_path` - Path to the output directory or file.
///
/// # Returns
/// A `Result` with the full path of the saved file.
pub fn save_to_json(apartment: &Apartment, file_path: &str) -> Result<String> {
    let json_data = serde_json::to_string_pretty(apartment)?;
    let mut final_path = PathBuf::from(file_path);

    if final_path.is_relative() {
        let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
        final_path = PathBuf::from(project_dir).join("output").join(final_path);
    }

    if final_path.is_dir() {
        final_path = final_path.join("").join(&apartment.id);
    }

    if final_path.file_name().is_none() {
        final_path = final_path.join(&apartment.id);
    }

    final_path.set_extension("json");
    let final_path_string = final_path.to_str().unwrap().to_string();

    let mut file = File::create(final_path)?;
    file.write_all(json_data.as_bytes())?;
    Ok(final_path_string)
}

/// Fetches and parses apartment details from a URL or file.
///
/// # Arguments
/// * `src` - A URL or path to a local HTML file.
///
/// # Returns
/// A `Result` containing the parsed `Apartment`.
pub fn save_apartments_to_directory(apartments: &[Apartment], file_path: &str) -> Result<String> {
    let mut final_path = PathBuf::from(file_path);
    if final_path.is_file() {
        return Err(anyhow!(
            "The specified path is a file. Please provide a directory."
        ));
    }

    if final_path.file_name().is_none() {
        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        final_path = final_path.join(timestamp);
    }

    if final_path.is_relative() {
        let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
        final_path = PathBuf::from(project_dir).join("output").join(final_path);
    }

    fs::create_dir_all(&final_path).map_err(|e| {
        anyhow!(
            "Failed to create output directory: {}. Error: {}",
            final_path.display(),
            e
        )
    })?;

    println!("{}", final_path.display());
    apartments.iter().for_each(|apartment| {
        if let Err(e) = save_to_json(apartment, final_path.to_str().unwrap()) {
            eprintln!(
                "Failed to save apartment to file: {} apartment id. Error: {}",
                apartment.id, e
            );
        }
    });

    Ok(final_path.to_str().unwrap().to_string())
}

/// Loads the content of an HTML file from the specified path.
///
/// # Arguments
/// * `file_path` - Path to the HTML file.
///
/// # Returns
/// A `Result` containing the content of the file as a `String`, or an error if the file cannot be read.
pub fn load_html(file_path: &str) -> Result<String> {
    fs::read_to_string(file_path).map_err(|e| anyhow!("Failed to load file: {}", e))
}

/// Fetches the HTML content of a single apartment page from the given URL.
///
/// # Arguments
/// * `url` - The URL of the apartment page.
///
/// # Returns
/// A `Result` containing the HTML content as a `String`, or an error if the URL is invalid
/// or the request fails.
///
/// # Errors
/// - Returns an error if the URL does not match the expected `apartment_link` grammar rule.
/// - Returns an error if the HTTP request fails or if the response cannot be parsed.
pub async fn fetch_apartment_html_from_url(url: &str) -> Result<String> {
    ApartmentParser::parse(Rule::apartment_link, url)
        .map_err(|_| anyhow!("Incorrect apartment link."))?;
    let response = reqwest::get(url).await?;
    let content = response.text().await?;
    Ok(content)
}

/// Fetches the HTML content of an apartment list page from the given URL.
///
/// # Arguments
/// * `url` - The URL of the apartment list page.
///
/// # Returns
/// A `Result` containing the HTML content as a `String`, or an error if the URL is invalid
/// or the request fails.
///
/// # Errors
/// - Returns an error if the URL does not match the expected `apartment_list_link` grammar rule.
/// - Returns an error if the HTTP request fails or if the response cannot be parsed.
pub async fn fetch_apartment_list_html_from_url(url: &str) -> Result<String> {
    ApartmentParser::parse(Rule::apartment_list_link, url)
        .map_err(|_| anyhow!("Incorrect apartment list link."))?;
    let response = reqwest::get(url).await?;
    let content = response.text().await?;
    Ok(content)
}

/// Parses a single apartment from the specified source.
///
/// # Arguments
/// * `src` - The source, which can be either a URL or a file path.
///
/// # Returns
/// A `Result` containing the parsed `Apartment` struct, or an error if parsing fails.
///
/// # Behavior
/// - If `src` is a valid URL, the method fetches the HTML content from the URL.
/// - If `src` is a file path, the method reads the HTML content from the file.
/// - The HTML content is then parsed into an `Apartment` struct.
pub async fn parse_apartment(src: &str) -> Result<Apartment> {
    if Url::parse(src).is_ok() && fs::read_to_string(src).is_err() {
        let html_content = fetch_apartment_html_from_url(src).await?;
        parse(&html_content)
    } else {
        let html_content = load_html(src)?;
        parse(&html_content)
    }
}

/// Parses a list of apartments from the specified source.
///
/// # Arguments
/// * `src` - The source, which can be either a URL or a file path.
///
/// # Returns
/// A `Result` containing a vector of `Apartment` structs, or an error if parsing fails.
///
/// # Behavior
/// - If `src` is a valid URL, the method fetches the HTML content from the URL.
/// - If `src` is a file path, the method reads the HTML content from the file.
/// - Extracts apartment links from the content and parses each apartment concurrently.
///
/// # Errors
/// - Returns an error if the source cannot be read or if parsing fails for any apartment.
pub async fn parse_apartment_list(src: &str) -> Result<Vec<Apartment>> {
    let html_content = if Url::parse(src).is_ok() && fs::read_to_string(src).is_err() {
        fetch_apartment_list_html_from_url(src).await?
    } else {
        load_html(src)?
    };
    let parsed = ApartmentParser::parse(Rule::apartment_list, html_content.as_str())?;

    let links: Vec<String> = parsed
        .flat_map(|pair| {
            if pair.as_rule() == Rule::apartment_link {
                Some(pair.as_str().to_string())
            } else {
                None
            }
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    let tasks = links.into_iter().map(|link| async move {
        let html_content = fetch_apartment_html_from_url(&link).await?;
        parse(html_content.as_str())
    });

    let results: Vec<Result<Apartment>> = join_all(tasks).await;

    let apartments: Vec<Apartment> = results.into_iter().collect::<Result<Vec<_>, _>>()?;

    Ok(apartments)
}
