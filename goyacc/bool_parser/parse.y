%{
package main

type Expression interface{}

type CompExpr struct {
    field string
    op string
    value string
}

type LogicExpr struct {
    left Expression
    op string
    right Expression
}
%}

%union{
    token string
    expr  Expression
    comp_expr Expression
}

%type<expr> program
%type<expr> expr

%type<comp_expr> comp_expr

%type<token> bin_op

%token<token> FIELD
%token<token>  AND OR

//%token<bin_op> NEQ GTE LTE

// 下面的 NEQ 和 lexer 中返回的 token 应该是对应的
// 有运算符优先级的定义的话
// 似乎也不需要在上面的 token 进行定义了
%token NEQ EQ GTE GT LTE LT
%left AND OR

%%

program
    : expr
    {
        $$ = $1 // 会把 $1 当成返回值赋值给 return val
        yylex.(*Lexer).ast= $$
    }

bin_op
    : NEQ { $$ = "!="}
    | EQ { $$ = "=" }
    | GTE { $$ = ">="}
    | LTE { $$ = "<="}
    | GT { $$ = ">"}
    | LT { $$ = "<"}

comp_expr
    : FIELD bin_op FIELD
    {
        $$ = CompExpr{field: $1, op: $2, value: $3}
    }

expr
    : expr AND expr
    {
        $$ = LogicExpr{left: $1, op: $2, right: $3}
    }
    | expr OR expr
    {
        $$ = LogicExpr{left: $1, op: $2, right: $3}
    }
    | comp_expr
    {
        $$ = $1
    }
%%
