# dicom-parser-rs
dicom parser written in Rust

## Goals

* Parse all standards compliant DICOM P10 files
* First class support for WebAssembly builds 
* Streaming compatible API
* Callback based parsing
* Does not utilize data dictionary

## Features

* [X] Callback based parsing
    * [X] Stop Parsing
    * [X] Skip Element Data
* [X] DICOM P10 Meta Information
* [X] Explicit Little Endian Transfer Syntax
* [X] Streaming Parser
* [X] Implicit Little Endian Transfer Syntax
* [ ] Deflate Transfer Syntax
* [ ] Explicit Big Endian Transfer Syntax
* [ ] Undefined Lengths
* [ ] Sequences
* [ ] Encapsulated Pixel Data
* [ ] Character Sets

## Status

Actively being developed (June 8, 2020)

