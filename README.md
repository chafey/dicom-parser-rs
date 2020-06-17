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

* Add no_std configuration?
* Reconsider DataSet object - either delete or make it complete (it is not useful currently)
  * Consider removing DataSetHandler to test code?
* Add more unit tests
* Add design documentation
* Add inline source documentation
* Add example applications
  * dump to DICOM JSON format
  * dump to text (in DCMTK dcmdump format) - requires data dictionary though
* Add performance benchmark to establish baseline to understand performance implications of future changes
* Build test suite of DICOM images that hit all conditions

## Possible Future Functionality

* Consider helpers to convert attribute data into rust types (e.g. strings, numbers, etc)
  * Note: already have meta_information::get_element() which will return a utf8 string
* Create handler that produces DICOM JSON?
* Consider adding FilterHandler that filters out handler calls for specific attributes.  
* Consider adding TagCancelHandler to cancel parsing on specific tag (or tags)
* Consider making a cancelled parse resumable?  Should work given that the parser is streaming capable

## Rafactorings

* Separate undefined length logic from known length logic
* Consider consolidating PixelDataFragmentParser and BasicOffsetTableParser into EncapsulatedPixelDataParser
* Refactor parse_tag_and_length() so we don't have two of them - perhaps replace with parse_attribute()?
* Consider moving DataSetParser and AttributeParser up a level out of the parser folder?