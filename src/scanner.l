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

"break" return tBREAK;
"case" return tCASE;
"chan" return tCHAN;
"const" return tCONST;
"continue" return tCONTINUE;

"default" return tDEFAULT;
"defer" return tDEFER;
"else" return tELSE;
"fallthrough" return tFALLTHROUGH;
"for" return tFOR;

"func" return tFUNC;
"go" return tGO;
"goto" return tGOTO;
"if" return tIF;
"import" return tIMPORT;

"interface" return tINTERFACE;
"map" return tMAP;
"package" return tPACKAGE;
"range" return tRANGE;
"return" return tRETURN;

"select" return tSELECT;
"struct" return tSTRUCT;
"switch" return tSWITCH;
"type" return tTYPE;
"var" return tVAR;

"print" return tPRINT;
"println" return tPRINTLN;
"append" return tAPPEND;


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
