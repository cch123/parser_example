package main

import (
	"fmt"

	gopeg "github.com/yhirose/go-peg"
)

/*
func setUpGrammer() map[string]*gopeg.Rule {
	g := map[string]*gopeg.Rule{}
	return g
}
*/

func main() {
	grammar := `
EXPR         ← ATOM ((AND/OR) ATOM)*
ATOM         ← (FIELD OP VALUE) / '(' EXPR ')'
FIELD       ←  < [a-z]+ >
AND         ← 'and'
OR         ← 'or'
OP           ← EQ / NEQ / GT
EQ           ← '='
NEQ          ← '!='
GT           ← '>'
VALUE        ←  STRING_LIT / NUM_LIT
NUM_LIT      ←  < [0-9]+ >
STRING_LIT   ←  < [a-z]+ >
%whitespace  ←  [ \t\r\n]*
	`
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

	ast, err := parser.ParseAndGetAst("a = 1 and b != 2", nil)
	if err != nil {
		fmt.Println(err)
		return
	}
	fmt.Println(ast, err)
}
