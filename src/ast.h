
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
DECLARE_VEC(expr, ExpressionNode);

typedef struct String String;
DECLARE_VEC(string, String);

String *make_string(char*);

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

ExpressionNode *make_identifier_expression(uint32_t, char*);
ExpressionNode *make_literal_expression(uint32_t, char*,enum BasicKind);
ExpressionNode *make_append_expression(uint32_t, ExpressionNode*, ExpressionNode*);
ExpressionNode *make_binary_operation_expression(uint32_t, enum BinaryOperator, ExpressionNode*, ExpressionNode*);
ExpressionNode *make_unary_operation_expression(uint32_t, enum UnaryOperator, ExpressionNode*);
ExpressionNode *make_selector_expression(uint32_t, ExpressionNode*, char *);
ExpressionNode *make_index_expression(uint32_t, ExpressionNode*, ExpressionNode*);
ExpressionNode *make_function_call_expression(uint32_t, ExpressionNode*, ExpressionNodeVec*);


