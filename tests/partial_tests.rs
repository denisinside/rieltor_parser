#[cfg(test)]
mod tests {
    use pest::Parser;
    use rieltor_parser::{ApartmentParser, Rule};

    #[test]
    fn test_price() {
        let content = r#"<div class="offer-view-price-title">35 000 грн/міс</div>"#;

        let mut pairs =
            ApartmentParser::parse(Rule::price, content).expect("Unsuccessful parsing.");
        assert_eq!(1, pairs.clone().count());
        let mut price_value = pairs.next().unwrap().into_inner();
        assert_eq!("35 000", price_value.next().unwrap().as_str());
        assert_eq!("грн", price_value.next().unwrap().as_str());
    }

    #[test]
    fn test_address() {
        let content = r#"<div class="offer-view-address">
                Петропавловсклівська, 13/8            </div>
                        <div class="offer-view-section-title2">
                <a href="https://rieltor.ua/flats-rent/">Оренда квартир</a>            </div>
            <div class="offer-view-region">
                <a class="address-link" href="https://rieltor.ua/flats-rent/">Київ</a>,<a class="address-link" href="/flats-rent/%D0%9F%D0%BE%D0%B4%D1%96%D0%BB%D1%8C%D1%81%D1%8C%D0%BA%D0%B8%D0%B9-d78/" data-analytics-event="card-click-region">Подільський р-н</a> "#;

        let mut pairs =
            ApartmentParser::parse(Rule::address, content).expect("Unsuccessful parsing.");
        assert_eq!(1, pairs.clone().count());
        let mut address = pairs.next().unwrap().into_inner();
        assert_eq!("Петропавловсклівська", address.next().unwrap().as_str());
        let house = address.next().unwrap();
        assert_eq!(Rule::house_number, house.as_rule());
        assert_eq!("13/8", house.as_str());
        assert_eq!("Київ", address.next().unwrap().as_str());
        assert_eq!("Подільський р-н", address.next().unwrap().as_str());
    }

    #[test]
    fn test_label_section() {
        let content = r#"<div class="offer-view-labels uilabels">
   <span class="uilabel -premium">
   ПРЕМІУМ                    </span>
   <span class="uilabel -green">
   Комісія 50%                    </span>
   <a class="uilabel -link -icon -subway-blue" href="test" data-analytics-event="card-click-subway_chip">
      </svg>
      <span>Контрактова площа</span>
   </a>
   <a class="uilabel -link" data-analytics-event="card-click-landmark_chip" href="test">
   Рибальський острів                            </a>
   <a class="uilabel -link" data-analytics-event="card-click-newhouse_chip" href="test">
   ЖК Житловий район Rybalsky                        </a>
   <a class="uilabel -link" data-analytics-event="card-click-allow_children_chip" href="https://rieltor.ua/flats-rent/?allow_children=1">
   <img src="/img/filters/allow_children.svg" width="20px">Можна з дітьми                    </a>
   <a class="uilabel -link" data-analytics-event="card-click-allow_pets_chip" href="https://rieltor.ua/flats-rent/?allow_pets=1">
   <img src="/img/filters/allow_pets2.svg" width="20px">Можна з тваринами                    </a>
</div>"#;

        let mut pairs =
            ApartmentParser::parse(Rule::label_section, content).expect("Unsuccessful parsing.");
        assert_eq!(1, pairs.clone().count());
        let mut label_section = pairs.next().unwrap().into_inner();
        assert_eq!(7, label_section.clone().count());

        let premium_advert = label_section.next().unwrap();
        assert_eq!(Rule::premium_advert, premium_advert.as_rule());
        assert_eq!("ПРЕМІУМ", premium_advert.as_str());

        let commission = label_section.next().unwrap();
        assert_eq!(Rule::commission, commission.as_rule());
        assert_eq!("50", commission.into_inner().next().unwrap().as_str());

        let subway_station = label_section.next().unwrap();
        assert_eq!(Rule::subway_station, subway_station.as_rule());
        let mut subway_station_pairs = subway_station.into_inner();
        assert_eq!("blue", subway_station_pairs.next().unwrap().as_str());
        assert_eq!(
            "Контрактова площа",
            subway_station_pairs.next().unwrap().as_str()
        );

        let landmark = label_section.next().unwrap();
        assert_eq!(Rule::landmark, landmark.as_rule());
        let landmark_value = landmark.into_inner();
        assert_eq!(
            "Рибальський острів",
            landmark_value.into_iter().next().unwrap().as_str()
        );

        let residential_complex = label_section.next().unwrap();
        assert_eq!(Rule::residential_complex, residential_complex.as_rule());
        let residential_complex_value = residential_complex.into_inner();
        assert_eq!(
            "ЖК Житловий район Rybalsky",
            residential_complex_value
                .into_iter()
                .next()
                .unwrap()
                .as_str()
        );

        let allow_children = label_section.next().unwrap();
        assert_eq!(Rule::allow_children, allow_children.as_rule());
        assert_eq!("Можна з дітьми", allow_children.as_str());

        let allow_pets = label_section.next().unwrap();
        assert_eq!(Rule::allow_pets, allow_pets.as_rule());
        assert_eq!("Можна з тваринами", allow_pets.as_str());
    }

