PROJECT=mx

CC=/usr/local/opt/llvm/bin/clang
CFLAGS=-O2 -Wall -Wextra -fpic -fopenmp -ffast-math
LDFLAGS=-shared -s -lomp

OBJS=$(PROJECT).o
TARGET=lib$(PROJECT).so

all: $(OBJS)
	$(CC) $(OBJS) $(LDFLAGS) -o $(TARGET)
	echo "# cp $(TARGET) ../"

clean:
	rm -f $(OBJS) $(TARGET)	


/usr/local/opt/llvm/bin/clang -I/usr/local/opt/llvm/include -fopenmp -ffast-math -c mx.c
