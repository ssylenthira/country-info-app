use reqwest::Error;
use serde::Deserialize;
use std::io;

// Define the structure to deserialize the API response.
#[derive(Deserialize, Debug, Clone)]
struct Country {
    name: Name,
    capital: Option<Vec<String>>,
    population: Option<u64>,
    region: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
struct Name {
    common: String,
    official: String,
}

// Function to fetch country data from the API.
async fn fetch_countries() -> Result<Vec<Country>, Error> {
    let url = "https://restcountries.com/v3.1/all";
    let response = reqwest::get(url).await?.json::<Vec<Country>>().await?;
    Ok(response)
}

// Function to display country information.
fn display_country_info(countries: &[Country]) {
    for country in countries {
        println!(
            "Common Name: {}\nOfficial Name: {}\nCapital: {}\nPopulation: {}\nRegion: {}\n",
            country.name.common,
            country.name.official,
            country.capital.as_ref().map_or("N/A".to_string(), |c| c.join(", ")),
            country.population.unwrap_or(0),
            country.region.as_ref().unwrap_or(&"N/A".to_string())
        );
        println!("----------------------------------");
    }
}

// Function to search countries by their common name.
fn search_country_by_name(countries: &[Country], search_term: &str) {
    let search_term = search_term.trim(); // Trim input
    let filtered_countries: Vec<&Country> = countries
        .iter()
        .filter(|c| c.name.common.to_lowercase().contains(&search_term.to_lowercase()))
        .collect();

    if filtered_countries.is_empty() {
        println!("No country found with the name: {}", search_term);
    } else {
        // Convert Vec<&Country> to Vec<Country> by cloning the filtered results
        let filtered_cloned: Vec<Country> = filtered_countries.into_iter().cloned().collect();
        display_country_info(&filtered_cloned);
    }
}

// Function to sort countries by a specified field.
fn sort_countries(countries: &mut [Country], field: &str) {
    match field {
        "name" => countries.sort_by(|a, b| a.name.common.cmp(&b.name.common)),
        "population" => countries.sort_by(|a, b| a.population.unwrap_or(0).cmp(&b.population.unwrap_or(0))),
        "region" => countries.sort_by(|a, b| a.region.as_ref().unwrap_or(&"".to_string()).cmp(b.region.as_ref().unwrap_or(&"".to_string()))),
        _ => println!("Invalid field! Choose from: name, population, region."),
    }
}

// Main function that runs the application.
#[tokio::main]
async fn main() {
    // Fetch data from the API.
    let countries = match fetch_countries().await {
        Ok(countries) => countries,
        Err(e) => {
            println!("Error fetching countries: {}", e);
            return;
        }
    };

    // Initial display of countries.
    println!("--- List of Countries ---");
    display_country_info(&countries);

    loop {
        println!("Options: \n1. Search Country \n2. Sort Countries \n3. Exit");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim() {
            "1" => {
                // Search for a country by name.
                println!("Enter the country name to search: ");
                let mut search_name = String::new();
                io::stdin().read_line(&mut search_name).expect("Failed to read input");
                search_country_by_name(&countries, search_name.trim());
            }
            "2" => {
                // Sort countries by a specified field.
                println!("Enter the field to sort by (name, population, region): ");
                let mut sort_field = String::new();
                io::stdin().read_line(&mut sort_field).expect("Failed to read input");

                if sort_field.trim().is_empty() {
                    println!("Invalid input, please enter a valid field.");
                    continue;
                }

                let mut countries_sorted = countries.clone();
                sort_countries(&mut countries_sorted, sort_field.trim());
                display_country_info(&countries_sorted);
            }
            "3" => {
                // Exit the loop.
                break;
            }
            _ => println!("Invalid option, please choose again."),
        }
    }
}
