use rieltor_parser::{parse_apartment, save_to_json};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let content = "https://rieltor.ua/flats-rent/view/11604637/";

    match parse_apartment(content).await {
        Ok(apartment) => {
            save_to_json(&apartment, "apartment.json")?;
            println!("Apartment data saved to JSON file: apartment.json");
        }
        Err(e) => {
            eprintln!("Failed to parse apartment data: {}", e);
        }
    }

    Ok(())
}
