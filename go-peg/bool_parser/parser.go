package main

import (
	"fmt"
	"io/ioutil"
	"strings"

	gopeg "github.com/yhirose/go-peg"
)

type andExpr struct {
	lhs interface{} // comp, expr
	rhs interface{} // comp, expr
}

type orExpr struct {
	lhs interface{} // comp, expr
	rhs interface{} // comp, expr
}

type comp struct {
	lhs string
	rhs string
	op  string
}

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

	ast, err := parser.ParseAndGetAst("(a=1 or b = 2 and c =4)", nil)
	if err != nil {
		fmt.Println("ff", err)
		return
	}
	myAST := traverseAST(ast)
	fmt.Printf("%#v\n", myAST)
}

func traverseAST(ast *gopeg.Ast) interface{} {
	switch ast.Name {
	case "EXPR":
		// fold
		initial := traverseAST(ast.Nodes[0])
		var lhs interface{} = initial
		idx := 1
		for idx < len(ast.Nodes)-1 {
			binop := ast.Nodes[idx]
			rhs := traverseAST(ast.Nodes[idx+1])
			fmt.Printf("%#v\n", binop.S)
			if strings.TrimSpace(binop.S) == "and" {
				lhs = andExpr{lhs: lhs, rhs: rhs}
			} else if strings.TrimSpace(binop.S) == "or" {
				lhs = orExpr{lhs: lhs, rhs: rhs}
			} else {
				fmt.Printf("failed %#v\n", binop.S)
				break
			}
			idx += 2
		}
		return lhs
	case "ATOM":
		// go deeper
		// (FIELD OP VALUE) / '(' EXPR ')'
		inner := ast.Nodes[0]
		switch inner.Name {
		case "EXPR":
			return traverseAST(inner)
		case "COMP":
			return comp{
				lhs: inner.Nodes[0].Token,
				op:  strings.TrimSpace(inner.Nodes[1].S), // 这里 token 取出来是空的，应该是 bug
				rhs: strings.TrimSpace(inner.Nodes[2].S), // 这里 token 取出来是空的
			}
		}
	default:
		println("fuck")
	}
	return ""
}
