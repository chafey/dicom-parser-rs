# Design Rationale for dicom-parser-rs

## Scope

This DICOM parsing library takes a minimalist approach similar to the
CornerstoneJS JavaScript dicom-parser library.  The goal of the minimalist
approach is to provide the least ammount of functionality that is still
useful and designed in a way where it can be used as a building block
to do more complex things.  This library therefore is focused on doing
one thing and one thing only - parsing DICOM binary byte streams.  
This focused piece of functionality is enough to support the needs for
many use cases.  Those that need more functionality can layer it on top
of this library.  It is in scope that this library has the right design to be
used as a building block to do more complex things such as writing DICOM
binary byte streams, DICOM JSON, de-identification, image decompression, etc.

## Background

This is the second DICOM parsing library I have written from scratch.  The
first was the CornerstoneJS DICOM Parser which started out as a quick and dirty
hack to support my CornerstoneJS development.  It grew over time due to my own
needs to use it in a production application as well as others who
provided fixes and feature enhancments.  While the original design has held up
fairly well for its intended purpose of parsing an entire DICOM P10 instance, 
there are new requirements that I want out of a dicom parser that require
a new design.  

## Data Dictionary

This library does not utilize a DICOM Data Dictionary as it isn't needed to
parse the DICOM DataSet binary format.  When it comes to DICOM Data 
Dictionaries, DICOM parsing use cases generally fall into one of the
following categories:
* Those that don't requre a Data Dictionary at all.  Most use cases fall
  into this category including viewing of images.
* Those that will make use of a Data Dictionary if available to deliver a
  better user experience.  For these use cases, the Data Dictionary does
  not have to be complete or up to date.  An example of this is a feature
  to visualize or dump a DICOM Data Set for debugging purposes.  If a 
  given tag is in the DataSet but not in the dictionary, the tool will
  still work but it won't have information about that specific tag.
* Those that may fail if they encounter an attribute not in the Data Dictionary.
  Having functionality that can fail is rare and generally undesirable - if
  this situation occurs, there are usually ways to workaround them (perhaps
  with some drawbacks).  An example of such a use case is converting the 
  transfer syntax for a DICOM P10 file from implicit little endian to explicit
  little endian.

## Streaming

Full support for streaming was an important requirement in the design of this
library.  Streaming provides the following benefits compared to a design
that doesn't support streaming:

* minimizes resource utilization - specifically memory, but also CPU, network
  and disk.  Non streaming libraries usually require loading the entire 
  byte stream into memory before parsing it.  Smart designs can leverage
  streaming to avoid additional reads/writes of byte streams.
* improves system concurrency - mainly due to reduced resource utilization above
* reduces costs - mainly due to reduced resource utilization above
* improves consistency of perceived user experience - mainly due to reduced 
  resource utilization above.

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
  a DataSet type object which the caller can interpret.  Efficiencies can be
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
  consumer/user of what has been parsed.  Complex processing logic can
  be broken up into multiple Handler implementations which can be chained
  together (see the included CancelHandler for an example of this)

## Words of Wisdom

I discovered a few things while implementing this library that I wish I had 
known before starting.  I wanted to capture them for those that want to 
implement their own parser, or if I happen to implement another DICOM parsing
library some day.

* The most complex part of parsing DICOM is parsing implicit little endian
  undefined length attributes.  To handle this case properly, you must check
  to see if it is a sequence and if so, parse it fully.  It is good to
  implement this case ASAP as it tends to impact the rest of the design.

* The variability associated with parsing the value field of an Attribute is
  independent of parsing DataSets and Attribute Tag/VR/Length.  Initially they
  seem similar since the code for parsing a DataSet and Attribute is used
  when parsing the value field (e.g. sequences).  Intially I implemented
  DataSet and Attribute Parsing behind the same trait/interface that I used
  for value parsing along with a state pattern which ended up being fairly
  brittle.  The design became more elegant and the codebase became cleaner
  when I used the trait to encapsulate value field parsing only and separated
  the DataSetParser and AttributeParser from it.

* It is tempting to combine parsing for undefined length with known length
  as the logic is very similar.  When the parsing logic for these two are
  combined, changing one tends to break the other.  It may be more efficient
  to start out implementing these cases separately and then combine them
  later once everything is working properly. 

