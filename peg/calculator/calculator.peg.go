package main

//go:generate peg ./calculator.peg

import (
	"fmt"
	"io"
	"math"
	"os"
	"sort"
	"strconv"
)

const endSymbol rune = 1114112

/* The rule types inferred from the grammar are below. */
type pegRule uint8

const (
	ruleUnknown pegRule = iota
	ruleexpr
	rulee1
	rulevalue
	ruleop
	ruleadd
	ruleminus
	rulemultiply
	ruledivide
	ruleopen
	ruleclose
	rulesp
	rulePegText
)

var rul3s = [...]string{
	"Unknown",
	"expr",
	"e1",
	"value",
	"op",
	"add",
	"minus",
	"multiply",
	"divide",
	"open",
	"close",
	"sp",
	"PegText",
}

type token32 struct {
	pegRule
	begin, end uint32
}

func (t *token32) String() string {
	return fmt.Sprintf("\x1B[34m%v\x1B[m %v %v", rul3s[t.pegRule], t.begin, t.end)
}

type node32 struct {
	token32
	up, next *node32
}

func (node *node32) print(w io.Writer, pretty bool, buffer string) {
	var print func(node *node32, depth int)
	print = func(node *node32, depth int) {
		for node != nil {
			for c := 0; c < depth; c++ {
				fmt.Fprintf(w, " ")
			}
			rule := rul3s[node.pegRule]
			quote := strconv.Quote(string(([]rune(buffer)[node.begin:node.end])))
			if !pretty {
				fmt.Fprintf(w, "%v %v\n", rule, quote)
			} else {
				fmt.Fprintf(w, "\x1B[34m%v\x1B[m %v\n", rule, quote)
			}
			if node.up != nil {
				print(node.up, depth+1)
			}
			node = node.next
		}
	}
	print(node, 0)
}

func (node *node32) Print(w io.Writer, buffer string) {
	node.print(w, false, buffer)
}

func (node *node32) PrettyPrint(w io.Writer, buffer string) {
	node.print(w, true, buffer)
}

type tokens32 struct {
	tree []token32
}

func (t *tokens32) Trim(length uint32) {
	t.tree = t.tree[:length]
}

func (t *tokens32) Print() {
	for _, token := range t.tree {
		fmt.Println(token.String())
	}
}

func (t *tokens32) AST() *node32 {
	type element struct {
		node *node32
		down *element
	}
	tokens := t.Tokens()
	var stack *element
	for _, token := range tokens {
		if token.begin == token.end {
			continue
		}
		node := &node32{token32: token}
		for stack != nil && stack.node.begin >= token.begin && stack.node.end <= token.end {
			stack.node.next = node.up
			node.up = stack.node
			stack = stack.down
		}
		stack = &element{node: node, down: stack}
	}
	if stack != nil {
		return stack.node
	}
	return nil
}

func (t *tokens32) PrintSyntaxTree(buffer string) {
	t.AST().Print(os.Stdout, buffer)
}

func (t *tokens32) WriteSyntaxTree(w io.Writer, buffer string) {
	t.AST().Print(w, buffer)
}

func (t *tokens32) PrettyPrintSyntaxTree(buffer string) {
	t.AST().PrettyPrint(os.Stdout, buffer)
}

func (t *tokens32) Add(rule pegRule, begin, end, index uint32) {
	if tree := t.tree; int(index) >= len(tree) {
		expanded := make([]token32, 2*len(tree))
		copy(expanded, tree)
		t.tree = expanded
	}
	t.tree[index] = token32{
		pegRule: rule,
		begin:   begin,
		end:     end,
	}
}

func (t *tokens32) Tokens() []token32 {
	return t.tree
}

type Calculator struct {
	Buffer string
	buffer []rune
	rules  [13]func() bool
	parse  func(rule ...int) error
	reset  func()
	Pretty bool
	tokens32
}

func (p *Calculator) Parse(rule ...int) error {
	return p.parse(rule...)
}

func (p *Calculator) Reset() {
	p.reset()
}

type textPosition struct {
	line, symbol int
}

type textPositionMap map[int]textPosition

func translatePositions(buffer []rune, positions []int) textPositionMap {
	length, translations, j, line, symbol := len(positions), make(textPositionMap, len(positions)), 0, 1, 0
	sort.Ints(positions)

search:
	for i, c := range buffer {
		if c == '\n' {
			line, symbol = line+1, 0
		} else {
			symbol++
		}
		if i == positions[j] {
			translations[positions[j]] = textPosition{line, symbol}
			for j++; j < length; j++ {
				if i != positions[j] {
					continue search
				}
			}
			break search
		}
	}

	return translations
}

type parseError struct {
	p   *Calculator
	max token32
}

