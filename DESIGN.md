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

The Handler trait defines a callback interface that he parser invokes as it 
processes the DICOM DataSet byte stream.  Using a callback interface provides
the following benefits compared to a non callback interface:

* Immediate accessibility - each DICOM Attribute is made available via the handler
  as it is parsed. 
* Efficient processing - different use cases require different processing of
  the DICOM DataSet.  The Handler abstraction introduces no overhead to the
  processing allowing highly efficient and optimized processing
  implementations.
* Flexible control logic.  Some use cases do not require full parsing of the
  DICOM DataSet byte stream.  The Handler trait enables implementations to
  implement custom control logic to cancel the parsing at any point which
  avoids unnecessary resource utilization.  
