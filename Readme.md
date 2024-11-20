# Rieltor.ua Apartment Parser

Rieltor.ua Apartment Parser is a Rust-based tool for parsing apartment listings from the Rieltor.ua website. It extracts detailed information about apartment listings and converts it into structured JSON data, suitable for further analysis or integration with other applications.

## Features
- Parses apartment details, including pricing, address, characteristics, description, and infrastructure.
- Supports single apartment and apartment list parsing.
- Saves parsed data in JSON format for easy consumption.
- Includes CLI commands for flexible usage.
- Provides support for personal and educational purposes only.
---
## Parsing Overview
This parser uses a custom-defined grammar (`grammar.pest`) to tokenize and extract apartment data from the Rieltor.ua HTML pages.
### What Is Parsed?
1. **General Information:**
   - **ID**: Unique identifier of the listing.
   - **Link**: Direct URL to the listing.
2. **Price:**
   - **Amount**: Apartment price as a number.
   - **Currency**: Currency type (e.g., "грн").
3. **Address:**
   - Street, house number, city, district.
4. **Characteristics:**
   - Room count.
   - Area (total, living, kitchen).
   - Floor information (current and maximum).
   - Building type, room planning, apartment condition.
5. **Description:**
   - General advert description.
   - Detailed description (house type, equipment, room availability, utilities, etc.).
6. **Permits and Special Attributes:**
   - Premium status, pet and child policies, commission details.
7. **Infrastructure:**
   - Nearby subway stations.
   - Local landmarks.
   - Residential complexes.
8. **Rieltor Details:**
   - Name, phone number, position, agency.
9. **Photos:**
   - URLs to apartment images.
---
### Parsing Process
1. **Input:**
   
   The parser processes either:
   - HTML files containing apartment data.
   - URLs pointing to listings on Rieltor.ua.
2. **Tokenization:**

   The grammar file defines the rules to split apartment listings into tokens such as `id`, `price`, `address`, `description`, etc.
   Example of token breakdown:
   -	id > number: “11639857”
   -	price
   -	price_number: “34 000”
   -	currency: “грн”
3. **Output:**

   The parsed data is saved in a structured JSON format, which looks like this:
   ```json
   {
     "id": "11639857",
     "price": {
       "price_number": 34000,
       "currency": "Uah"
     },
     "address": {
       "street": "Менделєєва вул.",
       "house_number": "1111",
       "city": "Київ",
       "district": "Печерський р-н"
     },
     "characteristics": {
       "room_count": 2,
       "area": {
         "total": 36.0,
         "living": 15.0,
         "kitchen": 17.0
       },
       "floor": 3,
       "max_floor": 6,
       "house_type": "Бетонно монолітний",
       "room_planning": "Роздільне",
       "state": "Дизайнерський ремонт"
     },
     "description": {
       "advert_description": "Без комісії! Довгострокова оренда квартири...",
       "details_description": "Будинок - Бетонно монолітний, в квартирі 2 кімнати..."
     },
     "rieltor": {
       "rieltor_name": "Малишко Максим",
       "rieltor_phone_number": "0991232323",
       "rieltor_position": "Рієлтор"
     },
     "photo": [
       "https://img.lunstatic.net/rieltor-offer-1600x1200/offers/433/33/3/????.jpeg"
     ]
   }
   ```
## CLI Usage
### Commands
1.	`parse` - Parses a single HTML file or URL and outputs JSON data.
*Arguments:*          
     - `<source>` - Specify the path to the HTML or URL file to parse.   
     - `<output>` - The path for saving the parsed result in JSON file. The file name is optional: it can be automatically generated.
---
**Examples:**
   ```
   cargo run parse https://rieltor.ua/flats-rent/view/12345678 apartment.json
   cargo run parse fetched_apartment.html
   ```
---
2.	`parse_list` - Parses a HTML file or fetched HTML from URL with list of apartments and displays their contents.  
      *Arguments:*
   - `<source>` - Specify the path to the HTML or URL file to parse.
   - `<output>` - The path for saving the parsed result in directory. The directory name is optional: it can be automatically generated in project output directory.
---
**Examples:**
   ```
   cargo run parse_list https://rieltor.ua/poltava/flats-rent/?price_min=8750"&"price_max=15000
   cargo run parse_list fetched_apartment_list.html
   ```
---
3.	`credits` - Shows credits and authorship information.
---
4.	`help` - Displays this help information.
---

## Grammar
The parsing logic relies on a well-defined grammar file (grammar.pest) to extract and organize data. The grammar breaks down HTML into tokens like price, address, characteristics, etc., for structured processing.

Here’s a visual representation of the grammar:

**Apartment:**
   - **ID**: Unique identifier of the apartment.
   - **Link**: Link to the apartment's webpage.
   - **Price**: The price of the apartment, including currency.
     - **Price number**: Numeric value of the price.
     - **Currency**: Currency of the price.
   - **Address**: Address details of the apartment.
     - **Street**: The name of the street where the apartment is located.
     - **House Number**: The specific number of the building.
     - **City**: The city of the listing.
     - **District**: The administrative district of the apartment.
   - **Characteristics**: Various characteristics of the apartment, such as rooms, area, and floor.
     - **Room Count**: The number of rooms in the apartment.
     - **Area**:
        - **Total**: Total square meters.
        - **Living**: Square meters of the living area.
        - **Kitchen**: Square meters of the kitchen area.
     - **Floor Information**:
        - **Current Floor**: The floor on which the apartment is located.
        - **Max Floor**: Total floors in the building.
     - **House Type**: E.g., "Concrete Monolithic".
     - **Room Planning**: E.g., "Separate".
     - **State**: E.g., "Designer Renovation".
     - **Statistics**:
        - **Renewed Date**: When the listing was last updated.
        - **Published Date**: When the listing was first published.
        - **Views**:
           - **Total Views**: Total number of views.
           - **Today**: Views for the current day.
           - **Yesterday**: Views for the previous day.
   - **Description**: Description of the apartment.
     - **Advert Description**: Overview of the apartment.
     - **Details Description**: Additional details like furniture, repairs, and utilities.
   - **Permits**: Permits or special tags associated with the apartment.
   - **Premium Advert**: Indicates if the listing is premium.
     - **Short Period**: Specifies if short-term rent is allowed.
     - **Commission**:
        - **Rate**: Commission percentage.
        - **Price**: Commission cost (if applicable).
     - **Children Allowed**: Indicates if children are allowed.
     - **Pets Allowed**: Indicates if pets are allowed.
     - **Bargaining**: Indicates if price negotiations are possible.
   - **Infrastructure**: Information about nearby infrastructure.
     - **Subway Stations**: Nearby stations with their respective lines.
     - **Landmarks**: Important landmarks near the apartment.
     - **Residential Complex**: Name of the residential complex.
   - **Rieltor**: Details about the realtor managing the apartment.
     - **Name**: The name of the realtor.
     - **Phone Number**: The realtor's contact number.
     - **Position**: E.g., "Realtor", "Owner".
     - **Agency**: The realtor's agency.
   - **Photo**: List of photo URLs for the apartment.
     - URLs to the apartment's photos.
---

### License and Usage Restrictions
This project is intended solely for personal and educational use. It must not be used for commercial purposes or violate the terms of service of Rieltor.ua.

### Limitations
This tool is designed specifically for parsing the structure of Rieltor.ua's website. Changes to the website's structure may require updates to the parsing logic.

### Credits
-	Author: Denys Shvachka
-	Email: d.shvachka@ukma.edu.ua
-	Created as part of the Rust course at the National University of “Kyiv-Mohyla Academy.”