func (e *parseError) Error() string {
	tokens, err := []token32{e.max}, "\n"
	positions, p := make([]int, 2*len(tokens)), 0
	for _, token := range tokens {
		positions[p], p = int(token.begin), p+1
		positions[p], p = int(token.end), p+1
	}
	translations := translatePositions(e.p.buffer, positions)
	format := "parse error near %v (line %v symbol %v - line %v symbol %v):\n%v\n"
	if e.p.Pretty {
		format = "parse error near \x1B[34m%v\x1B[m (line %v symbol %v - line %v symbol %v):\n%v\n"
	}
	for _, token := range tokens {
		begin, end := int(token.begin), int(token.end)
		err += fmt.Sprintf(format,
			rul3s[token.pegRule],
			translations[begin].line, translations[begin].symbol,
			translations[end].line, translations[end].symbol,
			strconv.Quote(string(e.p.buffer[begin:end])))
	}

	return err
}

func (p *Calculator) PrintSyntaxTree() {
	if p.Pretty {
		p.tokens32.PrettyPrintSyntaxTree(p.Buffer)
	} else {
		p.tokens32.PrintSyntaxTree(p.Buffer)
	}
}

func (p *Calculator) WriteSyntaxTree(w io.Writer) {
	p.tokens32.WriteSyntaxTree(w, p.Buffer)
}

