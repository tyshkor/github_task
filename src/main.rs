use std::{path::PathBuf, error::Error, fmt};

use clap::{ArgEnum, Parser};

use cli_table::{format::Justify, print_stdout, Table, Color, WithTitle};

use serde::Serialize;
use serde_json;

use hubcaps::{Credentials, Github};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {

    #[clap(long)]
    github_username: String,

    #[clap(long)]
    not_cached: bool,

    #[clap(long)]
    json: bool,

    #[clap(long, parse(from_os_str))]
    json_path: Option<PathBuf>,

    #[clap(long)]
    toml: bool,

    #[clap(long, parse(from_os_str))]
    toml_path: Option<PathBuf>,

    #[clap(long)]
    access_token: Option<String>,
}

#[derive(Debug, Table, Serialize)]
#[table(color = "Color::Green")]
pub struct Repo {
    #[table(color = "Color::Red")]
    name: String,
    #[table(color = "Color::Red")]
    url: String,
    #[table(color = "Color::Red")]
    description: String,
    #[table(color = "Color::Red")]
    star_count: u64,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let token = match args.access_token {
        Some(at) => Some(Credentials::Token(at)),
        None => None,
    };

    let github = Github::new(
        args.github_username.clone(),
        token,
      )?;

    let repositories_into = github.user_repos(args.github_username).list(&Default::default()).await?;

    let repositories:Vec<Repo> = repositories_into.iter().map(move |repo| {
            let repository = repo.clone();

            let description: String = match &repo.description {
                Some(descr) => descr.clone(),
                None => "None".to_string(), 
            };

            Repo {
                name: repo.name.clone(),
                url: repo.url.clone(),
                description,
                star_count: repo.stargazers_count.clone(),
            }

    }).collect();

    if !args.json && !args.toml {
        print_stdout(repositories.with_title());
    }

    if args.json && args.json_path.is_some() {

        let json_path = args.json_path.unwrap();

        let json = serde_json::to_string(&repositories).unwrap();

        std::fs::write(json_path, json);
    }

    if args.toml && args.toml_path.is_some() {

        let toml_path = args.toml_path.unwrap();

        let toml = toml::to_string(&repositories).unwrap();

        std::fs::write(toml_path, toml);
    }

    Ok(())
}