    #[test]
    fn test_characteristics() {
        let content = r###"<div class="offer-view-details-column">
   <div class="offer-view-details-row"> </svg>
      <span>
      <a href="https://rieltor.ua/flats-rent/1-room/">1 кімната</a>
      </span>
   </div>
   <div class="offer-view-details-row">
      </svg>
      <span>32 / 15 / 5 м²</span>
   </div>
   <div class="offer-view-details-row">
      </svg>
      <span>поверх 3 з 9</span>
   </div>
</div>
<div class="offer-view-details-column">
   <div class="offer-view-details-row">
      <span>Українська цегла</span>
   </div>
   <div class="offer-view-details-row">                            <span>Роздільне</span>
   </div>
   <div class="offer-view-details-row">
      <span>Хороший стан</span>
   </div>
</div>
<div class="offer-view-details-column-aside">
   <div class="offer-view-details-row">
      </svg>
      <span>вчора</span>
   </div>
   <div class="offer-view-details-row">
      </svg>
      <span>1 тиж. тому</span>
   </div>
   <div class="offer-view-details-row">
      </svg>
      <span>
      128                            (сьогодні 1,
      вчора 26)                        </span>
   </div>
</div>
</div>"###;

        let mut pairs =
            ApartmentParser::parse(Rule::characteristics, content).expect("Unsuccessful parsing.");
        assert_eq!(1, pairs.clone().count());
        let mut characteristics = pairs.next().unwrap().into_inner();
        assert_eq!(6, characteristics.clone().count());

        let room_count = characteristics.next().unwrap();
        assert_eq!(Rule::room_count, room_count.as_rule());
        assert_eq!("1", room_count.into_inner().next().unwrap().as_str());

        let area = characteristics.next().unwrap();
        assert_eq!(Rule::area, area.as_rule());
        let mut area_pairs = area.into_inner();
        assert_eq!(3, area_pairs.clone().count());
        assert_eq!("32", area_pairs.next().unwrap().as_str());
        assert_eq!("15", area_pairs.next().unwrap().as_str());
        assert_eq!("5", area_pairs.next().unwrap().as_str());

        let floor_info = characteristics.next().unwrap();
        assert_eq!(Rule::floor_info, floor_info.as_rule());
        let mut floor_info_pairs = floor_info.into_inner();
        assert_eq!(2, floor_info_pairs.clone().count());
        assert_eq!("3", floor_info_pairs.next().unwrap().as_str());
        assert_eq!("9", floor_info_pairs.next().unwrap().as_str());

        let renewed = characteristics.next().unwrap();
        assert_eq!(Rule::renewed, renewed.as_rule());
        let renewed_value = renewed.into_inner().next().unwrap();
        assert_eq!(Rule::event_date, renewed_value.as_rule());
        assert_eq!("вчора", renewed_value.as_str());

        let published = characteristics.next().unwrap();
        assert_eq!(Rule::published, published.as_rule());
        let published_value = published.into_inner().next().unwrap();
        assert_eq!(Rule::event_date, published_value.as_rule());
        assert_eq!("1 тиж. тому", published_value.as_str());

        let views = characteristics.next().unwrap();
        assert_eq!(Rule::views, views.as_rule());
        let mut views_pairs = views.into_inner();
        assert_eq!("128", views_pairs.next().unwrap().as_str());
        assert_eq!("1", views_pairs.next().unwrap().as_str());
        assert_eq!("26", views_pairs.next().unwrap().as_str());
    }

