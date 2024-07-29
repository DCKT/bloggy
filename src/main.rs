use std::{
    fs,
    path::{self},
    vec,
};

use clap::{Parser, Subcommand};
use gray_matter::engine::YAML;
use gray_matter::Matter;
use markdown::{CompileOptions, Options};
use minijinja::{context, Environment};

#[derive(serde::Serialize)]
struct Article {
    title: String,
    description: String,
    content: String,
    path: String,
    tags: Vec<String>,
    #[serde(skip)]
    assets: Vec<ArticleAsset>,
}

#[derive(serde::Deserialize)]
struct FrontMatter {
    title: String,
    description: String,
    #[serde(default)]
    tags: Vec<String>,
}

struct ArticleAsset {
    path: String,
    file_name: String,
}

const OUTPUT_FOLDER_PATH: &str = "./dist";

fn build(working_path: &str) {
    rm_rf::ensure_removed(OUTPUT_FOLDER_PATH).unwrap();
    let absolute_working_path = path::absolute(working_path).expect("Invalid path");
    let mut articles: Vec<Article> = vec![];
    let mut env = Environment::new();
    let index_template = {
        let path = absolute_working_path.as_path().join("templates/index.html");
        fs::read_to_string(path).expect("index.html is missing")
    };
    let article_template = {
        let path = absolute_working_path
            .as_path()
            .join("templates/article.html");
        fs::read_to_string(path).expect("article.html is missing")
    };
    let nav_template = {
        let path = absolute_working_path.as_path().join("templates/nav.html");
        fs::read_to_string(path).expect("nav.html is missing")
    };

    env.add_template("index", &index_template).unwrap();
    env.add_template("article", &article_template).unwrap();

    let index_template = env.get_template("index").unwrap();
    let article_template = env.get_template("article").unwrap();

    fs::create_dir(OUTPUT_FOLDER_PATH).expect("Unable to create \"dist\" folder");
    fs::create_dir(String::from(OUTPUT_FOLDER_PATH) + "/blog")
        .expect("Unable to create \"blog\" folder");

    let blog_files = {
        let path = absolute_working_path.as_path().join("blog");
        fs::read_dir(path).expect("Missing \"blog\" folder")
    };

    for blog_file in blog_files.into_iter() {
        let blog_file = blog_file.unwrap();
        let file_name = blog_file.file_name();
        let article_path = blog_file.path().as_path().join("index.md");
        let article_md = fs::read_to_string(&article_path).expect(
            format!(
                "markdown file is missing from {}",
                article_path.to_str().unwrap()
            )
            .as_str(),
        );
        let matter = Matter::<YAML>::new();

        let assets = fs::read_dir(blog_file.path()).unwrap();

        let matter_result = matter
            .parse_with_struct::<FrontMatter>(&article_md)
            .expect("Invalid frontmatter content");

        let html_content = markdown::to_html_with_options(
            matter_result.content.as_str(),
            &Options {
                compile: CompileOptions {
                    allow_dangerous_html: true,
                    ..CompileOptions::default()
                },
                ..Options::default()
            },
        )
        .unwrap();

        let article = Article {
            title: matter_result.data.title,
            description: matter_result.data.description,
            content: html_content,
            tags: matter_result.data.tags,
            path: String::from("/blog/") + file_name.to_str().unwrap(),
            assets: assets
                .into_iter()
                .filter_map(|file| {
                    let file = file.unwrap();

                    if file.file_name() != "index.md" {
                        Some(ArticleAsset {
                            path: String::from(file.path().to_str().unwrap()),
                            file_name: file.file_name().into_string().unwrap(),
                        })
                    } else {
                        None
                    }
                })
                .collect(),
        };

        articles.push(article);
    }

    // Write index.html to dist/
    fs::write(
        String::from(OUTPUT_FOLDER_PATH) + "/index.html",
        index_template
            .render(context! {
                articles,
                nav => nav_template
            })
            .expect("unable to render index template"),
    )
    .expect(format!("Unable to write {}/index.html", OUTPUT_FOLDER_PATH).as_str());

    // For each article, create an index.html file and copy his assets
    for article in articles {
        let article_file_path = String::from(OUTPUT_FOLDER_PATH) + &article.path;

        fs::create_dir(&article_file_path)
            .expect(format!("Unable to create {} folder", article_file_path).as_str());

        fs::write(
            article_file_path.clone() + "/index.html",
            article_template
                .render(context! {
                    article,
                    nav => nav_template
                })
                .expect("Unable to render article template"),
        )
        .expect(format!("Unable to write {}", article_file_path).as_str());

        for asset in article.assets {
            let _ = fs::copy(
                asset.path,
                String::from(OUTPUT_FOLDER_PATH) + article.path.as_str() + "/" + &asset.file_name,
            )
            .expect(format!("\nUnable to copy asset from {} \n\n", asset.file_name).as_str());
        }
    }

    fs::copy(
        absolute_working_path.as_path().join("templates/style.css"),
        String::from(OUTPUT_FOLDER_PATH) + "/style.css",
    )
    .expect("Unable to copy style.css");

    println!("\n✅ Site built")
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    build: Option<BuildCommands>,
}

#[derive(Subcommand)]
enum BuildCommands {
    Build {
        #[arg(short, long)]
        path: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.build {
        Some(BuildCommands::Build { path }) => {
            let working_path = match path.as_deref() {
                None => "./",
                Some(p) => p,
            };

            build(working_path);
        }
        None => (),
    }
}
