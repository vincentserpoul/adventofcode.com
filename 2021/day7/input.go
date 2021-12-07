package day7

import (
	"os"
	"strconv"
	"strings"
)

func getInput() ([]int, error) {
	dat, err := os.ReadFile("./inputs/7.txt")
	if err != nil {
		return nil, err
	}

	lines := strings.Split(string(dat), "\n")
	// cut out the last one
	lines = lines[:len(lines)-1]

	ds := strings.Split(lines[0], ",")

	dsi := make([]int, len(ds))
	for i, d := range ds {
		di, errI := strconv.Atoi(d)
		if errI != nil {
			return nil, errI
		}

		dsi[i] = di
	}

	return dsi, nil
}

func minE(i, j int) int {
	if i <= j {
		return i
	}

	return j
}

func max(i, j int) int {
	if i > j {
		return i
	}

	return j
}
