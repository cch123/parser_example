package main

import (
	"fmt"
	"io/ioutil"

	gopeg "github.com/yhirose/go-peg"
)

func main() {
	contentBytes, err := ioutil.ReadFile("./grammar.peg")
	if err != nil {
		fmt.Println(err)
		return
	}
	grammar := string(contentBytes)
	parser, err := gopeg.NewParser(grammar)
	if err != nil {
		fmt.Println(err)
		return
	}

	// 之后就是和其它 parser 一样的树遍历了
	err = parser.EnableAst()
	if err != nil {
		fmt.Println(err)
		return
	}

	ast, err := parser.ParseAndGetAst("(a=1)", nil)
	if err != nil {
		fmt.Println("ff", err)
		return
	}
	traverseAST(ast)
}

func traverseAST(ast *gopeg.Ast) {
	println(ast.Name)
	switch ast.Name {
	case "EXPR":
		// fold
		println(ast.Nodes[0].Name)
		traverseAST(ast.Nodes[0])
	case "ATOM":
		// go deeper
		// (FIELD OP VALUE) / '(' EXPR ')'
		// return
		println(ast.Nodes[0].Name)
		inner := ast.Nodes[0]
		switch inner.Name {
		case "EXPR":
			traverseAST(inner)
		case "COMP":
			println(inner.Nodes[0].Name)
			println(inner.Nodes[1].Name)
			println(inner.Nodes[2].Name)
		}
	default:
		println("fuck")
	}
}
