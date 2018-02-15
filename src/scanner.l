%{
#include "parser.tab.h"

int last_token = -1;

#define RETURN(a) last_token = a; return a;

int needsSemicolon() {
  return a == tIDENTIFIER
    || a == tINTVAL || a == tFLOATVAL || a == tRUNEVAL || a == tSTRINGVAL
    || a == tBREAK || a == tCONTINUE || a == tFALLTHROUGH || a == tRETURN
    || a == tINC || a == tDEC
    || a == ')' || a == ']' || a == '}';
}

// Stores the line number with each token sent to bison
#define YY_USER_ACTION yylloc.first_line = yylloc.last_line = yylineno;
%}

STRING_CHAR [ a-zA-Z0-9~@#$%^&*\-+/`<>=_|'.,;:!?{}[\]()]
ESCAPE_SEQUENCE [abfnrtv"\\]

%option yylineno

%%
[\r\n]+
[ \t]+
"//".*$

"+"|"-"|"*"|"/"|"%"|"&"|"|"|"^"|"<"|">"|"="|"!"|"("|")"|"["|"]"|"{"|"}"|","|"."|";"|":"{
  RETURN(yytext[0]);
}

"==" return tEQ;
"!=" return tNEQ;
"&&" return tAND;
"||" return tOR;

"var" return tVAR;
"float" return tFLOAT;
"int" return tINT;
"string" return tSTRING;

"while" return tWHILE;
"if" return tIF;
"else" return tELSE;
"print" return tPRINT;
"read" return tREAD;

"boolean" return tBOOL;
"TRUE" return tTRUE;
"FALSE" return tFALSE;


[a-zA-Z_]+[a-zA-Z_0-9]* {
  yylval.text = strdup(yytext);
  return tIDENT;
}

0|[1-9][0-9]* {
  yylval.text = strdup(yytext);
  return tINTVAL;
}

(0|[1-9][0-9]*)\.[0-9]+ {
  yylval.text = strdup(yytext);
  return tFLOATVAL;
}

\"({STRING_CHAR}|\\{ESCAPE_SEQUENCE})*\" {
  yylval.text = strdup(yytext);
  return tSTRINGVAL;
}


. {
  fprintf(stderr, "Error: (line %d) unexpected character `%s`\n", yylineno, yytext);
  exit(1);
}

%%
