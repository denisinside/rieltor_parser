use anyhow::Result;
use clap::{Parser, Subcommand};
use rieltor_parser::*;

#[derive(Parser, Debug)]
#[command(
    author = "Denys Shvachka <d.shvachka@ukma.edu.ua>",
    version = "1.0",
    about = "Rieltor.ua Apartment Parser",
    long_about = "This CLI parses HTML of rieltor.ua apartments and displays their contents in json format.",
    disable_help_flag = true,
    disable_help_subcommand = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Parse {
        source: String,
        output: Option<String>,
    },
    #[command(name = "parse_list")]
    ParseList {
        source: String,
        output: Option<String>,
    },
    Credits,
    Help,
}

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::args().len() == 1 {
        println!("No command provided. Use `help` for more information.");
        std::process::exit(2);
    }

    let cli = Cli::parse();

    match &cli.command {
        Commands::Parse { source, output } => {
            let apartment = parse_apartment(source).await?;
            let output_file = output.clone().unwrap_or_else(|| "".to_string());
            let result = save_to_json(&apartment, &output_file)?;
            println!("Parsed apartment data saved to '{}'.", result);
        }
        Commands::ParseList { source, output } => {
            let apartments = parse_apartment_list(source).await?;
            let output_dir = output.clone().unwrap_or_else(|| "".to_string());
            let result = save_apartments_to_directory(&apartments, &output_dir)?;
            println!("Parsed apartment data saved to '{}'.", result);
        }
        Commands::Credits => {
            println!("Rieltor.ua apartment parser");
            println!("Version: 1.0");
            println!("Author: Denys Shvachka <d.shvachka@ukma.edu.ua>");
            println!("This tool was created as a part of the Rust course at National University of \"Kyiv-Mohyla Academy\".");
            println!("Use this tool for educational or personal purposes only.");
        }
        Commands::Help => {
            println!("Rieltor.ua apartment parser");
            println!();
            println!();
            println!("Commands:");
            println!(
                "parse   Parses a HTML file or fetched HTML from URL and displays its contents."
            );
            println!("\tArguments:");
            println!(
                "\t\t<source>              Specify the path to the HTML or URL file to parse."
            );
            println!("\t\t<output>     The path for saving the parsed result in JSON file. The file name is optional: it can be automatically generated.");
            println!("\tExamples:");
            println!("\t\tparser parse https://rieltor.ua/flats-rent/view/12345678 apartment.json");
            println!("\t\tparser parse fetched_apartment.html");
            println!();
            println!("parse_list   Parses a HTML file or fetched HTML from URL with list of apartments and displays their contents.");
            println!("\tArguments:");
            println!(
                "\t\t<source>              Specify the path to the HTML or URL file to parse."
            );
            println!("\t\t<output>     The path for saving the parsed result in directory. The directory name is optional: it can be automatically generated in project output directory.");
            println!("\tExamples:");
            println!("\t\tparser parse_list https://rieltor.ua/poltava/flats-rent/?price_min=8750\"&\"price_max=15000");
            println!("\t\tparser parse_list fetched_apartment_list.html");
            println!();
            println!("credits Shows credits and authorship information.");
            println!();
            println!("help    Displays this help information.");
            println!();
        }
    }

    Ok(())
}
