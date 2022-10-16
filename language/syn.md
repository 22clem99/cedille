inst
    | expr '.'
    | '.'
    | list_expr '!'
    | if_then_else
    | 'faire' ':' '('
    | list_decl_var '.'

list_decl_var
    | type decl_var (';' decl_var)*

decl_var
    | ident ('égale' expr)?

list_expr 
    | ( expr
        ( ';' expr) ∗ ) ?

inst_block 
    | '-'* inst

if_then_else
    | 'si' expr 'alors'
      inst_block
      ('sinon' 'si' expr 'alors'
      inst_block)*
      ('sinon'
      list_inst)?

expr
    | assign_expr


assign_expr
    | or_expr (
      {condition: expression must be a "lvalue"}
      'égale' assign_expr
    | ε )

or_expr
    | and_expr
    | or_expr 'ou' and_expr

and_expr
    | eq_neq_expr
    | and_expr 'et' eq_neq_expr

eq_neq_expr
    | inequality_expr
    | eq_neq_expr 'non' 'égale' inequality_expr
    | eq_neq_expr 'différent' inequality_expr

inequality_expr
    | sum_expr
    | inequality_expr 'plus' 'petit' 'ou 'égale' sum_expr
    | inequality_expr 'plus' 'grand' 'ou 'égale' sum_expr
    | inequality_expr 'plus' 'grand' sum_expr
    | inequality_expr 'plus' 'petit' sum_expr

sum_expr
    | mult_expr
    | sum_expr 'plus' mult_expr
    | sum_expr 'moins' mult_expr

mult_expr
    | unary_expr
    | mult_expr 'fois' unary_expr
    | mult_expr 'divisé' unary_expr
    | mult_expr 'modulo' unary_expr

unary_expr
    | 'moins' unary_expr
    | 'non' unary_expr
    | primary_expr

primary_expr
    | ident
    | ident '( ' list_expr ') '
    | '( ' expr ') '
    | literal

type
    | INT
    | FLOAT
    | STRING

literal
    | type
    | 'vrai'
    | 'faux'

ident
    | IDENT
