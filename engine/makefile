# Specifies the executable
OBJ_NAME = valkyrie.exe

# Specifies the objects to compile as part of our project
OBJS = main.c

# Compiler
CC = gcc

# Cleanup
RM = rm -f

#COMPILER_FLAGS specifies the additional compilation options we're using
# -w suppresses all warnings
# -Wl,-subsystem,windows gets rid of the console window
COMPILER_FLAGS = -g -w -Wl,-subsystem,windows

# Include paths
INCLUDE_PATHS = -IC:\dev\valkyrie\engine\deps\SDL2_x86_64-w64-mingw32\include\SDL2

# Lib paths
LIBRARY_PATHS = -LC:\dev\valkyrie\engine\deps\SDL2_x86_64-w64-mingw32\lib

# Linker flags
LINKER_FLAGS = -lSDL2main -lSDL2


default: all

# The command to create the target must begin with a tab or Make will reject it.
all: $(OBJS)
	$(CC) -o  $(OBJ_NAME) $(OBJS)  $(INCLUDE_PATHS) $(LIBRARY_PATHS) $(COMPILER_FLAGS) $(LINKER_FLAGS) 

clean veryclean:
	$(RM) $(OBJ_NAME) SDL2.dll