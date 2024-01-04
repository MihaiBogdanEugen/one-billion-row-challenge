use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use clap::Parser;
use rand::seq::SliceRandom;
use rand_distr::Distribution;
use rand_distr::Normal;

const STD_DEV: f64 = 10.0;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    size: usize,
}

struct WeatherStation {
    id: &'static str,
    distr: Normal<f64>,
}

impl WeatherStation {
    fn new(id: &'static str, mean: f64) -> WeatherStation {
        WeatherStation {
            id,
            distr: Normal::new(mean, STD_DEV).unwrap(),
        }
    }

    fn get_measurement(&self) -> f64 {
        format!("{:.1$}", self.distr.sample(&mut rand::thread_rng()), 1)
            .parse::<f64>()
            .unwrap()
    }
}

fn get_weather_stations() -> Vec<WeatherStation> {
    let mut result: Vec<WeatherStation> = Vec::new();
    result.push(WeatherStation::new("Abha", 18.0));
    result.push(WeatherStation::new("Abidjan", 26.0));
    result.push(WeatherStation::new("Abéché", 29.4));
    result.push(WeatherStation::new("Accra", 26.4));
    result.push(WeatherStation::new("Addis Ababa", 16.0));
    result.push(WeatherStation::new("Adelaide", 17.3));
    result.push(WeatherStation::new("Aden", 29.1));
    result.push(WeatherStation::new("Ahvaz", 25.4));
    result.push(WeatherStation::new("Albuquerque", 14.0));
    result.push(WeatherStation::new("Alexandra", 11.0));
    result.push(WeatherStation::new("Alexandria", 20.0));
    result.push(WeatherStation::new("Algiers", 18.2));
    result.push(WeatherStation::new("Alice Springs", 21.0));
    result.push(WeatherStation::new("Almaty", 10.0));
    result.push(WeatherStation::new("Amsterdam", 10.2));
    result.push(WeatherStation::new("Anadyr", -6.9));
    result.push(WeatherStation::new("Anchorage", 2.8));
    result.push(WeatherStation::new("Andorra la Vella", 9.8));
    result.push(WeatherStation::new("Ankara", 12.0));
    result.push(WeatherStation::new("Antananarivo", 17.9));
    result.push(WeatherStation::new("Antsiranana", 25.2));
    result.push(WeatherStation::new("Arkhangelsk", 1.3));
    result.push(WeatherStation::new("Ashgabat", 17.1));
    result.push(WeatherStation::new("Asmara", 15.6));
    result.push(WeatherStation::new("Assab", 30.5));
    result.push(WeatherStation::new("Astana", 3.5));
    result.push(WeatherStation::new("Athens", 19.2));
    result.push(WeatherStation::new("Atlanta", 17.0));
    result.push(WeatherStation::new("Auckland", 15.2));
    result.push(WeatherStation::new("Austin", 20.7));
    result.push(WeatherStation::new("Baghdad", 22.77));
    result.push(WeatherStation::new("Baguio", 19.5));
    result.push(WeatherStation::new("Baku", 15.1));
    result.push(WeatherStation::new("Baltimore", 13.1));
    result.push(WeatherStation::new("Bamako", 27.8));
    result.push(WeatherStation::new("Bangkok", 28.6));
    result.push(WeatherStation::new("Bangui", 26.0));
    result.push(WeatherStation::new("Banjul", 26.0));
    result.push(WeatherStation::new("Barcelona", 18.2));
    result.push(WeatherStation::new("Bata", 25.1));
    result.push(WeatherStation::new("Batumi", 14.0));
    result.push(WeatherStation::new("Beijing", 12.9));
    result.push(WeatherStation::new("Beirut", 20.9));
    result.push(WeatherStation::new("Belgrade", 12.5));
    result.push(WeatherStation::new("Belize City", 26.7));
    result.push(WeatherStation::new("Benghazi", 19.9));
    result.push(WeatherStation::new("Bergen", 7.7));
    result.push(WeatherStation::new("Berlin", 10.3));
    result.push(WeatherStation::new("Bilbao", 14.7));
    result.push(WeatherStation::new("Birao", 26.5));
    result.push(WeatherStation::new("Bishkek", 11.3));
    result.push(WeatherStation::new("Bissau", 27.0));
    result.push(WeatherStation::new("Blantyre", 22.2));
    result.push(WeatherStation::new("Bloemfontein", 15.6));
    result.push(WeatherStation::new("Boise", 11.4));
    result.push(WeatherStation::new("Bordeaux", 14.2));
    result.push(WeatherStation::new("Bosaso", 30.0));
    result.push(WeatherStation::new("Boston", 10.9));
    result.push(WeatherStation::new("Bouaké", 26.0));
    result.push(WeatherStation::new("Bratislava", 10.5));
    result.push(WeatherStation::new("Brazzaville", 25.0));
    result.push(WeatherStation::new("Bridgetown", 27.0));
    result.push(WeatherStation::new("Brisbane", 21.4));
    result.push(WeatherStation::new("Brussels", 10.5));
    result.push(WeatherStation::new("Bucharest", 10.8));
    result.push(WeatherStation::new("Budapest", 11.3));
    result.push(WeatherStation::new("Bujumbura", 23.8));
    result.push(WeatherStation::new("Bulawayo", 18.9));
    result.push(WeatherStation::new("Burnie", 13.1));
    result.push(WeatherStation::new("Busan", 15.0));
    result.push(WeatherStation::new("Cabo San Lucas", 23.9));
    result.push(WeatherStation::new("Cairns", 25.0));
    result.push(WeatherStation::new("Cairo", 21.4));
    result.push(WeatherStation::new("Calgary", 4.4));
    result.push(WeatherStation::new("Canberra", 13.1));
    result.push(WeatherStation::new("Cape Town", 16.2));
    result.push(WeatherStation::new("Changsha", 17.4));
    result.push(WeatherStation::new("Charlotte", 16.1));
    result.push(WeatherStation::new("Chiang Mai", 25.8));
    result.push(WeatherStation::new("Chicago", 9.8));
    result.push(WeatherStation::new("Chihuahua", 18.6));
    result.push(WeatherStation::new("Chișinău", 10.2));
    result.push(WeatherStation::new("Chittagong", 25.9));
    result.push(WeatherStation::new("Chongqing", 18.6));
    result.push(WeatherStation::new("Christchurch", 12.2));
    result.push(WeatherStation::new("City of San Marino", 11.8));
    result.push(WeatherStation::new("Colombo", 27.4));
    result.push(WeatherStation::new("Columbus", 11.7));
    result.push(WeatherStation::new("Conakry", 26.4));
    result.push(WeatherStation::new("Copenhagen", 9.1));
    result.push(WeatherStation::new("Cotonou", 27.2));
    result.push(WeatherStation::new("Cracow", 9.3));
    result.push(WeatherStation::new("Da Lat", 17.9));
    result.push(WeatherStation::new("Da Nang", 25.8));
    result.push(WeatherStation::new("Dakar", 24.0));
    result.push(WeatherStation::new("Dallas", 19.0));
    result.push(WeatherStation::new("Damascus", 17.0));
    result.push(WeatherStation::new("Dampier", 26.4));
    result.push(WeatherStation::new("Dar es Salaam", 25.8));
    result.push(WeatherStation::new("Darwin", 27.6));
    result.push(WeatherStation::new("Denpasar", 23.7));
    result.push(WeatherStation::new("Denver", 10.4));
    result.push(WeatherStation::new("Detroit", 10.0));
    result.push(WeatherStation::new("Dhaka", 25.9));
    result.push(WeatherStation::new("Dikson", -11.1));
    result.push(WeatherStation::new("Dili", 26.6));
    result.push(WeatherStation::new("Djibouti", 29.9));
    result.push(WeatherStation::new("Dodoma", 22.7));
    result.push(WeatherStation::new("Dolisie", 24.0));
    result.push(WeatherStation::new("Douala", 26.7));
    result.push(WeatherStation::new("Dubai", 26.9));
    result.push(WeatherStation::new("Dublin", 9.8));
    result.push(WeatherStation::new("Dunedin", 11.1));
    result.push(WeatherStation::new("Durban", 20.6));
    result.push(WeatherStation::new("Dushanbe", 14.7));
    result.push(WeatherStation::new("Edinburgh", 9.3));
    result.push(WeatherStation::new("Edmonton", 4.2));
    result.push(WeatherStation::new("El Paso", 18.1));
    result.push(WeatherStation::new("Entebbe", 21.0));
    result.push(WeatherStation::new("Erbil", 19.5));
    result.push(WeatherStation::new("Erzurum", 5.1));
    result.push(WeatherStation::new("Fairbanks", -2.3));
    result.push(WeatherStation::new("Fianarantsoa", 17.9));
    result.push(WeatherStation::new("Flores,  Petén", 26.4));
    result.push(WeatherStation::new("Frankfurt", 10.6));
    result.push(WeatherStation::new("Fresno", 17.9));
    result.push(WeatherStation::new("Fukuoka", 17.0));
    result.push(WeatherStation::new("Gabès", 19.5));
    result.push(WeatherStation::new("Gaborone", 21.0));
    result.push(WeatherStation::new("Gagnoa", 26.0));
    result.push(WeatherStation::new("Gangtok", 15.2));
    result.push(WeatherStation::new("Garissa", 29.3));
    result.push(WeatherStation::new("Garoua", 28.3));
    result.push(WeatherStation::new("George Town", 27.9));
    result.push(WeatherStation::new("Ghanzi", 21.4));
    result.push(WeatherStation::new("Gjoa Haven", -14.4));
    result.push(WeatherStation::new("Guadalajara", 20.9));
    result.push(WeatherStation::new("Guangzhou", 22.4));
    result.push(WeatherStation::new("Guatemala City", 20.4));
    result.push(WeatherStation::new("Halifax", 7.5));
    result.push(WeatherStation::new("Hamburg", 9.7));
    result.push(WeatherStation::new("Hamilton", 13.8));
    result.push(WeatherStation::new("Hanga Roa", 20.5));
    result.push(WeatherStation::new("Hanoi", 23.6));
    result.push(WeatherStation::new("Harare", 18.4));
    result.push(WeatherStation::new("Harbin", 5.0));
    result.push(WeatherStation::new("Hargeisa", 21.7));
    result.push(WeatherStation::new("Hat Yai", 27.0));
    result.push(WeatherStation::new("Havana", 25.2));
    result.push(WeatherStation::new("Helsinki", 5.9));
    result.push(WeatherStation::new("Heraklion", 18.9));
    result.push(WeatherStation::new("Hiroshima", 16.3));
    result.push(WeatherStation::new("Ho Chi Minh City", 27.4));
    result.push(WeatherStation::new("Hobart", 12.7));
    result.push(WeatherStation::new("Hong Kong", 23.3));
    result.push(WeatherStation::new("Honiara", 26.5));
    result.push(WeatherStation::new("Honolulu", 25.4));
    result.push(WeatherStation::new("Houston", 20.8));
    result.push(WeatherStation::new("Ifrane", 11.4));
    result.push(WeatherStation::new("Indianapolis", 11.8));
    result.push(WeatherStation::new("Iqaluit", -9.3));
    result.push(WeatherStation::new("Irkutsk", 1.0));
    result.push(WeatherStation::new("Istanbul", 13.9));
    result.push(WeatherStation::new("İzmir", 17.9));
    result.push(WeatherStation::new("Jacksonville", 20.3));
    result.push(WeatherStation::new("Jakarta", 26.7));
    result.push(WeatherStation::new("Jayapura", 27.0));
    result.push(WeatherStation::new("Jerusalem", 18.3));
    result.push(WeatherStation::new("Johannesburg", 15.5));
    result.push(WeatherStation::new("Jos", 22.8));
    result.push(WeatherStation::new("Juba", 27.8));
    result.push(WeatherStation::new("Kabul", 12.1));
    result.push(WeatherStation::new("Kampala", 20.0));
    result.push(WeatherStation::new("Kandi", 27.7));
    result.push(WeatherStation::new("Kankan", 26.5));
    result.push(WeatherStation::new("Kano", 26.4));
    result.push(WeatherStation::new("Kansas City", 12.5));
    result.push(WeatherStation::new("Karachi", 26.0));
    result.push(WeatherStation::new("Karonga", 24.4));
    result.push(WeatherStation::new("Kathmandu", 18.3));
    result.push(WeatherStation::new("Khartoum", 29.9));
    result.push(WeatherStation::new("Kingston", 27.4));
    result.push(WeatherStation::new("Kinshasa", 25.3));
    result.push(WeatherStation::new("Kolkata", 26.7));
    result.push(WeatherStation::new("Kuala Lumpur", 27.3));
    result.push(WeatherStation::new("Kumasi", 26.0));
    result.push(WeatherStation::new("Kunming", 15.7));
    result.push(WeatherStation::new("Kuopio", 3.4));
    result.push(WeatherStation::new("Kuwait City", 25.7));
    result.push(WeatherStation::new("Kyiv", 8.4));
    result.push(WeatherStation::new("Kyoto", 15.8));
    result.push(WeatherStation::new("La Ceiba", 26.2));
    result.push(WeatherStation::new("La Paz", 23.7));
    result.push(WeatherStation::new("Lagos", 26.8));
    result.push(WeatherStation::new("Lahore", 24.3));
    result.push(WeatherStation::new("Lake Havasu City", 23.7));
    result.push(WeatherStation::new("Lake Tekapo", 8.7));
    result.push(WeatherStation::new("Las Palmas de Gran Canaria", 21.2));
    result.push(WeatherStation::new("Las Vegas", 20.3));
    result.push(WeatherStation::new("Launceston", 13.1));
    result.push(WeatherStation::new("Lhasa", 7.6));
    result.push(WeatherStation::new("Libreville", 25.9));
    result.push(WeatherStation::new("Lisbon", 17.5));
    result.push(WeatherStation::new("Livingstone", 21.8));
    result.push(WeatherStation::new("Ljubljana", 10.9));
    result.push(WeatherStation::new("Lodwar", 29.3));
    result.push(WeatherStation::new("Lomé", 26.9));
    result.push(WeatherStation::new("London", 11.3));
    result.push(WeatherStation::new("Los Angeles", 18.6));
    result.push(WeatherStation::new("Louisville", 13.9));
    result.push(WeatherStation::new("Luanda", 25.8));
    result.push(WeatherStation::new("Lubumbashi", 20.8));
    result.push(WeatherStation::new("Lusaka", 19.9));
    result.push(WeatherStation::new("Luxembourg City", 9.3));
    result.push(WeatherStation::new("Lviv", 7.8));
    result.push(WeatherStation::new("Lyon", 12.5));
    result.push(WeatherStation::new("Madrid", 15.0));
    result.push(WeatherStation::new("Mahajanga", 26.3));
    result.push(WeatherStation::new("Makassar", 26.7));
    result.push(WeatherStation::new("Makurdi", 26.0));
    result.push(WeatherStation::new("Malabo", 26.3));
    result.push(WeatherStation::new("Malé", 28.0));
    result.push(WeatherStation::new("Managua", 27.3));
    result.push(WeatherStation::new("Manama", 26.5));
    result.push(WeatherStation::new("Mandalay", 28.0));
    result.push(WeatherStation::new("Mango", 28.1));
    result.push(WeatherStation::new("Manila", 28.4));
    result.push(WeatherStation::new("Maputo", 22.8));
    result.push(WeatherStation::new("Marrakesh", 19.6));
    result.push(WeatherStation::new("Marseille", 15.8));
    result.push(WeatherStation::new("Maun", 22.4));
    result.push(WeatherStation::new("Medan", 26.5));
    result.push(WeatherStation::new("Mek'ele", 22.7));
    result.push(WeatherStation::new("Melbourne", 15.1));
    result.push(WeatherStation::new("Memphis", 17.2));
    result.push(WeatherStation::new("Mexicali", 23.1));
    result.push(WeatherStation::new("Mexico City", 17.5));
    result.push(WeatherStation::new("Miami", 24.9));
    result.push(WeatherStation::new("Milan", 13.0));
    result.push(WeatherStation::new("Milwaukee", 8.9));
    result.push(WeatherStation::new("Minneapolis", 7.8));
    result.push(WeatherStation::new("Minsk", 6.7));
    result.push(WeatherStation::new("Mogadishu", 27.1));
    result.push(WeatherStation::new("Mombasa", 26.3));
    result.push(WeatherStation::new("Monaco", 16.4));
    result.push(WeatherStation::new("Moncton", 6.1));
    result.push(WeatherStation::new("Monterrey", 22.3));
    result.push(WeatherStation::new("Montreal", 6.8));
    result.push(WeatherStation::new("Moscow", 5.8));
    result.push(WeatherStation::new("Mumbai", 27.1));
    result.push(WeatherStation::new("Murmansk", 0.6));
    result.push(WeatherStation::new("Muscat", 28.0));
    result.push(WeatherStation::new("Mzuzu", 17.7));
    result.push(WeatherStation::new("N'Djamena", 28.3));
    result.push(WeatherStation::new("Naha", 23.1));
    result.push(WeatherStation::new("Nairobi", 17.8));
    result.push(WeatherStation::new("Nakhon Ratchasima", 27.3));
    result.push(WeatherStation::new("Napier", 14.6));
    result.push(WeatherStation::new("Napoli", 15.9));
    result.push(WeatherStation::new("Nashville", 15.4));
    result.push(WeatherStation::new("Nassau", 24.6));
    result.push(WeatherStation::new("Ndola", 20.3));
    result.push(WeatherStation::new("New Delhi", 25.0));
    result.push(WeatherStation::new("New Orleans", 20.7));
    result.push(WeatherStation::new("New York City", 12.9));
    result.push(WeatherStation::new("Ngaoundéré", 22.0));
    result.push(WeatherStation::new("Niamey", 29.3));
    result.push(WeatherStation::new("Nicosia", 19.7));
    result.push(WeatherStation::new("Niigata", 13.9));
    result.push(WeatherStation::new("Nouadhibou", 21.3));
    result.push(WeatherStation::new("Nouakchott", 25.7));
    result.push(WeatherStation::new("Novosibirsk", 1.7));
    result.push(WeatherStation::new("Nuuk", -1.4));
    result.push(WeatherStation::new("Odesa", 10.7));
    result.push(WeatherStation::new("Odienné", 26.0));
    result.push(WeatherStation::new("Oklahoma City", 15.9));
    result.push(WeatherStation::new("Omaha", 10.6));
    result.push(WeatherStation::new("Oranjestad", 28.1));
    result.push(WeatherStation::new("Oslo", 5.7));
    result.push(WeatherStation::new("Ottawa", 6.6));
    result.push(WeatherStation::new("Ouagadougou", 28.3));
    result.push(WeatherStation::new("Ouahigouya", 28.6));
    result.push(WeatherStation::new("Ouarzazate", 18.9));
    result.push(WeatherStation::new("Oulu", 2.7));
    result.push(WeatherStation::new("Palembang", 27.3));
    result.push(WeatherStation::new("Palermo", 18.5));
    result.push(WeatherStation::new("Palm Springs", 24.5));
    result.push(WeatherStation::new("Palmerston North", 13.2));
    result.push(WeatherStation::new("Panama City", 28.0));
    result.push(WeatherStation::new("Parakou", 26.8));
    result.push(WeatherStation::new("Paris", 12.3));
    result.push(WeatherStation::new("Perth", 18.7));
    result.push(WeatherStation::new("Petropavlovsk-Kamchatsky", 1.9));
    result.push(WeatherStation::new("Philadelphia", 13.2));
    result.push(WeatherStation::new("Phnom Penh", 28.3));
    result.push(WeatherStation::new("Phoenix", 23.9));
    result.push(WeatherStation::new("Pittsburgh", 10.8));
    result.push(WeatherStation::new("Podgorica", 15.3));
    result.push(WeatherStation::new("Pointe-Noire", 26.1));
    result.push(WeatherStation::new("Pontianak", 27.7));
    result.push(WeatherStation::new("Port Moresby", 26.9));
    result.push(WeatherStation::new("Port Sudan", 28.4));
    result.push(WeatherStation::new("Port Vila", 24.3));
    result.push(WeatherStation::new("Port-Gentil", 26.0));
    result.push(WeatherStation::new("Portland (OR)", 12.4));
    result.push(WeatherStation::new("Porto", 15.7));
    result.push(WeatherStation::new("Prague", 8.4));
    result.push(WeatherStation::new("Praia", 24.4));
    result.push(WeatherStation::new("Pretoria", 18.2));
    result.push(WeatherStation::new("Pyongyang", 10.8));
    result.push(WeatherStation::new("Rabat", 17.2));
    result.push(WeatherStation::new("Rangpur", 24.4));
    result.push(WeatherStation::new("Reggane", 28.3));
    result.push(WeatherStation::new("Reykjavík", 4.3));
    result.push(WeatherStation::new("Riga", 6.2));
    result.push(WeatherStation::new("Riyadh", 26.0));
    result.push(WeatherStation::new("Rome", 15.2));
    result.push(WeatherStation::new("Roseau", 26.2));
    result.push(WeatherStation::new("Rostov-on-Don", 9.9));
    result.push(WeatherStation::new("Sacramento", 16.3));
    result.push(WeatherStation::new("Saint Petersburg", 5.8));
    result.push(WeatherStation::new("Saint-Pierre", 5.7));
    result.push(WeatherStation::new("Salt Lake City", 11.6));
    result.push(WeatherStation::new("San Antonio", 20.8));
    result.push(WeatherStation::new("San Diego", 17.8));
    result.push(WeatherStation::new("San Francisco", 14.6));
    result.push(WeatherStation::new("San Jose", 16.4));
    result.push(WeatherStation::new("San José", 22.6));
    result.push(WeatherStation::new("San Juan", 27.2));
    result.push(WeatherStation::new("San Salvador", 23.1));
    result.push(WeatherStation::new("Sana'a", 20.0));
    result.push(WeatherStation::new("Santo Domingo", 25.9));
    result.push(WeatherStation::new("Sapporo", 8.9));
    result.push(WeatherStation::new("Sarajevo", 10.1));
    result.push(WeatherStation::new("Saskatoon", 3.3));
    result.push(WeatherStation::new("Seattle", 11.3));
    result.push(WeatherStation::new("Ségou", 28.0));
    result.push(WeatherStation::new("Seoul", 12.5));
    result.push(WeatherStation::new("Seville", 19.2));
    result.push(WeatherStation::new("Shanghai", 16.7));
    result.push(WeatherStation::new("Singapore", 27.0));
    result.push(WeatherStation::new("Skopje", 12.4));
    result.push(WeatherStation::new("Sochi", 14.2));
    result.push(WeatherStation::new("Sofia", 10.6));
    result.push(WeatherStation::new("Sokoto", 28.0));
    result.push(WeatherStation::new("Split", 16.1));
    result.push(WeatherStation::new("St. John's", 5.0));
    result.push(WeatherStation::new("St. Louis", 13.9));
    result.push(WeatherStation::new("Stockholm", 6.6));
    result.push(WeatherStation::new("Surabaya", 27.1));
    result.push(WeatherStation::new("Suva", 25.6));
    result.push(WeatherStation::new("Suwałki", 7.2));
    result.push(WeatherStation::new("Sydney", 17.7));
    result.push(WeatherStation::new("Tabora", 23.0));
    result.push(WeatherStation::new("Tabriz", 12.6));
    result.push(WeatherStation::new("Taipei", 23.0));
    result.push(WeatherStation::new("Tallinn", 6.4));
    result.push(WeatherStation::new("Tamale", 27.9));
    result.push(WeatherStation::new("Tamanrasset", 21.7));
    result.push(WeatherStation::new("Tampa", 22.9));
    result.push(WeatherStation::new("Tashkent", 14.8));
    result.push(WeatherStation::new("Tauranga", 14.8));
    result.push(WeatherStation::new("Tbilisi", 12.9));
    result.push(WeatherStation::new("Tegucigalpa", 21.7));
    result.push(WeatherStation::new("Tehran", 17.0));
    result.push(WeatherStation::new("Tel Aviv", 20.0));
    result.push(WeatherStation::new("Thessaloniki", 16.0));
    result.push(WeatherStation::new("Thiès", 24.0));
    result.push(WeatherStation::new("Tijuana", 17.8));
    result.push(WeatherStation::new("Timbuktu", 28.0));
    result.push(WeatherStation::new("Tirana", 15.2));
    result.push(WeatherStation::new("Toamasina", 23.4));
    result.push(WeatherStation::new("Tokyo", 15.4));
    result.push(WeatherStation::new("Toliara", 24.1));
    result.push(WeatherStation::new("Toluca", 12.4));
    result.push(WeatherStation::new("Toronto", 9.4));
    result.push(WeatherStation::new("Tripoli", 20.0));
    result.push(WeatherStation::new("Tromsø", 2.9));
    result.push(WeatherStation::new("Tucson", 20.9));
    result.push(WeatherStation::new("Tunis", 18.4));
    result.push(WeatherStation::new("Ulaanbaatar", -0.4));
    result.push(WeatherStation::new("Upington", 20.4));
    result.push(WeatherStation::new("Ürümqi", 7.4));
    result.push(WeatherStation::new("Vaduz", 10.1));
    result.push(WeatherStation::new("Valencia", 18.3));
    result.push(WeatherStation::new("Valletta", 18.8));
    result.push(WeatherStation::new("Vancouver", 10.4));
    result.push(WeatherStation::new("Veracruz", 25.4));
    result.push(WeatherStation::new("Vienna", 10.4));
    result.push(WeatherStation::new("Vientiane", 25.9));
    result.push(WeatherStation::new("Villahermosa", 27.1));
    result.push(WeatherStation::new("Vilnius", 6.0));
    result.push(WeatherStation::new("Virginia Beach", 15.8));
    result.push(WeatherStation::new("Vladivostok", 4.9));
    result.push(WeatherStation::new("Warsaw", 8.5));
    result.push(WeatherStation::new("Washington, D.C.", 14.6));
    result.push(WeatherStation::new("Wau", 27.8));
    result.push(WeatherStation::new("Wellington", 12.9));
    result.push(WeatherStation::new("Whitehorse", -0.1));
    result.push(WeatherStation::new("Wichita", 13.9));
    result.push(WeatherStation::new("Willemstad", 28.0));
    result.push(WeatherStation::new("Winnipeg", 3.0));
    result.push(WeatherStation::new("Wrocław", 9.6));
    result.push(WeatherStation::new("Xi'an", 14.1));
    result.push(WeatherStation::new("Yakutsk", -8.8));
    result.push(WeatherStation::new("Yangon", 27.5));
    result.push(WeatherStation::new("Yaoundé", 23.8));
    result.push(WeatherStation::new("Yellowknife", -4.3));
    result.push(WeatherStation::new("Yerevan", 12.4));
    result.push(WeatherStation::new("Yinchuan", 9.0));
    result.push(WeatherStation::new("Zagreb", 10.7));
    result.push(WeatherStation::new("Zanzibar City", 26.0));
    result.push(WeatherStation::new("Zürich", 9.3));
    result
}

fn main() {
    let cli: Cli = Cli::parse();
    let weather_stations: Vec<WeatherStation> = get_weather_stations();
    let file: File = File::create(format!("measurements_{}.txt", cli.size)).unwrap();
    let mut writer: BufWriter<File> = BufWriter::new(file);

    for _ in 0..cli.size {
        let station: &WeatherStation = weather_stations.choose(&mut rand::thread_rng()).unwrap();
        writeln!(writer, "{};{}", station.id, station.get_measurement()).unwrap();
    }
}