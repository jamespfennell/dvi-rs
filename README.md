# dvi-rs
A parser and dumper for the "device independent file" (dvi) format in rust

## Cargo features

There is a single Cargo feature `parse` that is enabled by default.
With this feature disabled, the crate has only a single direct dependency
  (and no transitive dependencies), but can only be used for dumping DVI data.

## TODO
 - At the moment, the "parse" function actually lexes. Could parse into a list
   of pages and fonts.
 - Documentation/examples, currently best examples are the tests