func (p *Calculator) Init() {
	var (
		max                  token32
		position, tokenIndex uint32
		buffer               []rune
	)
	p.reset = func() {
		max = token32{}
		position, tokenIndex = 0, 0

		p.buffer = []rune(p.Buffer)
		if len(p.buffer) == 0 || p.buffer[len(p.buffer)-1] != endSymbol {
			p.buffer = append(p.buffer, endSymbol)
		}
		buffer = p.buffer
	}
	p.reset()

	_rules := p.rules
	tree := tokens32{tree: make([]token32, math.MaxInt16)}
	p.parse = func(rule ...int) error {
		r := 1
		if len(rule) > 0 {
			r = rule[0]
		}
		matches := p.rules[r]()
		p.tokens32 = tree
		if matches {
			p.Trim(tokenIndex)
			return nil
		}
		return &parseError{p, max}
	}

	add := func(rule pegRule, begin uint32) {
		tree.Add(rule, begin, position, tokenIndex)
		tokenIndex++
		if begin != position && position > max.end {
			max = token32{rule, begin, position}
		}
	}

	matchDot := func() bool {
		if buffer[position] != endSymbol {
			position++
			return true
		}
		return false
	}

	/*matchChar := func(c byte) bool {
		if buffer[position] == c {
			position++
			return true
		}
		return false
	}*/

	/*matchRange := func(lower byte, upper byte) bool {
		if c := buffer[position]; c >= lower && c <= upper {
			position++
			return true
		}
		return false
	}*/

	_rules = [...]func() bool{
		nil,
		/* 0 expr <- <(sp e1 !.)> */
		func() bool {
			position0, tokenIndex0 := position, tokenIndex
			{
				position1 := position
				if !_rules[rulesp]() {
					goto l0
				}
				if !_rules[rulee1]() {
					goto l0
				}
				{
					position2, tokenIndex2 := position, tokenIndex
					if !matchDot() {
						goto l2
					}
					goto l0
				l2:
					position, tokenIndex = position2, tokenIndex2
				}
				add(ruleexpr, position1)
			}
			return true
		l0:
			position, tokenIndex = position0, tokenIndex0
			return false
		},
		/* 1 e1 <- <(sp value (sp op value sp)*)> */
		func() bool {
			position3, tokenIndex3 := position, tokenIndex
			{
				position4 := position
				if !_rules[rulesp]() {
					goto l3
				}
				if !_rules[rulevalue]() {
					goto l3
				}
			l5:
				{
					position6, tokenIndex6 := position, tokenIndex
					if !_rules[rulesp]() {
						goto l6
					}
					if !_rules[ruleop]() {
						goto l6
					}
					if !_rules[rulevalue]() {
						goto l6
					}
					if !_rules[rulesp]() {
						goto l6
					}
					goto l5
				l6:
					position, tokenIndex = position6, tokenIndex6
				}
				add(rulee1, position4)
			}
			return true
		l3:
			position, tokenIndex = position3, tokenIndex3
			return false
		},
		/* 2 value <- <((<[0-9]+> sp) / (open e1 close))> */
		func() bool {
			position7, tokenIndex7 := position, tokenIndex
			{
				position8 := position
				{
					position9, tokenIndex9 := position, tokenIndex
					{
						position11 := position
						if c := buffer[position]; c < rune('0') || c > rune('9') {
							goto l10
						}
						position++
					l12:
						{
							position13, tokenIndex13 := position, tokenIndex
							if c := buffer[position]; c < rune('0') || c > rune('9') {
								goto l13
							}
							position++
							goto l12
						l13:
							position, tokenIndex = position13, tokenIndex13
						}
						add(rulePegText, position11)
					}
					if !_rules[rulesp]() {
						goto l10
					}
					goto l9
				l10:
					position, tokenIndex = position9, tokenIndex9
					if !_rules[ruleopen]() {
						goto l7
					}
					if !_rules[rulee1]() {
						goto l7
					}
					if !_rules[ruleclose]() {
						goto l7
					}
				}
			l9:
				add(rulevalue, position8)
			}
			return true
		l7:
			position, tokenIndex = position7, tokenIndex7
			return false
		},
		/* 3 op <- <((sp add) / minus / multiply / divide)> */
		func() bool {
			position14, tokenIndex14 := position, tokenIndex
			{
				position15 := position
				{
					position16, tokenIndex16 := position, tokenIndex
					if !_rules[rulesp]() {
						goto l17
					}
					if !_rules[ruleadd]() {
						goto l17
					}
					goto l16
				l17:
					position, tokenIndex = position16, tokenIndex16
					if !_rules[ruleminus]() {
						goto l18
					}
					goto l16
				l18:
					position, tokenIndex = position16, tokenIndex16
					if !_rules[rulemultiply]() {
						goto l19
					}
					goto l16
				l19:
					position, tokenIndex = position16, tokenIndex16
					if !_rules[ruledivide]() {
						goto l14
					}
				}
			l16:
				add(ruleop, position15)
			}
			return true
		l14:
			position, tokenIndex = position14, tokenIndex14
			return false
		},
		/* 4 add <- <('+' sp)> */
		func() bool {
			position20, tokenIndex20 := position, tokenIndex
			{
				position21 := position
				if buffer[position] != rune('+') {
					goto l20
				}
				position++
				if !_rules[rulesp]() {
					goto l20
				}
				add(ruleadd, position21)
			}
			return true
		l20:
			position, tokenIndex = position20, tokenIndex20
			return false
		},
		/* 5 minus <- <('-' sp)> */
		func() bool {
			position22, tokenIndex22 := position, tokenIndex
			{
				position23 := position
				if buffer[position] != rune('-') {
					goto l22
				}
				position++
				if !_rules[rulesp]() {
					goto l22
				}
				add(ruleminus, position23)
			}
			return true
		l22:
			position, tokenIndex = position22, tokenIndex22
			return false
		},
		/* 6 multiply <- <('*' sp)> */
		func() bool {
			position24, tokenIndex24 := position, tokenIndex
			{
				position25 := position
				if buffer[position] != rune('*') {
					goto l24
				}
				position++
				if !_rules[rulesp]() {
					goto l24
				}
				add(rulemultiply, position25)
			}
			return true
		l24:
			position, tokenIndex = position24, tokenIndex24
			return false
		},
		/* 7 divide <- <('/' sp)> */
		func() bool {
			position26, tokenIndex26 := position, tokenIndex
			{
				position27 := position
				if buffer[position] != rune('/') {
					goto l26
				}
				position++
				if !_rules[rulesp]() {
					goto l26
				}
				add(ruledivide, position27)
			}
			return true
		l26:
			position, tokenIndex = position26, tokenIndex26
			return false
		},
		/* 8 open <- <('(' sp)> */
		func() bool {
			position28, tokenIndex28 := position, tokenIndex
			{
				position29 := position
				if buffer[position] != rune('(') {
					goto l28
				}
				position++
				if !_rules[rulesp]() {
					goto l28
				}
				add(ruleopen, position29)
			}
			return true
		l28:
			position, tokenIndex = position28, tokenIndex28
			return false
		},
		/* 9 close <- <(')' sp)> */
		func() bool {
			position30, tokenIndex30 := position, tokenIndex
			{
				position31 := position
				if buffer[position] != rune(')') {
					goto l30
				}
				position++
				if !_rules[rulesp]() {
					goto l30
				}
				add(ruleclose, position31)
			}
			return true
		l30:
			position, tokenIndex = position30, tokenIndex30
			return false
		},
		/* 10 sp <- <(' ' / '\t')*> */
		func() bool {
			{
				position33 := position
			l34:
				{
					position35, tokenIndex35 := position, tokenIndex
					{
						position36, tokenIndex36 := position, tokenIndex
						if buffer[position] != rune(' ') {
							goto l37
						}
						position++
						goto l36
					l37:
						position, tokenIndex = position36, tokenIndex36
						if buffer[position] != rune('\t') {
							goto l35
						}
						position++
					}
				l36:
					goto l34
				l35:
					position, tokenIndex = position35, tokenIndex35
				}
				add(rulesp, position33)
			}
			return true
		},
		nil,
	}
	p.rules = _rules
}
