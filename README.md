# Group 4's GoLite Compiler

A compiler for a subset of Go, built in Rust and Flex/Bison, as a semester project for McGill's compiler course.

Our team tied first place for implementation accuracy!

The specs for this specific subset of Go were provided in the project description:

* [Lexing + Parsing Specs](http://www.cs.mcgill.ca/~cs520/2018/project/Milestone1_Specifications.pdf)
* [Typechecking Specs](http://www.cs.mcgill.ca/~cs520/2018/project/Milestone1_Specifications.pdf)

We were allowed to emit any language (other than C++) in the backend, so we chose JavaScript for pragmatic reasons.

2025 Update: I (Paul-Andre) added an "interpreter" mode to practice my Rust.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.
### Prerequisites

In order to run this project locally, you must have the following installed:

* A C compiler (i.e GCC or Clang) -- This shouldn't be an issue for most computers
* [Flex](https://github.com/westes/flex) - Scanner Generator
* [Bison](https://www.gnu.org/software/bison/) - Parser Generator
* [Rust](https://www.rust-lang.org/en-US/) - Used for all other phases of compilation

The following command will get Rust installed on most computers:
```
curl https://sh.rustup.rs -sSf | sh
```

### Installing

The first step is to make sure you have to prerequisites installed. This essentially means making sure that you have Rust installed locally.

After you have to proper environment installed, you can clone the repository:

```
git clone https://github.com/comp520/2018_group04.git
```


## Running the compiler

### Building 

In order to build the source code, run:

```
./build.sh
```

If you ever need to do some changes to the C files for some reason, you will need to force cargo to rebuild as it will not detect the changes. Simply clean the directory using:

```
cargo clean
```

Then build again.

### Running a single phase

To run a single phase on a single file, run:

```
./run.sh <mode> <file>
```

### Running all test programs completely

To run the tests that sit in the programs directory, run:

```
./test.sh
```


## Built With

* [C](https://en.wikipedia.org/wiki/C_(programming_language)) - Used in order to run Bison and Flex
* [Flex](https://github.com/westes/flex) - Our scanner generator of choice
* [Bison](https://www.gnu.org/software/bison/) - Our parser generator of choice
* [Rust](https://www.rust-lang.org/en-US/) - Our language of choice for all other phases of the compilation process

## Contributing

* Paul-Andre Henegar
* James Brace
* Youri Tamitegama

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/your/project/tags). 

## License

This project is licensed under the MIT License.

## Acknowledgments

* Alexander Krolik
* McGill University
* Amir Bawab
* Several other people from the facebook group

