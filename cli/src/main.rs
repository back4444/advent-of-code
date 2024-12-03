mod error;

use clap::{Parser, Subcommand};
use dotenv::dotenv;
use error::CustomError;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Subcommand)]
enum SubCommands {
    Scaffold {
        #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..=25))]
        day: u32,
    },
}

// THIS SHOULD BE RUN RELATIVE TO THE WORKSPACE ROOT
// cargo run -p cli -- scaffold -d <day>
fn main() {
    dotenv().ok();

    let cli = Cli::parse();

    if let Err(e) = match cli.command {
        SubCommands::Scaffold { day } => {
            let workspace_root = std::env::current_dir()
                .expect("could not find workspace root")
                .to_path_buf();

            let path = workspace_root.join(format!("day-{:02}", day));
            scaffold_project(&path, day)
        }
    } {
        eprintln!("Error: {}", e)
    }
}

fn scaffold_project(path: &Path, day: u32) -> Result<(), CustomError> {
    let day_string = format!("day-{:02}", day);

    let part_template = read_template("part.txt")?.replace("<day>", &day_string);
    let cargo_toml_template = read_template("toml.txt")?.replace("<day>", &day_string);

    let bin_path = path.join("src/bin");
    fs::create_dir_all(&bin_path)?;

    for part_name in ["part-1.rs", "part-2.rs"] {
        let file_path = &bin_path.join(part_name);
        let template = part_template.replace("<part>", &part_name.replace(".rs", ""));

        create_and_write_file(file_path, Some(template))?;
    }

    create_and_write_file(&path.join("Cargo.toml"), Some(cargo_toml_template))?;

    let session_token = std::env::var("SESSION").unwrap();
    let day_input = request_input(day, &session_token)?;
    create_and_write_file(&path.join("src/input.txt"), Some(day_input))?;

    create_and_write_file(&path.join("src/test.txt"), None)
}

fn read_template(name: &str) -> Result<String, CustomError> {
    fs::read_to_string(format!("cli/src/templates/{name}")).map_err(CustomError::Io)
}

fn create_and_write_file(path: &Path, content: Option<String>) -> Result<(), CustomError> {
    let mut file = OpenOptions::new().create_new(true).write(true).open(path)?;

    match content {
        Some(c) => file.write_all(c.as_bytes()).map_err(CustomError::Io),
        None => Ok(()),
    }
}

fn request_input(day: u32, session_token: &str) -> Result<String, CustomError> {
    let input_url = format!("https://adventofcode.com/2024/day/{}/input", day);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::COOKIE,
        reqwest::header::HeaderValue::from_str(&format!("session={session_token}")).unwrap(),
    );

    let client = reqwest::blocking::ClientBuilder::new()
        .default_headers(headers)
        .cookie_store(true)
        .build()?;

    let response = client.get(input_url).send()?;
    response.text().map_err(CustomError::Reqwest)
}
