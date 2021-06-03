# g++ settings
CC = g++
CFLAGS = -c -Werror
LDFLAGS =

# name of compiler executable
EXECUTABLE = OpenGal 

# files for compiler
COMPILER_MAIN = Compiler.cpp
SHARED = Shared/ConvertRust.cpp Shared/Utility.cpp Shared/API.cpp Shared/Validate.cpp
TRANSLATOR = Translator/DNF.cpp Translator/Fuses.cpp Translator/Helper.cpp Translator/Serialization.cpp  Translator/Translator.cpp  Translator/Configs.cpp

ifeq ($(shell uname),Darwin)
    EXT := dylib
else
    EXT := so
endif

# all object for compiler
OBJECTS= $(SHARED:.cpp=.o)  $(TRANSLATOR:.cpp=.o) $(COMPILER_MAIN:.cpp=.o)

all: target/debug/libparser.$(EXT)
# I know not pretty but int works
	make obj 
	$(CC) $(OBJECTS) ./target/debug/libparser.$(EXT)  -o $(EXECUTABLE)

target/debug/libparser.$(EXT): src/lib.rs Cargo.toml
	cbindgen . -o src/open_gal.h
	cargo build

obj: $(OBJECTS)

.cpp.o:
	$(CC) $(CFLAGS) $< -o $@

clean:
	rm $(OBJECTS)
	rm -rf target