use std::fs;

use minijinja::{context, Environment};

#[derive(serde::Serialize)]
struct Article {
    title: String,
    content: String,
    path: String,
}

fn main() {
    rm_rf::ensure_removed("./dist").unwrap();
    let mut articles: Vec<Article> = vec![];
    let mut env = Environment::new();
    let index_template = fs::read_to_string("./index.html").expect("index.html is missing");
    let article_template = fs::read_to_string("./article.html").expect("article.html is missing");
    env.add_template("index", &index_template).unwrap();
    env.add_template("article", &article_template).unwrap();

    let blog_files = fs::read_dir("./blog").expect("Missing \"blog\" folder");

    for blog_file in blog_files.into_iter() {
        let blog_file = blog_file.unwrap();
        let file_name = blog_file.file_name();
        let markdown_path = String::from(file_name.to_str().unwrap()) + ".md";
        let article_path = blog_file.path().as_path().join(markdown_path);
        let article_md = fs::read_to_string(&article_path).expect(
            format!(
                "markdown file is missing from {}",
                article_path.to_str().unwrap()
            )
            .as_str(),
        );

        let article = Article {
            title: String::from("No title yet"),
            content: article_md,
            path: String::from("/blog/") + file_name.to_str().unwrap(),
        };

        articles.push(article);
    }

    let index_template = env.get_template("index").unwrap();
    let article_template = env.get_template("article").unwrap();

    fs::create_dir("./dist").expect("Unable to create \"dist\" folder");
    fs::create_dir("./dist/blog").expect("Unable to create \"blog\" folder");

    fs::write(
        "./dist/index.html",
        index_template
            .render(context! {
                articles
            })
            .expect("unable to render index template"),
    )
    .expect("Unable to write dist/index.html");

    for article in articles {
        let article_file_path = String::from("./dist") + &article.path + ".html";
        fs::write(
            article_file_path.clone(),
            article_template
                .render(context! {
                    article
                })
                .expect("Unable to render article template"),
        )
        .expect(format!("Unable to write {}", article_file_path).as_str());
    }
}
