CC = gcc

CFLAGS = -Wall

main: lex.yy.c
	$(CC) main.c -o mini -lfl

parser.tab.c parser.tab.h: mini.y
	bison -v --output=parser.tab.h mini.y

lex.yy.c: mini.l parser.tab.h
	flex mini.l
clean:
	rm parser.tab.* lex.yy.*; rm *.o