    #[test]
    fn test_description() {
        let content = r#"<div class="offer-view-section-title">Опис</div>
                    <div class="offer-view-section-text">
                    Сдам в оренду 1-но кімнатну квартиру для орендарів без тварин. Всі необхідні меблі є , новий холодильник і пральна машинка. Квартира чиста і охайна. Є відеоогляд квартири.
                    </div>"#;

        let mut pairs =
            ApartmentParser::parse(Rule::description, content).expect("Unsuccessful parsing.");
        assert_eq!(1, pairs.clone().count());
        let description_text = pairs.next().unwrap();
        assert_eq!(Rule::description_text, description_text.as_rule());
        assert!(description_text
            .as_str()
            .trim()
            .starts_with("Сдам в оренду 1-но кімнатну квартиру"));
        assert!(description_text
            .as_str()
            .trim()
            .ends_with("Є відеоогляд квартири."));
    }

    #[test]
    fn test_details_description() {
        let content = r#"<div class="offer-view-section-title">Деталі</div>
                    <div class="offer-view-section-text">
                    Будинок - Українська цегла, в квартирі 1 кімната. Планування кімнат Роздільне. Загальний стан квартири - Хороший стан. Комісія за послуги 50 %. Торг доречний
                    </div>"#;

        let mut pairs = ApartmentParser::parse(Rule::details_description, content)
            .expect("Unsuccessful parsing.");
        assert_eq!(1, pairs.clone().count());
        let mut det_descr_text = pairs
            .next()
            .unwrap()
            .into_inner()
            .next()
            .unwrap()
            .into_inner();
        assert_eq!(4, det_descr_text.clone().count());

        let house_value = det_descr_text.next().unwrap();
        assert_eq!(Rule::house_value, house_value.as_rule());
        assert_eq!("Українська цегла", house_value.as_str());

        let planning_value = det_descr_text.next().unwrap();
        assert_eq!(Rule::planning_value, planning_value.as_rule());
        assert_eq!("Роздільне", planning_value.as_str());

        let state_value = det_descr_text.next().unwrap();
        assert_eq!(Rule::state_value, state_value.as_rule());
        assert_eq!("Хороший стан", state_value.as_str());

        let bargain = det_descr_text.next().unwrap();
        assert_eq!(Rule::bargain, bargain.as_rule());
        assert_eq!("Торг доречний", bargain.as_str());
    }

    #[test]
    fn test_rieltor() {
        let content = r#"<div class="offer-view-rieltor-header-info">
              <a href="https://0501112233.rieltor.ua/" class="offer-view-rieltor-name" rel="">
          Пес Патрон        </a>
            <div class="offer-view-rieltor-position">
        Рієлтор      </div>
        <a href="" class="offer-view-rieltor-agency-link">
            Flower-Group          </a>"#;

        let mut pairs =
            ApartmentParser::parse(Rule::rieltor, content).expect("Unsuccessful parsing.");
        assert_eq!(1, pairs.clone().count());
        let mut rieltor = pairs.next().unwrap().into_inner();
        let phone_number = rieltor.next().unwrap();
        assert_eq!(Rule::phone_number, phone_number.as_rule());
        assert_eq!("0501112233", phone_number.as_str());
        let rieltor_name = rieltor.next().unwrap();
        assert_eq!(Rule::rieltor_name, rieltor_name.as_rule());
        assert_eq!(
            "Пес Патрон",
            rieltor_name.into_inner().next().unwrap().as_str()
        );
        let rieltor_position = rieltor.next().unwrap();
        assert_eq!(Rule::rieltor_position, rieltor_position.as_rule());
        assert_eq!(
            "Рієлтор",
            rieltor_position.into_inner().next().unwrap().as_str()
        );
        let rieltor_agency = rieltor.next().unwrap();
        assert_eq!(Rule::rieltor_agency, rieltor_agency.as_rule());
        assert_eq!(
            "Flower-Group",
            rieltor_agency.into_inner().next().unwrap().as_str()
        );
    }

