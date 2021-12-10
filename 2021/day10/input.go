package day10

import (
	"os"
	"strings"
)

func getInput() ([][]rune, error) {
	dat, err := os.ReadFile("./inputs/10.txt")
	if err != nil {
		return nil, err
	}

	lines := strings.Split(string(dat), "\n")
	// cut out the last one
	lines = lines[:len(lines)-1]

	res := make([][]rune, len(lines))
	for i, l := range lines {
		res[i] = []rune(l)
	}

	return res, nil
}

func matching(r rune) rune {
	switch r {
	case ')':
		return '('
	case ']':
		return '['
	case '}':
		return '{'
	case '>':
		return '<'
	case '(':
		return ')'
	case '[':
		return ']'
	case '{':
		return '}'
	case '<':
		return '>'
	default:
		return 'x'
	}
}
