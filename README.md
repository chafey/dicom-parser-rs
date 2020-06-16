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

Actively being developed (June 16, 2020)

## To Do's

* Consider enhancing the Handler API to receive the parser state for nested attributes (sequences)
  * Could be some kind of path, e.g. 00186011/0/x00186012
  * Consider adding FilterHandler that filters out handler calls for specific attributes.  
* Add more unit tests
* Add design documentation
* Add inline source documentation
* Add example applications
* Add no_std configuration?
* Build test suite of DICOM images that hit all conditions
* Create handler that produces DICOM JSON?
* Consider helpers to convert attribute data into rust types (e.g. strings, numbers, etc)
  * Note: already have meta_information::get_element() which will return a utf8 string
* Consider adding TagCancelHandler to cancel parsing on specific tag (or tags)
* Consider making a cancelled parse resumable?  Should work given that the parser is streaming capable
* Consider adding data_position property to Attribute?
* Reconsider DataSet object - either delete or make it complete (it is not useful currently)
  * Consider removing DataSetHandler to test code?

## Rafactorings

* Change Parser's to use Attribute by reference (not by value)
* Separate undefined length logic from known length logic
* Consider consolidating PixelDataFragmentParser and BasicOffsetTableParser into EncapsulatedPixelDataParser
* Refactor parse_tag_and_length() so we don't have two of them - perhaps replace with parse_attribute()?
