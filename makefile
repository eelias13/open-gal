# -------------------------------------------------------- general --------------------------------------------------------

# g++ settings
CC = g++
CFLAGS = -c -Werror
LDFLAGS =

.cpp.o:
	$(CC) $(CFLAGS) $< -o $@

# -------------------------------------------------------- compiler --------------------------------------------------------

# files for compiler
COMPILER_MAIN = Compiler.cpp
SHARED = Shared/Utility.cpp Shared/API.cpp Shared/Validate.cpp
PARSER = Parser/Lexer.cpp Parser/Parser.cpp Parser/Error.cpp Parser/FunctionParser.cpp Parser/TableParser.cpp
TRANSLATOR = Translator/DNF.cpp Translator/Fuses.cpp Translator/Helper.cpp Translator/Serialization.cpp  Translator/Translator.cpp  Translator/Configs.cpp

# all object for compiler
OBJECTS= $(COMPILER_MAIN:.cpp=.o) $(SHARED:.cpp=.o) $(PARSER:.cpp=.o)  $(TRANSLATOR:.cpp=.o)

# name of compiler executable
EXECUTABLE = EasyGAL 

# g++ compiler instruction 
$(EXECUTABLE): $(OBJECTS)
	$(CC) $(LDFLAGS) $(OBJECTS) -o $@

# build instruction for compiler
compiler: parser translator shared compiler_main
compiler_main: $(COMPILER_MAIN) $(COMPILER_EXECUTABLE)
shared: $(SHARED) $(COMPILER_EXECUTABLE)
parser: $(PARSER) $(COMPILER_EXECUTABLE)
translator: $(TRANSLATOR) $(COMPILER_EXECUTABLE)

# -------------------------------------------------------- clean --------------------------------------------------------

clean:
	rm $(COMPILER_OBJECTS)
	rm $(SIMULATOR_OBJECTS)