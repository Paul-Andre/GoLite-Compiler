# Milestone 1

## Implementation

We decided to used flex+bison for parsing and Rust for all phases afterwards.

We used structs and Rust enums (tagged unions) to represent our AST.

We wrote out the flex scanner as usual. In the the bison file we called Rust functions to build the AST.

We decided to pass around opaque pointers in the parser actions, including opaque pointers to Rust vectors.

### Rationale

We used Flex and Bison because they were tool we knew how to use. We decided to use Rust for the ability to pattern match and manipulate trees in a memory-safe manner.


## Team Organization

We did not have a very strict team organization. When we would code together we would either decide what micro-tasks everyone should do on the spot, or have one person code and the other help (particularly for tasks that required making design decisions, like writing out the AST structure). When we were not together, people would volunteer or we would decide at the moment who should do a task.

The scanner was done together.

The grammar for the parser was mostly done by Paul.

The AST structure was decided and coded together.

The AST builder functions (including the boilerplate required to link with C) was done mostly by James and Paul.

The rule actions for the parser were done by Youri.

The pretty printer was mostly done by James.
