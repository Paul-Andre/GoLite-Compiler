%{
#include "parser.tab.h"

int last_token = -1;

#define RETURN(a) {last_token = a; return a;}

int needsSemicolon() {
  RETURN(a == tIDENTIFIER
    || a == tINTVAL || a == tFLOATVAL || a == tRUNEVAL || a == tSTRINGVAL
    || a == tBREAK || a == tCONTINUE || a == tFALLTHROUGH || a == tRETURN
    || a == tINC || a == tDEC
    || a == ')' || a == ']' || a == '}';
}

// Stores the line number with each token sent to bison
#define YY_USER_ACTION yylloc.first_line = yylloc.last_line = yylineno;
%}

STRING_CHAR [ a-zA-Z0-9~@#$%^&*\-+/`<>=_|'.,;:!?{}[\]()]
RUNE_ESCAPE_SEQUENCE [abfnrtv'\\]
STRING_ESCAPE_SEQUENCE [abfnrtv"\\]

%option yylineno

%%
[\r\n]+
[ \t]+
"//".*$

"+"|"-"|"*"|"/"|"%"|"&"|"|"|"^"|"<"|">"|"="|"!"|"("|")"|"["|"]"|"{"|"}"|","|"."|";"|":"{
  RETURN(yytext[0]);
}

"==" RETURN(tEQ);
"!=" RETURN(tNEQ);
"&&" RETURN(tAND);
"||" RETURN(tOR);

"break" RETURN(tBREAK);
"case" RETURN(tCASE);
"chan" RETURN(tCHAN);
"const" RETURN(tCONST);
"continue" RETURN(tCONTINUE);

"default" RETURN(tDEFAULT);
"defer" RETURN(tDEFER);
"else" RETURN(tELSE);
"fallthrough" RETURN(tFALLTHROUGH);
"for" RETURN(tFOR);

"func" RETURN(tFUNC);
"go" RETURN(tGO);
"goto" RETURN(tGOTO);
"if" RETURN(tIF);
"import" RETURN(tIMPORT);

"interface" RETURN(tINTERFACE);
"map" RETURN(tMAP);
"package" RETURN(tPACKAGE);
"range" RETURN(tRANGE);
"return" RETURN(tRETURN);

"select" RETURN(tSELECT);
"struct" RETURN(tSTRUCT);
"switch" RETURN(tSWITCH);
"type" RETURN(tTYPE);
"var" RETURN(tVAR);

"print" RETURN(tPRINT);
"println" RETURN(tPRINTLN);
"append" RETURN(tAPPEND);


[a-zA-Z_]+[a-zA-Z_0-9]* {
  yylval.text = strdup(yytext);
  RETURN(tIDENTIFIER);
}

0|[1-9][0-9]* {
  yylval.text = strdup(yytext);
  RETURN(tINTVAL);
}

0[0-7]+ {
  yylval.text = strdup(yytext);
  RETURN(tINTVAL);
}

0(x|X)[0-9A-Fa-f]+ {
  yylval.text = strdup(yytext);
  RETURN(tINTVAL);
}

[0-9]+\.[0-9]* {
  yylval.text = strdup(yytext);
  RETURN(tFLOATVAL);
}

\.[0-9]+ {
  yylval.text = strdup(yytext);
  RETURN(tFLOATVAL);
}

'[^\\\n']|\\{RUNE_ESCAPE_SEQUENCE}' {
  yylval.text = strdup(yytext);
  RETURN(tRUNE);
}

\"([^\"\\\n]|\\{STRING_ESCAPE_SEQUENCE})*\" {
  yylval.text = strdup(yytext);
  RETURN(tSTRINGVAL);
}

`[^`]*` {
  yylval.text = strdup(yytext);
  RETURN(tSTRINGVAL);
}


. {
  fprintf(stderr, "Error: (line %d) unexpected character `%s`\n", yylineno, yytext);
  exit(1);
}

%%
