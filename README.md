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

Actively being developed (June 19, 2020)

## To Do's

* Reconsider DataSet object - either delete or make it complete (it is not useful currently)
  * Consider removing DataSetHandler to test code?
* Add no_std configuration?
* Add example applications
  * dump to DICOM JSON format
  * dump to text (in DCMTK dcmdump format) - requires data dictionary though
* Add performance benchmark to establish baseline to understand performance implications of future changes
* Build test suite of DICOM images that hit all conditions

## Possible Future Functionality

* Consider helpers to convert attribute data into rust types (e.g. strings, numbers, etc)
  * Note: meta_information already has functionality to convert to utf8 strings
* Create handler that produces DICOM JSON?
* Consider adding FilterHandler that filters out handler calls for specific attributes.  
* Consider adding TagCancelHandler to cancel parsing on specific tag (or tags)
* Consider making a cancelled parse resumable?  Should work given that the parser is streaming capable
* Add P10StreamParser which provides a simplified interface for stream parsing by buffering data from
  incomplete parses
* Consider adding a Handler that aggregates mutliple data callbacks into a single buffer 
* Explore ways to automate mapping from Handler to types in a struct, perhaps using macros?
  * would be nice to be able to do something like: !map(0x0020, 0x000D, &self.study_instance_uid);
  * could use this for creating the MetaInformation struct rather than a custom Handler

## Refactorings

* Separate undefined length logic from known length logic
  * SequenceItemDataParser->SequenceItemDataUndefinedLengthParser
  * SequenceParser -> SequenceUndefinedLengthParser 
* Refactor parse_tag_and_length() so we don't have two of them - perhaps replace with parse_attribute()?
