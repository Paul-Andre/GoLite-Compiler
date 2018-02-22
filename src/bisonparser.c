#include "stdio.h"
#include "stdlib.h"
#include "lex.yy.c" // eh?

typedef struct Program Program;

Program *root;

Program *make_program(int32_t a);



void say_hello() {
	printf("hello\n");
}

void scan() {
	while(yylex()) {}
}

void print_tokens() {
	while(1) {
		int token = yylex();
		if (token == 0) break;

		switch(token) {
			case tINTVAL:
			case tFLOATVAL:
			case tRUNEVAL:
			case tSTRINGVAL:
			case tIDENTIFIER:
				printf("%s( %s )\n", yytname[yytranslate[token]], yylval.text);
				break;
			default:
				printf("%s\n", yytname[yytranslate[token]]);
				break;
		}
	}
}

Program *parse() {
	yyparse();
	return make_program(12);
	//return root;
}
