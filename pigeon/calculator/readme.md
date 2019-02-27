rm -rf main.go && pigeon ./calc.peg > main.go && go run main.go "  1 + 2    *10"
