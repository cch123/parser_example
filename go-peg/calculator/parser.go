package main

import (
	"fmt"
	"io/ioutil"

	gopeg "github.com/yhirose/go-peg"
)

/*
func setUpGrammer() map[string]*gopeg.Rule {
	g := map[string]*gopeg.Rule{}
	return g
}
*/

func main() {
	contentBytes, err := ioutil.ReadFile("./grammar.peg")
	if err != nil {
		fmt.Println(err)
		return
	}
	parser, err := gopeg.NewParser(string(contentBytes))
	if err != nil {
		fmt.Println(err)
		return
	}

	// 如果我们不需要控制 ast 生成过程
	// 直接使用 EnableAst 就可以了
	/*
		+ EXPR
		  + EXPR
		    + ATOM
		      - NUMBER ("1")
		    - BINOP ("+")
		    + ATOM
		      - NUMBER ("2")
		  - BINOP ("+")
		  + EXPR
		    + ATOM
		      - NUMBER ("4")
		    - BINOP ("*")
		    + ATOM
		      - NUMBER ("5")
	*/
	// 之后就是和其它 parser 一样的树遍历了
	err = parser.EnableAst()
	if err != nil {
		fmt.Println(err)
		return
	}

	ast, err := parser.ParseAndGetAst("1+2+4*5", nil)
	if err != nil {
		fmt.Println(err)
		return
	}
	fmt.Println(ast)
}