    #[test]
    fn test_photo_list() {
        let content = r#"<img class="offer-photo-gallery__image" src="https://img.lunstatic.net/rieltor-offer-1600x1200/offers/x.jpeg" alt="1814416057572659" loading="lazy">
                                            <img class="offer-photo-gallery__image" src="https://img.lunstatic.net/rieltor-offer-1600x1200/offers/x.jpeg" alt="1814416057564261" loading="lazy">
                                            <img class="offer-photo-gallery__image" src="https://img.lunstatic.net/rieltor-offer-1600x1200/offers/x.jpeg" alt="1814416057573214" loading="lazy">
                                            <img class="offer-photo-gallery__image" src="https://img.lunstatic.net/rieltor-offer-1600x1200/offers/x.jpeg" alt="1814416057564281" loading="lazy">
                                            <img class="offer-photo-gallery__image" src="https://img.lunstatic.net/rieltor-offer-1600x1200/offers/x.jpeg" alt="1814416057573294" loading="lazy">
                                            <img class="offer-photo-gallery__image" src="https://img.lunstatic.net/rieltor-offer-1600x1200/offers/x.jpeg" alt="1814416060375345" loading="lazy">
                                            <img class="offer-photo-gallery__image" src="https://img.lunstatic.net/rieltor-offer-1600x1200/offers/x.jpeg" alt="1814416060010137" loading="lazy">
                                            <img class="offer-photo-gallery__image" src="https://img.lunstatic.net/rieltor-offer-1600x1200/offers/x.jpeg" alt="1814416060239321" loading="lazy">
                                            <img class="offer-photo-gallery__image" src="https://img.lunstatic.net/rieltor-offer-1600x1200/offers/x.jpeg" alt="1814416059797714" loading="lazy">
                                            <img class="offer-photo-gallery__image" src="https://img.lunstatic.net/rieltor-offer-1600x1200/offers/x.jpeg" alt="1814416060350043" loading="lazy">"#;

        let mut pairs =
            ApartmentParser::parse(Rule::photo_list, content).expect("Unsuccessful parsing.");
        assert_eq!(1, pairs.clone().count());
        let photo_list = pairs.next().unwrap().into_inner();
        assert_eq!(10, photo_list.clone().len());
        for photo in photo_list {
            assert_eq!(photo.as_rule(), Rule::attr_value);
            let photo_string = photo.as_str();
            assert!(photo_string.starts_with("https://img.lunstatic.net/"));
            assert!(photo_string.ends_with("x.jpeg"));
        }
    }

    #[test]
    fn test_apartment_links() {
        let test1 = "https://rieltor.ua/harkov/flats-rent/view/11569123/";
        let pairs1 =
            ApartmentParser::parse(Rule::apartment_link, test1).expect("Unsuccessful parsing.");
        assert_eq!(1, pairs1.clone().count());
        assert_eq!(
            Rule::apartment_link,
            pairs1.clone().next().unwrap().as_rule()
        );

        let test2 = "https://rieltor.ua/flats-rent/view/11569123/";
        let pairs2 =
            ApartmentParser::parse(Rule::apartment_link, test2).expect("Unsuccessful parsing.");
        assert_eq!(1, pairs2.clone().count());
        assert_eq!(
            Rule::apartment_link,
            pairs2.clone().next().unwrap().as_rule()
        );

        let test3 = r"https://rieltor.ua/harkov/flats-rent/3-rooms/?price_max=6250&sort=-default";
        let pairs3 = ApartmentParser::parse(Rule::apartment_list_link, test3)
            .expect("Unsuccessful parsing.");
        assert_eq!(1, pairs3.clone().count());
        assert_eq!(
            Rule::apartment_list_link,
            pairs3.clone().next().unwrap().as_rule()
        );

        let test4 = r"https://rieltor.ua/flats-rent";
        let pairs4 = ApartmentParser::parse(Rule::apartment_list_link, test4)
            .expect("Unsuccessful parsing.");
        assert_eq!(1, pairs4.clone().count());
        assert_eq!(
            Rule::apartment_list_link,
            pairs4.clone().next().unwrap().as_rule()
        );
    }

    #[test]
    fn test_event_date() {
        let test1 = "вчора";
        let test2 = "6 днів тому";
        let test3 = "1 тиж. тому";
        let test4 = "3 міс. тому";
        let test5 = "2 р. 5 міс. тому";
        let test6 = "1 р. 3 міс. тому";

        ApartmentParser::parse(Rule::event_date, test1).expect("Unsuccessful parsing.");
        ApartmentParser::parse(Rule::event_date, test2).expect("Unsuccessful parsing.");
        ApartmentParser::parse(Rule::event_date, test3).expect("Unsuccessful parsing.");
        ApartmentParser::parse(Rule::event_date, test4).expect("Unsuccessful parsing.");
        ApartmentParser::parse(Rule::event_date, test5).expect("Unsuccessful parsing.");
        ApartmentParser::parse(Rule::event_date, test6).expect("Unsuccessful parsing.");
    }
}
