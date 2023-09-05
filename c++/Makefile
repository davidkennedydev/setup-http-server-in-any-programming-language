CXX = g++
PACKAGE_NAME = cpp-httplib
HEADERS := $(shell spack location -i $(PACKAGE_NAME))/include
CXXFLAGS += -I $(HEADERS)

all: server

server: ./main.cpp
	$(CXX) $^ $(CXXFLAGS) -o $@

clean:
	rm -rf server

