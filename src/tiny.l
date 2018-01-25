%{
#include <stdio.h>
%}
DIGIT [0-9]
%option yylineno

%%
[\r\n]+
[ \t]+

"+" return '+';
"-" return '-';
"*" return '*';
"/" return '/';
"=" return '=';
"(" return '(';
")" return '(';

"==" return tASSIGN;

0|[1-9][0-9]* {
  return tINT;
}

0[0-9]+ {
  fprintf(stderr, "Error: (line %d) integer literal with leading zero `%s`\n", yylineno, yytext);
  exit(1);
}


(0|[1-9][0-9]*)\.[0-9]+ {
  return tFLOAT_LIT;
}

[0-9]+\. {
  fprintf(stderr, "Error: (line %d) floating point literal doesn't have digits after the point `%s`\n", yylineno, yytext);
  exit(1);
}

\.[0-9]+ {
  fprintf(stderr, "Error: (line %d) floating point literal doesn't have digits before the point `%s`\n", yylineno, yytext);
  exit(1);
}

0[0-9]+\.[0-9]+ {
  fprintf(stderr, "Error: (line %d) floating point literal with leading zero `%s`\n", yylineno, yytext);
  exit(1);
}



[a-zA-Z_]+[a-zA-Z_0-9]* {
  return tIDENT;
}

[0-9]+[a-zA-Z_]+[a-zA-Z_0-9]* {
  fprintf(stderr, "Error: (line %d) identifier starts with a digit `%s`\n", yylineno, yytext);
  exit(1);
}


.       {
  fprintf(stderr, "Error: (line %d) unexpected character `%s`\n", yylineno, yytext);
  exit(1);
}

%%
int main() { printf("%d\n", yylex ()); return 0; }
