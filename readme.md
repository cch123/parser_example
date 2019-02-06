# Parser Example

编译原理、图形学和操作系统是程序员的三大浪漫。嗯，所以常见的 parser 和 parser generator 还是要了解一下。

本仓库里会有一些各种各样的 parser 的 demo，提供给大家参考。每个 parser 尽量都实现一个 bool expression 和简单的计算器的 parser。当然，那些 SQL Parser 不计算在内。

## 完成情况

- [ ] antlr
  - [ ] calculator
  - [ ] bool_parser
- [ ] combine
  - [ ] calculator
  - [ ] bool_parser
- [x] go-internal
  - [x] [calculator](go-internal/calculator)
  - [x] [bool_parser](go-internal/bool_parser)
- [ ] go-peg
  - [ ] calculator
  - [ ] bool_parser
- [ ] go-sqlparser
  - [ ] calculator
  - [ ] bool_parser
- [ ] go-tidb-parser
  - [ ] calculator
  - [ ] bool_parser
- [ ] goyacc
  - [ ] calculator
  - [ ] bool_parser
- [ ] lalrpop
  - [ ] calculator
  - [ ] bool_parser
- [x] nom
  - [x] [calculator](nom/calculator)
  - [x] [bool_parser](nom/bool_expr_left_assoc)
- [ ] pest
  - [ ] calculator
  - [x] [bool_parser](pest/bool_parser)
- [ ] syn
  - [ ] calculator
  - [ ] bool_parser
