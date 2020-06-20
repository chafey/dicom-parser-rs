# dicom-parser-rs
dicom parser written in Rust

## Design Goals

* Parse all standards compliant DICOM P10 files
* First class support for WebAssembly builds 
* Streaming compatible API
* SAX Style callback based parsing
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

Actively being developed (June 20, 2020)

## To Do's (before first version release)

* Add no_std configuration?
* Add example applications
  * dump to DICOM JSON format
  * dump to text (in DCMTK dcmdump format) - requires data dictionary though
* Test library with large number of images (GDCM test images, etc)
  * Would be nice to build an automated regression suite

## Performance Benchmarking

Add performance benchmark to understand parser performance for various use
cases and data variations.  Establishing a baseline here will help avoid
performance regressions due to future changes and also serve as a useful
comparsion agains other parsing implementations.

* Use Cases
  * SOP Instance Identification - extract Study,Series and SOPInstanceUID Only
  * Basic Image Display - Extract minimum attributes to display image (ImagePixel Module)
  * Basic Encoding Validation - make sure the entire dataset can be parsed (ignore data though)
  * Basic Metadata - extract basic metadata like patietn demographics, study description, etc
  * Metadata ingestion - parse all metadata fields
  * Pixel Ingestion - parse all pixel data
* Data Variations
  * Transfer Syntaxes (Implicit Little Endian, Explicit Little Endian, Explicit Big Endian)
  * Pixel Data (uncompressed, compressed/encapsulated not fragmented, encapsulated fragmented)
  * Lengths (defined and undefined)
  * Sizes (large multiframe, small single frame)
  * Private Attributes (large, small)
  * Stream Parsing - full dataset, chunked, single bytes

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
* replace unit test dependencies on actual P10 files with synthetic data 