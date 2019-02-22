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

	ast, err := parser.ParseAndGetAst("a= 1 and b != 2 and c = 3", nil)
	if err != nil {
		fmt.Println(err)
		return
	}
	fmt.Println(ast, err)
}
