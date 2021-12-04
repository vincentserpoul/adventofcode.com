package day3

import (
	"os"
	"strings"
)

func getBinaries() ([]string, error) {
	dat, err := os.ReadFile("./inputs/3.txt")
	if err != nil {
		return []string{}, err
	}

	binaries := strings.Split(string(dat), "\n")

	// cut out the last one
	binaries = binaries[:len(binaries)-1]

	return binaries, nil
}

func getMapCountGammaEpsilon(bs []string) ([]string, []rune, []rune, error) {
	// make a count of 0 and 1 for each index
	countMap := make([][2]int, len(bs[0]))

	for _, v := range bs {
		for j, b := range v {
			if b == '0' {
				countMap[j][0]++
			} else {
				countMap[j][1]++
			}
		}
	}

	// reconstitute gamma and epsilon
	gamma := make([]rune, len(bs[0]))
	epsilon := make([]rune, len(bs[0]))

	for i, v := range countMap {
		if v[0] == v[1] {
			gamma[i] = '1'
			epsilon[i] = '0'
		} else if v[0] > v[1] {
			gamma[i] = '0'
			epsilon[i] = '1'
		} else {
			gamma[i] = '1'
			epsilon[i] = '0'
		}
	}

	return bs, gamma, epsilon, nil
}
