package day11

import (
	"os"
	"strings"
)

func getInput() ([][]int, error) {
	dat, err := os.ReadFile("./inputs/11.txt")
	if err != nil {
		return nil, err
	}

	lines := strings.Split(string(dat), "\n")
	// cut out the last one
	lines = lines[:len(lines)-1]

	res := make([][]int, len(lines))
	for i, l := range lines {
		res[i] = make([]int, len([]rune(l)))
		for j, v := range []rune(l) {
			res[i][j] = int(v - '0')
		}
	}

	return res, nil
}
