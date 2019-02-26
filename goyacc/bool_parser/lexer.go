package main

import (
	"fmt"
	"os"
	"strings"
	"text/scanner"
)

// Lexer scanner 和 ast 的结合体
type Lexer struct {
	scanner.Scanner
	ast Expression
}

// Lex 词法分析器
func (l *Lexer) Lex(lval *yySymType) int {
	var token rune
	l.Scan()
	// 这里需要额外处理一些多字符的 token 的情况
	// 比如 ! 开头的
	// 比如 a in [1,2,3,4 这种的]
	// 比如 a is null 这种的
	// >= <= 等等
	//fmt.Printf("%#v\n", l.TokenText())
	switch l.TokenText() {
	case "!":
		token = NEQ
		l.Scan()
		lval.token = "!="
	case ">":
		op := ">"
		token = GT
		if l.Peek() == '=' {
			l.Scan()
			token = GTE
			op = ">="
		}
		//fmt.Println("after >", l.TokenText())
		lval.token = op
	case "<":
		op := "<"
		token = LT
		if l.Peek() == '=' {
			l.Scan()
			token = LTE
			op = "<="
		}
		lval.token = op
	case "=":
		token = EQ
		lval.token = "="
	case "and", "AND":
		token = AND
		lval.token = "and"
	case "or", "OR":
		token = OR
		lval.token = "or"
	case "":
		// do nothing
		// correct ?
	default:
		token = FIELD
		lval.token = l.TokenText()
	}

	//println(lval.token, token)
	return int(token)
}

func (l *Lexer) Error(e string) {
	panic(e)
}

func main() {
	l := new(Lexer)
	l.Init(strings.NewReader(os.Args[1]))
	yyParse(l)
	fmt.Printf("%#v\n", l.ast)
}
