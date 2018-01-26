%{
#include <stdio.h>
%}
DIGIT [0-9]
// Is tab included?
STRING_CHAR [ a-zA-Z0-9~@#$%^&*\-+/`<>=_|'.,;:!?{}[\]()]
ESCAPE_SEQUENCE [abfnrtv"\\]
STRING_ELEMENT ({STRING_CHAR}|\\ESCAPE_SEQUENCE)

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
")" return ')';

0|[1-9][0-9]* {
  return 1200;
}

0[0-9]+ {
  fprintf(stderr, "Error: (line %d) integer literal with leading zero `%s`\n", yylineno, yytext);
  exit(1);
}


(0|[1-9][0-9]*)\.[0-9]+ {
  return 1201;
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
  return 1202;
}

[0-9]+[a-zA-Z_]+[a-zA-Z_0-9]* {
  fprintf(stderr, "Error: (line %d) identifier starts with a digit `%s`\n", yylineno, yytext);
  exit(1);
}


\"{STRING_ELEMENT}*\" {
  return 213412;
}

\"{STRING_ELEMENT}*(\r|\n|<<EOF>>) {
  fprintf(stderr, "Error: (line %d) unclosed string\n", yylineno);
  exit(1);
}

\"{STRING_ELEMENT}*\\ {
  fprintf(stderr, "Error: (line %d) unrecognized espace sequence in string\n", yylineno);
  exit(1);
}

\"{STRING_ELEMENT}*. {
  fprintf(stderr, "Error: (line %d) unrecognized character in string\n", yylineno);
  exit(1);
}

. {
  fprintf(stderr, "Error: (line %d) unexpected character `%s`\n", yylineno, yytext);
  exit(1);
}



%%
int main() {
  int y;
  while(y = yylex()) {
    printf("%d\n", y); return 0;
  }
}

