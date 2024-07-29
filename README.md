# Bloggy

A minimalistic blog builder.

Build a file based structure blog in less than a second.

## Structure

2 files templates are **required** :

<details>
  <summary>index.html</summary>

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Hello world</title>
    <link rel="stylesheet" href="/style.css" />
  </head>
  <body>
    <nav class="nav">
      <a href="/" class="nav-title">~/blog</a>
    </nav>

    <main class="articles-list">
      <div class="article">
        <a href="/blog/new-article" class="article-link">
          <h2 class="article-title">new fancy article !</h2>
          <p class="article-description">
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque
            elementum a quam a varius.
          </p>
        </a>
      </div>

      <div class="article">
        <a href="/blog/random-article" class="article-link">
          <h2 class="article-title">Random article</h2>
          <p class="article-description">
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque
            elementum a quam a varius.
          </p>
        </a>
      </div>
    </main>
  </body>
</html>
```

</details>
<details>
  <summary>article.html</summary>

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{{article.title}}</title>
    <link rel="stylesheet" href="/style.css" />
    <meta name="description" content="{{article.description}}" />
  </head>
  <body>
    {{nav}}
    <main class="article-detail">
      <h1 class="article-title">{{article.title}}</h1>
      <article>{{article.content}}</article>
    </main>
    <script type="module">
      import { codeToHtml } from "https://esm.sh/shiki@1.0.0";
      const codeBlocks = [...document.querySelectorAll("code")];

      codeBlocks.forEach(async (codeBlock) => {
        codeBlock.innerHTML = await codeToHtml(codeBlock.innerText, {
          lang: codeBlock.className.replace("language-", ""),
          theme: "github-dark",
        });
      });
    </script>
  </body>
</html>
```

</details>

```
./
  templates/
    index.html
    article.html
  blog/
    article-title/
      index.md
      article-assets.png
```

### Templates

Available variables :

```rust
struct Article {
  title: String,
  description: String,
  content: String,
  path: String,
  tags: Vec<String>,
}
```
