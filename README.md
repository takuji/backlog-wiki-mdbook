# backlog-wiki-mdbook

A command line tool to create an [mdBook](https://rust-lang.github.io/mdBook/) project from wiki pages of a Backlog project.

This only works for the projects where "Markdown" is used for the text formatting rule.

## How to use

```
backlog-wiki-mdbook --apikey=xxxxxxxxxx --domain=(SPACE_DOMAIN) --project=(PROJECT_KEY) --dir=(OUTPUT_DIRECTORY) --build
```

## Options

```
      --apikey <APIKEY>    API key
      --domain <DOMAIN>    Domain of your space (e.g. example.backlog.com)
      --project <PROJECT>  Project key (e.g. EXAMPLE)
      --dir <DIR>          Directory to create the book in
      --build              Build the book after creating an mdBook directory
  -h, --help               Print help
  -V, --version            Print version
```
