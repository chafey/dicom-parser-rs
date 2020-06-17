# Design Rationale for dicom-parser-rs

This DICOM parsing library takes a minimalist approach similar to the
CornerstoneJS JavaScript dicom-parser library.  The minimalist approach
targets providing the minimum functionality possible but designed to be
flexible so it can be used as a building block for more complex functionality.
This library does not try to provide everything that might be needed from a
DICOM library - it is focused on parsing the DICOM DataSet binary format.
This is enough functionality for many use cases, but is also suitable
for building higher levels of functionality such as writing DICOM binary
byte streams, de-identification, image decompression, etc.

## Background

This is the second DICOM parsing library I have written from scratch.  The
first was the CornerstoneJS DICOM Parser which started out as a quick and dirty
hack to support my CornerstoneJS development.  It grew over time due to my own
needs taking it beyond a hack to actual production use as well as others who
provided fixes and feature enhancments.  While the original design has held up
fairly well for its intended purpose of parsing an entire DICOM P10 instance,
there are several things I have wanted to change in it based on new 
requirements/needs and an improved understanding of DICOM.  Some of this
improved understanding was learned due to fixing issues in the CornersoneJS
DICOM parser library.

## Data Dictionary

This library does not utilize a DICOM Data Dictionary as it isn't needed to
parse the DICOM DataSet binary format.  When it comes to DICOM Data 
Dictionaries, DICOM parsing use cases generally fall into one of the
following categories:
* Those that don't requre a Data Dictionary at all.  Viewing a DICOM image
  does not typically require a Data Dictionary, but some tools or other
  features of a viewer may require it.  
* Those that will make use of a Data Dictionary if available for a better
  user experience.  In this case, the Data Dictionary does not have to be
  comprehensive - there may be some tags encountered that are not in the 
  Data Dictionary.  An example of this is a feature to visualize or dump
  a DICOM Data Set for debugging purposes.
* Those that require a comprehensive Data Dictionary.  In this case, the
  feature requires an entry for specific tags or it cannot function.  An
  example of this is functionality to convert an implicit little endian
  byte stream to an explicit little endian transfer syntax.

## Streaming

Full support for streaming was an important require in the design of this
library.  Streaming provides the following benefits compared to a design
that doesn't support streaming:

* minimizes resource utilization
* improves system concurrency
* reduces costs
* improves consistency of perceived user experience

The only drawback to a streaming based design is increased complexity of
the implementation.

## Callback

The Handler trait defines a callback interface that the parser invokes as it 
processes the DICOM DataSet byte stream.  Using a callback interface provides
the following benefits compared to a non callback interface:

* Immediate accessibility - each DICOM Attribute is immediately made available
  to the Handler as it is parsed.  In non callback designs, the attributes
  are not available until parsing is complete thus delaying access.
* Efficient processing - different use cases require different processing of
  the DICOM DataSet.  The Handler abstraction introduces no overhead to the
  processing allowing highly efficient and optimized processing
  implementations.  In non callback designs, the result of a parse is usually
  a DataSet type object which the caller can interpret.  Efficienes can be
  gained by avoiding the construction of this intermediary DataSet object
  and let the processing logic access the underlying data stream directly.
  Note that a DataSet parse interface can still be provided on top of a
  callback interface by creating a Handler that produces a DataSet type object.
* Flexible control logic.  Some use cases do not require full parsing of the
  DICOM DataSet byte stream.  The Handler trait enables implementations to
  implement custom control logic to cancel the parsing at any point which
  avoids unnecessary resource utilization.  In non callback based designs, the
  parser needs to implement different control logics itself which may not be
  as flexible as a callback based design.  Implementing control logic in the
  parser also complicates the parser implementation.

## Modularity

The library is built in a modular way such that it can be easily used to handle
a variety of use cases.  For example, the DataSetParser can be used directly
to parse non DICOM P10 byte streams.  The MetaInformation class can be used
directly to read the P10 header.  

## Encapsulating Complexity

There is a large ammount of condition logic involved with parsing DICOM 
DataSets.  When I designed the CornerstoneJS dicom parser, the design was
limited by the available language features in JavaScript at that time.  The
strategy I used was to break the complexity down into functions for each
major condition branch.  For example - explicit vs implicit, big endian vs 
little endian, undefined length vs known lengths.  This approach made the
code easier to debug and understand, but resulted in a lot of code 
duplication.

Since this library is built on Rust, there are powerful language features such
as generics and traits which can be used to improve the encapsulation, 
improve the performance and eliminate code duplication with no trade offs.
Some applications of these features:

* Encoding trait provides an interface for encapsulating logic for decoding
  big endian vs little endian and explict vs implicit.  There are three 
  concrete classes which encapsulate the three variations: 
  ExplicitLittleEndian, ImplicitLittleEndian, ExplicitBigEndian.  

* ValueParser trait provides an interface for parsing the value field of
  a DICOM Attribute for a specific Encoding.  The Encoding is provided via
  a generic parameter T to ValueParser.

* Handler trait provides a callback interface for the parser to notify the
  consumer/user of what has been parsed.  Users of this library can
  use a provided concrete Handler implementation or create their own.  
  The CancelProvider

## Words of Wisdom

I discovered a few things while implementing this library that I wish I had 
known before starting.  I wanted to capture them for those that want to 
implement their own parser, or if I happen to do so again some day.

* The most complex part of parsing DICOM is parsing implicit little endian
  undefined length attributes.  To handle this case properly, you must check
  to see if it is a sequence and if so, parse it fully.  It is good to
  implement this case ASAP as it tends to impact the rest of the design.

* The variability associated with parsing the value field of an attribute is
  independent of parsing DataSets and attribute Tag/VR/Length.  Initially they
  seem similar since the code for parsing a DataSet and Attribute is used
  when parsing the value field (e.g. sequences).  Intially I implemented
  DataSet and Attribute Parsing behind the same trait/interface that I used
  for value parsing along with a state pattern which ended up being fairly
  brittle.  The design became more elegant and cleaner when I used the trait
  to encapsulate value field parsing only.

* It is tempting to combine parsing for undefined length with known length
  as the logic is very similar.  When the parsing logic for these two are
  combined, changing one tends to break the other.  It may be more efficient
  to start out implementing these cases separately and then combine them
  later once everything is working properly. 

