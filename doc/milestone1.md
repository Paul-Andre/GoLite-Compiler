# Milestone 1

## Implementation

We decided to use flex+bison toolchain for the lexing and parsing phase. We used Rust for all phases afterwards.

We used structs and Rust enums (tagged unions) to represent our AST.

We wrote out the flex scanner as usual. In the the bison file we called the Rust functions declared in the ast header file to build the AST.

We decided to pass around opaque pointers in the parser actions, including opaque pointers to Rust vectors.

We performed weeding at two levels. First, when constructing the AST we validate the following:
* The existence of at most one default switch case 
* Assignments having the same number of elements on each side
* That the post simple statement of a ForClause is not an assignment
* Expression statements can only be functions

We then do a recursive pass through the AST and weed out the following:
* Incorrect break and continue usage, i.e breaks and continues that don't exist within a loop
* Incorrect blank identifier usage, i.e any scenario where a blank identifier is attempted to be read

### Rationale

We used Flex and Bison because they were tools we knew how to use. We decided to use Rust for the ability to pattern match and manipulate trees in a memory-safe manner.


## Team Organization

We did not have a very strict team organization. When we would code together we would either decide what micro-tasks everyone should do on the spot, or have one person code and the other help (particularly for tasks that required making design decisions, like writing out the AST structure). When we were not together, people would volunteer or we would decide at the moment who should do a task.

We most communicated through Slack, and whenever someone would have time to start a task he would post what he was going to work on, and afterward post what he successfully accomplished.

We also used Slack to report bugs.

The scanner was done together.

The grammar for the parser was mostly done by Paul and James.

The AST structure was decided and coded together.

The AST builder functions (including the boilerplate required to link with C) was done mostly by James and Paul.

The rule actions for the parser were done by Youri.

The post-AST-construction weeding phase and pretty printer was mostly done by James.

Paul fixed a lot of pretty printer bugs and made sure the pretty printer actually printed pretty.
