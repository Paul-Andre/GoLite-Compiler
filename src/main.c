#include "stdio.h"
#include "stdlib.h"
#include "lex.yy.c" // eh?

int main(int argc, char *argv[]) {
  if (argc < 2) {
    fprintf(stderr, "Requires option\n");
    return 1;
  }

  if (strcmp(argv[1], "scan") == 0) {
    while(yylex()) {}
    printf("OK\n");
  }
  else if (strcmp(argv[1], "tokens") == 0) {
    while(1) {
      int token = yylex();
      if (token == 0) break;

      switch(token) {
        case tINTVAL:
        case tFLOATVAL:
        case tSTRING:
        case tIDENT:
          printf("%s( %s )\n", yytname[yytranslate[token]], yylval.text);
          break;
        default:
          printf("%s\n", yytname[yytranslate[token]]);
          break;
      }
    }
  }
  else if (strcmp(argv[1], "parse") == 0) {
    yyparse();
    printf("OK\n");
  }
  else {
    fprintf(stderr, "Incorrect option `%s`\n", argv[1]);
    return 1;
  }

	return 0;
}
