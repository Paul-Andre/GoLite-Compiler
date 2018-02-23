
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
