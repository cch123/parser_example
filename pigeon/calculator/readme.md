rm -rf main.go && pigeon ./calc.peg > main.go && go run main.go "  1 + 2    *10"

语法里的 _ 应该是要 skip 的元素

在 pigeon 里也要考虑别人会传空格过来。。。
