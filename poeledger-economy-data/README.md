# poeledger-economy-data

A Rust crate and generated Typescript lib for converting poe.ninja CSV data dumps into usable Rust structs

## Typscript Generation

The Rust library is annotated with `typeshare` annotations as needed, and the Typeshare CLI can be used to generate a basic Typescript library to keep types in sync between the two languages. Here's the Typeshare CLI command to use:
```sh
typeshare . --lang=typescript --output-file=generated/lib.ts
```
Some types can't be mapped to Typescript, such as `NaiveDate` from `chrono`, so we must manually replace this with `Date` in the generated bindings
