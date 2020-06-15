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
    * [X] Cancel Parsing
    * [ ] Skip Element Data
* [X] DICOM P10 Meta Information
* [X] Explicit Little Endian Transfer Syntax
* [X] Streaming Parser
* [X] Implicit Little Endian Transfer Syntax
* [X] Explicit Big Endian Transfer Syntax
* [X] Encapsulated Pixel Data
* [X] Sequences with known lengths
* [X] Sequences with undefined lengths
* [X] UN with undefined lengths
* [ ] Deflate Transfer Syntax

## Status

Actively being developed (June 15, 2020)

## To Do's

* [ ] Get rid of ParseState::Partial
* [ ] Implement skip element data parsing
* [ ] Separate undefined length logic from known length logic
* [ ] Add more unit tests
* [ ] Add streaming test app that verifies stream operation by parsing one byte at a time
* [ ] implement data skipping for sequence attributes?
* [ ] Add design documentation
* [ ] Add example applications
* [ ] Add no_std configuration?
* [ ] Build test suite of DICOM images that hit all conditions
* [ ] Create handler that produces DICOM JSON?
* [ ] Consider helpers to convert attribute data into rust types (e.g. strings, numbers, etc)
* [ ] Consider adding TagCancelHandler to cancel parsing on specific tag (or tags)
* [ ] Consider consolidating PixelDataFragmentParser and BasicOffsetTableParser into EncapsulatedPixelDataParser
* [ ] Refactor parse_tag_and_length() so we don't have two of them - perhaps replace with parse_attribute()?