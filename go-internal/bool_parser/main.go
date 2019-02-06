package main

import (
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
)

func main() {
	expr := `a == 1 && b == 2 && in_array(c, int_arr)`
	fset := token.NewFileSet()
	exprAst, err := parser.ParseExpr(expr)
	if err != nil {
		fmt.Println(err)
		return
	}

	ast.Print(fset, exprAst)
}
