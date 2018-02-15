CC = gcc

CFLAGS = -Wall

golitec: lex.yy.c
	$(CC) main.c -o golitec -lfl

parser.tab.c parser.tab.h: parser.y
	bison -v --output=parser.tab.h parser.y

lex.yy.c: scanner.l parser.tab.h
	flex scanner.l
clean:
	-rm parser.tab.* lex.yy.*;
	-rm *.o;
	-rm golitec;
	-rm parser.output;
