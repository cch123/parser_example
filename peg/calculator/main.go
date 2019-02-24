package main

import "fmt"

func main() {
	calc := &Calculator{
		Buffer: "1 +2 * 5",
	}
	calc.Init()
	err := calc.Parse()
	if err != nil {
		fmt.Println(err)
		return
	}
	calc.PrettyPrintSyntaxTree(calc.Buffer)

	println()
	//ast := calc.AST()
	begin, end := 0, 0
	for _, tok := range calc.Tokens() {
		//fmt.Println(tok.pegRule)
		switch tok.pegRule {
		//case ruleadd:
		//fmt.Println(tok.String())
		//case rulemultiply:
		//fmt.Println(tok.String())
		case rulePegText:
			begin, end = int(tok.begin), int(tok.end)
			fmt.Println(begin, end)
			// case rulevalue:
			//fmt.Println(calc.Buffer[tok.begin:tok.end], "ff")
		case rulews:
			// do nothing
		default:
			fmt.Println("ffff", rul3s[tok.pegRule])
			fmt.Println(tok.String())
		}
	}
}
