CC = gcc -fPIC -g

CFLAGS = -Wall

libbisonparser.a: parser+lexer/lex.yy.c parser/bisonparser.c
	$(CC) $(CFLAGS) -c parser/bisonparser.c -o bisonparser.o -lfl
	ar rcs libbisonparser.a bisonparser.o

parser.tab.c parser.tab.h: parser.y
	bison -v --output=parser.tab.h parser.y

lex.yy.c: scanner.l parser.tab.h
	flex parser+lexer/scanner.l

clean:
	-rm parser.tab.* lex.yy.*;
	-rm *.o;
	-rm bisonparser.o libbisonparser.a;
	-rm parser.output;
