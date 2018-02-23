
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

ExpressionNode *expr_identifier(uint32_t, char*);
ExpressionNode *expr_literal(uint32_t, char*,enum BasicKind);
ExpressionNode *expr_append(uint32_t, ExpressionNode*, ExpressionNode*);


