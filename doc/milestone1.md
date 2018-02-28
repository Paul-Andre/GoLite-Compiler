# Milestone 1

## Implementation

We decided to use flex+bison for parsing and Rust for all phases afterwards.

We used structs and Rust enums (tagged unions) to represent our AST.

We wrote out the flex scanner as usual. In the the bison file we called the Rust functions declared in the ast header file to build the AST.

We decided to pass around opaque pointers in the parser actions, including opaque pointers to Rust vectors.

We performed weeding at two levels. First, when constructing the AST we check the following things:
* Unicity of default case in switch statements 
* Assignments need to have the same number of elements on each side
* The post simple statement of a ForClause cannot be an assignment
* Expression statements can only be functions

We then do a recursive pass through the AST and check the following things:
* Incorrect break and continue usage
* Incorrect blank identifier usage

### Rationale

We used Flex and Bison because they were tools we knew how to use. We decided to use Rust for the ability to pattern match and manipulate trees in a memory-safe manner. And because Paul wanted to.


## Team Organization

We did not have a very strict team organization. When we would code together we would either decide what micro-tasks everyone should do on the spot, or have one person code and the other help (particularly for tasks that required making design decisions, like writing out the AST structure). When we were not together, people would volunteer or we would decide at the moment who should do a task.

The scanner was done together.

The grammar for the parser was mostly done by Paul.

The AST structure was decided and coded together.

The AST builder functions (including the boilerplate required to link with C) was done mostly by James and Paul.

The rule actions for the parser were done by Youri.

The pretty printer was mostly done by James.
