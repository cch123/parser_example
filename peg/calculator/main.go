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

	calc = &Calculator{
		Buffer: "1+2",
	}
	calc.Init()
	calc.Parse()
	calc.PrettyPrintSyntaxTree(calc.Buffer)
	calc.AST()
}
