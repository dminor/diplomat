.DEFAULT_GOAL := test
.PHONY: build test

ALL_HEADERS := $(wildcard *.h) $(wildcard *.hpp) $(wildcard tests/*.hpp)
ALL_RUST := $(wildcard ../src/*.rs)

CXX=g++

$(ALL_RUST):

$(ALL_HEADERS):


../../target/debug/libexample.a: $(ALL_RUST)
	cargo build

./tests/fixeddecimal.out: ../../target/debug/libexample.a $(ALL_HEADERS) ./tests/fixeddecimal.cpp
	$(CXX) -std=c++20 ./tests/fixeddecimal.cpp ../../target/debug/libexample.a -ldl -lpthread -lm -g -o ./tests/fixeddecimal.out

./tests/structs.out: ../../target/debug/libexample.a $(ALL_HEADERS) ./tests/structs.cpp
	$(CXX) -std=c++20 ./tests/structs.cpp ../../target/debug/libexample.a -ldl -lpthread -lm -g -o ./tests/structs.out


test: ./tests/fixeddecimal.out ./tests/structs.out
	./tests/fixeddecimal.out
	./tests/structs.out
