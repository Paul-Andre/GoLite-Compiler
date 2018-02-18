cmake_minimum_required(VERSION 3.9)
project(2018_group04)

set(CMAKE_CXX_STANDARD 11)

add_executable(2018_group04
        src/main.c
        src/Makefile
        src/parser.y
        src/scanner.l
        build.sh
        run.sh
        test.sh)
