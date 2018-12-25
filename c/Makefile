PROJECT=mx

# CC=clang
CC = /usr/local/opt/llvm/bin/clang
CFLAGS=-O2 -Wall -Wextra -fpic -fopenmp -ffast-math -msse2
LDFLAGS=-shared -s -lomp

OBJS=$(PROJECT).o
TARGET=lib$(PROJECT).so

all: $(OBJS)
	$(CC) $(OBJS) $(LDFLAGS) -o $(TARGET)

clean:
	rm -f $(OBJS) $(TARGET)	
