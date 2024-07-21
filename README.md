# Minimalistic blog builder

Build a file based structure blog in less than a second.

## Structure

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