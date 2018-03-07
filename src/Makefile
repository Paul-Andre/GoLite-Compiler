CC = gcc -fPIC -g

CFLAGS = -Wall

libbisonparser.a: lex.yy.c bisonparser.c
	$(CC) $(CFLAGS) -c bisonparser.c -o bisonparser.o -ll
	ar rcs libbisonparser.a bisonparser.o

parser.tab.c parser.tab.h: parser.y
	bison -v --output=parser.tab.h parser.y

lex.yy.c: scanner.l parser.tab.h
	flex scanner.l

clean:
	-rm parser.tab.* lex.yy.*;
	-rm *.o;
	-rm bisonparser.o libbisonparser.a;
	-rm parser.output;
