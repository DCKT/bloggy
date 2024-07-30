# Bloggy

A minimalistic blog builder.

Build a file based structure blog in less than a second.

## Structure

```
./
  public/
    styles.css
  templates/
    index.html
    article.html
  blog/
    article-title/
      index.md
      article-assets.png
```

2 HTML files templates are **required** :

<details>
  <summary>index.html</summary>

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Hello world</title>
    <link rel="stylesheet" href="/public/style.css" />
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
    <link rel="stylesheet" href="/public/style.css" />
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

A CSS `styles.css` is required in the `public` folder
<details>
  <summary>styles.css</summary>

```css
/*
  1. Use a more-intuitive box-sizing model.
*/
*,
*::before,
*::after {
  box-sizing: border-box;
}
/*
  2. Remove default margin
*/
* {
  margin: 0;
}
/*
  Typographic tweaks!
  3. Add accessible line-height
  4. Improve text rendering
*/
body {
  line-height: 1.5;
  -webkit-font-smoothing: antialiased;
}
/*
  5. Improve media defaults
*/
img,
picture,
video,
canvas,
svg {
  display: block;
  max-width: 100%;
}
/*
  6. Remove built-in form typography styles
*/
input,
button,
textarea,
select {
  font: inherit;
}
/*
  7. Avoid text overflows
*/
p,
h1,
h2,
h3,
h4,
h5,
h6 {
  overflow-wrap: break-word;
}

@import url("https://fonts.googleapis.com/css2?family=Nunito+Sans:ital,opsz,wght@0,6..12,200..1000;1,6..12,200..1000&display=swap");
@import url("https://fonts.googleapis.com/css2?family=JetBrains+Mono:ital,wght@0,100..800;1,100..800&display=swap");

/* https://catppuccin.com/palette */
:root {
  /* Mocha theme */
  --rosewater: #f5e0dc;
  --flamingo: #f2cdcd;
  --pink: #f5c2e7;
  --mauve: #cba6f7;
  --red: #f38ba8;
  --maroon: #eba0ac;
  --peach: #fab387;
  --yellow: #f9e2af;
  --green: #a6e3a1;
  --teal: #94e2d5;
  --sky: #89dceb;
  --sapphire: #74c7ec;
  --blue: #89b4fa;
  --lavender: #b4befe;
  --text: #cdd6f4;
  --subtext-1: #bac2de;
  --subtext-0: #a6adc8;
  --overlay-2: #9399b2;
  --overlay-1: #7f849c;
  --overlay-0: #6c7086;
  --surface-2: #585b70;
  --surface-1: #45475a;
  --surface-0: #313244;
  --base: #1e1e2e;
  --mantle: #181825;
  --crust: #11111b;

  --container: 650px;
}

body {
  background-color: var(--base);
  padding: 1rem;
  font-family: "Nunito Sans", sans-serif;
  font-optical-sizing: auto;
  text-decoration: none;
}
.nav {
  max-width: var(--container);
  margin: 0 auto 3rem auto;

  .nav-title {
    font-size: 2.25rem;
    font-weight: bold;
    color: var(--rosewater);
    font-family: "JetBrains Mono";
    text-decoration: none;
  }
}

.articles-list {
  max-width: var(--container);
  margin: auto;
  display: flex;
  flex-direction: column;
  gap: 24px;

  .article-link {
    font-size: 1.25rem;
    color: var(--peach);
    font-weight: 700;
    text-decoration: none;
    padding: 0.25rem 0.5rem;
    display: block;
    border-radius: 0.5rem;

    &:hover {
      background: #f5e0dc1c;
    }
  }

  .article-description {
    font-size: 1.125rem;
    font-weight: 400;
    color: var(--rosewater);
  }
}

.article-detail {
  max-width: var(--container);
  margin: auto;
}

.article-detail .article-title {
  font-size: 2.2rem;
  color: var(--peach);
  font-weight: 800;
}

.article-detail article {
  color: var(--rosewater);

  h2,
  h3,
  h4,
  h5,
  h6 {
    color: var(--yellow);
    margin-bottom: 0.25rem;
  }

  h2 {
    font-size: 2rem;
  }
  h3 {
    font-size: 1.75rem;
  }
  h4 {
    font-size: 1.525rem;
  }
  h5 {
    font-size: 1.3125rem;
  }
  h5 {
    font-size: 1.25rem;
  }
  h6 {
    font-size: 1.125rem;
  }

  strong {
    font-weight: 600;
    color: var(--yellow);
  }
  em {
    font-style: italic;
  }

  p {
    font-size: 1.125rem;
    margin-bottom: 1rem;
  }

  code pre {
    border-radius: 0.25rem;
    padding: 0.5rem 0.75rem;
    margin-bottom: 1rem;
    font-size: 0.875rem;
  }

  img {
    border-radius: 0.5rem;
    max-width: 100%;
    margin: auto;
    display: block;
  }
}
```

</details>

Now run the script : 
```sh
./bloggy build
```

You should see a `dist` folder with your blog built !


## Templates

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

For your blog post, you can use frontmatter :

```md
---
title: Random article
description: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque elementum a quam a varius.
tags:
  - Rust
---

My blog post
```

## How it works

The `build` commands will read every folders in the `blog` folders, copies each assets and transforms the `index.md` in an `index.html` file with his content in the current working directory under `dist/blog`.
The `public` folder is copied as his in `dist/public`.

Under the hood it uses the template engine [minijinja](https://docs.rs/minijinja/latest/minijinja/).

Note: the `build` command clean the `dist` folder.