# RieltorParser

RieltorParser is a Rust-based parser designed to extract comprehensive information about apartments listed on the rieltor.ua website from its HTML structure. This tool is specifically tailored to navigate the website's HTML, retrieve relevant data points, and structure them in a usable format.

## Parsing Process

1. **HTML Retrieval**: The parser fetches the HTML content of the rieltor.ua apartment listings.
2. **Data Extraction**: It identifies and extracts key details such as:
    - Apartment address
    - Price
    - Number of rooms
    - Square footage
    - Description
    - Contact information
3. **Data Structuring**: The extracted data is then structured into a JSON format for easy integration with other applications or databases.

## Usage

The results of the parsing process can be used for various purposes, including:
- Creating a database of apartment listings
- Analyzing market trends
- Providing insights for real estate research

> [!WARNING]
> Use only for educational or personal purposes.  

## Installation and Setup

To use RieltorParser, add the following to your `Cargo.toml`:

```toml
[dependencies]
rieltor_parser = "0.1.0"
```
## Output example

```text
- price
   - price_number: "9 000"
   - currency: "грн"
- address
   - street_value: "Петропавловсклівська"
   - house_number: "13/*"
   - value: "Київ"
   - value: "Подільський р-н"
- characteristics
   - room_count > number: "1"
   - area
      - number: "32"
      - number: "15"
      - number: "5"
   - floor_info
      - number: "3"
      - number: "9"
   - house_type_1 > value: "Українська цегла"
   - room_planning_1 > value: "Роздільне"
   - state_1 > value: "Хороший стан"
   - renewed > event_date: "вчора"
   - published > event_date: "1 тиж. тому"
   - views
      - number: "128"
      - number: "1"
      - number: "26"
- description > div_value: "Сдам в оренду 1-но кімнатну квартиру для орендарів без тварин. Всі необхідні меблі є , новий холодильник і пральна машинка. Квартира чиста і охайна. Є відеоогляд квартири."
- det_descr_text
   - house_value: "Українська цегла"
   - any: "в квартирі 1 кімната."
   - planning_value: "Роздільне"
   - state_value: "Хороший стан"
   - commission > number: "50"
- rieltor
   - phone_number: "05035*****"
   - rieltor_name > value: "***** Ірина"
   - rieltor_position > value: "Рієлтор"
```