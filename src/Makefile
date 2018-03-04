CC = gcc -fPIC -g

CFLAGS = -Wall

libbisonparser.a: lexer_parser/lex.yy.c lexer_parser/bisonparser.c
	$(CC) $(CFLAGS) -c lexer_parser/bisonparser.c -o bisonparser.o -lfl
	ar rcs libbisonparser.a bisonparser.o

parser.tab.c parser.tab.h: lexer_parser/parser.y
	bison -v --output=parser.tab.h lexer_parser/parser.y

lex.yy.c: lexer_parser/scanner.l parser.tab.h
	flex lexer_parser/scanner.l

clean:
	-rm parser.tab.* lex.yy.*;
	-rm *.o;
	-rm bisonparser.o libbisonparser.a;
	-rm parser.output;
