
/* This macro is meant to generate the boilerplate to manipulate rust vectors.
 * It generates a {TYPE_NAME}Vec type and functions to create a vector and push to it.
 *
 * The vectors are assumed to contain types that are otherwise manipulated
 * through opaque pointers.
 *
 * For example the first invocation of the macro produces the following code:
 
 typedef struct ExpressionNodeVec ExpressionNodeVec;
 ExpressionNodeVec * make_expr_vec (void);
 void expr_vec_push (ExpressionNodeVec *, ExpressionNode *);

*/

#define DECLARE_VEC(NAME, TYPE) \
  typedef struct TYPE##Vec TYPE##Vec;\
  TYPE##Vec * make_##NAME##_vec (void);\
  void NAME##_vec_push (TYPE##Vec *, TYPE *);\


typedef struct ExpressionNode ExpressionNode;
DECLARE_VEC(expression, ExpressionNode);

typedef struct VarSpec VarSpec;
DECLARE_VEC(var_spec, VarSpec);

typedef struct TypeSpec TypeSpec;
DECLARE_VEC(type_spec, VarTypeSpec);

typedef struct String String;
DECLARE_VEC(string, String);

String *make_string(char*);

typedef struct StatementNode StatementNode;
DECLARE_VEC(statement, StatementNode);

typedef struct AstKindNode AstKindNode;

typedef struct Field Field;
DECLARE_VEC(field, Field);

typedef struct CaseClause CaseClause;
DECLARE_VEC(case_clause, CaseClause);

enum BasicKind {
  kInt = 0,
  kFloat = 1,
  kRune = 2,
  kString = 3,
};

enum BinaryOperator {
    opOr,
    opAnd,

    opEq,
    opNeq,
    opLt,
    opLeq,
    opGt,
    opGeq,

    opAdd,
    opSub,
    opBwOr,
    opBwXor,

    opMul,
    opDiv,
    opMod,
    opLShift,
    opRShift,
    opBwAnd,
    opBwAndNot,
};

enum UnaryOperator {
    opPlus,
    opNeg,
    opBwCompl,
    opNot,
};

Program *make_program(char*, TopLevelDeclarationVec*);

TopLevelDeclarationNode *make_var_top_level_declaration(uint32_t, VarSpecVec*);
TopLevelDeclarationNode *make_type_top_level_declaration(uint32_t, TypeSpecVec*);
TopLevelDeclarationNode *make_function_top_level_declaration(
    uint32_t, char*, FieldVec*, AstKindNode*, StatementNodeVec*);

ExpressionNode *make_identifier_expression(uint32_t, char*);
ExpressionNode *make_literal_expression(uint32_t, char*,enum BasicKind);
ExpressionNode *make_append_expression(uint32_t, ExpressionNode*, ExpressionNode*);
ExpressionNode *make_binary_operation_expression(uint32_t, enum BinaryOperator, ExpressionNode*, ExpressionNode*);
ExpressionNode *make_unary_operation_expression(uint32_t, enum UnaryOperator, ExpressionNode*);
ExpressionNode *make_selector_expression(uint32_t, ExpressionNode*, char *);
ExpressionNode *make_index_expression(uint32_t, ExpressionNode*, ExpressionNode*);
ExpressionNode *make_function_call_expression(uint32_t, ExpressionNode*, ExpressionNodeVec*);

StatementNode *make_empty_statement(uint32_t);
StatementNode *make_block_statement(uint32_t, StatementNodeVec*);
StatementNode *make_expression_statement(uint32_t, ExpressionNode*);
StatementNode *make_assignment_statement(uint32_t, ExpressionNodeVec*, ExpressionNodeVec*);
StatementNode *make_op_assignment_statement(uint32_t, ExpressionNode*, ExpressionNode*);
StatementNode *make_var_declaration_statement(uint32_t, VarSpecVec*);
StatementNode *make_type_declaration_statement(uint32_t, TypeSpecVec*);
StatementNode *make_short_var_declaration_statement(uint32_t, StringVec*, ExpressionNodeVec*);
StatementNode *make_inc_dec_statement(uint32_t, int, ExpressionNode*);
StatementNode *make_print_statement(uint32_t, ExpressionNodeVec*);
StatementNode *make_println_statement(uint32_t, ExpressionNodeVec*);
StatementNode *make_if_statement(uint32_t, StatementNode*, ExpressionNode*, StatementNodeVec*, StatementNode*);
StatementNode *make_loop_statement(uint32_t, StatementNodeVec*);
StatementNode *make_while_statement(uint32_t, ExpressionNode*, StatementNodeVec*);
StatementNode *make_for_statement(uint32_t, StatementNode*, ExpressionNode*, StatementNode*, StatementNodeVec*);
StatementNode *make_switch_statement(uint32_t, StatementNode*, ExpressionNode*, CaseClauseVec*);
StatementNode *make_break_statement(uint32_t);
StatementNode *make_continue_statement(uint32_t);
StatementNode *make_return_statement(uint32_t, ExpressionNode*);

AstKindNode *make_identifier_kind(uint32_t, char*);
AstKindNode *make_slice_kind(uint32_t, AstKindNode*);
AstKindNode *make_array_kind(uint32_t, AstKindNode*, char*);
AstKindNode *make_struct_kind(uint32_t, FieldVec*);

Field *make_field(uint32_t, StringVec*, AstKindNode*);

CaseClause *make_case_clause(uint32_t, ExpressionNodeVec*, StatementNodeVec*);
VarSpec *make_var_spec(uint32_t, StringVec*, AstKindNode*, ExpressionNodeVec*);
TypeSpec *make_type_spec(uint32_t, StringVec*, AstKindNode*);

