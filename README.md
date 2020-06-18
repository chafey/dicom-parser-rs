# dicom-parser-rs
dicom parser written in Rust

## Design Goals

* Parse all standards compliant DICOM P10 files
* First class support for WebAssembly builds 
* Streaming compatible API
* Callback based parsing
* Does not utilize a DICOM data dictionary
* Modular design enabling flexible re-use of the library functionality

Read about the [design rationale for this library](DESIGN.md)

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

Actively being developed (June 18, 2020)

## To Do's

* Reconsider DataSet object - either delete or make it complete (it is not useful currently)
  * Consider removing DataSetHandler to test code?
* Add no_std configuration?
* Add more unit tests
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
* Add P10StreamParser which provides a simplified interface for stream parsing by buffering data from
  incomplete parses
* Add a handler that aggregates mutliple data callbacks into a single buffer (requires change to make data streamble first)

## Refactorings

* Change MetaInformation to not use DataSetHandler/DataSet
  * Custom Handler or a new one that is more flexible
  * Want to be able to provide access to all MetaInformation attributes
* Separate undefined length logic from known length logic
  * SequenceItemDataParser->SequenceItemDataUndefinedLengthParser
  * SequenceParser -> SequenceUndefinedLengthParser 
* Refactor parse_tag_and_length() so we don't have two of them - perhaps replace with parse_attribute()?
