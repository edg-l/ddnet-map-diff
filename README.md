# DDNet Map Diff

[![Version](https://img.shields.io/crates/v/ddnet-map-diff)](https://crates.io/crates/ddnet-map-diff)
[![Downloads](https://img.shields.io/crates/d/ddnet-map-diff)](https://crates.io/crates/ddnet-map-diff)
[![License](https://img.shields.io/crates/l/ddnet-map-diff)](https://crates.io/crates/ddnet-map-diff)
![Rust](https://github.com/edg-l/ddnet-map-diff/workflows/Rust/badge.svg)
[![Docs](https://docs.rs/ddnet-map-diff/badge.svg)](https://docs.rs/ddnet-map-diff)
[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/ddnet-map-diff.svg)](https://web.crev.dev/rust-reviews/crate/ddnet-map-diff/)

A DDNet map diff tool made in Rust using [twmap](https://docs.rs/twmap).

Inspired by the original Ravie's python script.

```
A DDraceNetwork map diff tool.

Usage: ddnet-map-diff <MAP_OLD> <MAP_NEW> <RESULT>

Arguments:
  <MAP_OLD>  The base map to compare
  <MAP_NEW>  The map to be compared
  <RESULT>   The resulting map path

Options:
  -h, --help     Print help
  -V, --version  Print version
```
