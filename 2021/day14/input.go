package day14

import (
	"math"
	"os"
	"strings"
)

func stepsApply(steps int) (int, error) {
	polymer, transfo, err := getInput()
	if err != nil {
		return 0, err
	}

	polymerC := make(map[string]int)
	pR := []rune(polymer)
	for i, r := range pR[:len(pR)-1] {
		key := string([]rune{r, pR[i+1]})
		if _, ok := polymerC[key]; !ok {
			polymerC[key] = 0
		}
		polymerC[key]++
	}
	polymerC[string(pR[len(pR)-1])] = 1

	for step := 1; step <= steps; step++ {
		newPolymerC := make(map[string]int)
		for p, count := range polymerC {
			v, ok := transfo[p]
			if !ok {
				if _, ok := newPolymerC[p]; !ok {
					newPolymerC[p] = 0
				}
				newPolymerC[p] += count

				continue
			}

			pR := []rune(p)

			newPair1 := string([]rune{pR[0], v})
			if _, ok := newPolymerC[newPair1]; !ok {
				newPolymerC[newPair1] = 0
			}
			newPolymerC[newPair1] += count

			newPair2 := string([]rune{v, pR[1]})
			if _, ok := newPolymerC[newPair2]; !ok {
				newPolymerC[newPair2] = 0
			}
			newPolymerC[newPair2] += count
		}

		polymerC = newPolymerC
	}

	// count for each
	countElem := make(map[rune]int)

	for p, count := range polymerC {
		pR := []rune(p)
		r := pR[0]

		if _, ok := countElem[r]; !ok {
			countElem[r] = 0
		}
		countElem[r] += count
	}

	min, max := math.MaxInt64, math.MinInt64
	for _, v := range countElem {
		if v > max {
			max = v
		}

		if v < min {
			min = v
		}
	}

	return max - min, nil
}

func getInput() (string, map[string]rune, error) {
	dat, err := os.ReadFile("./inputs/14.txt")
	if err != nil {
		return "", nil, err
	}

	lines := strings.Split(string(dat), "\n")
	// cut out the last one
	lines = lines[:len(lines)-1]

	// get starting polymer
	polymer := lines[0]
	lines = lines[2:]

	transfo := make(map[string]rune)
	for _, l := range lines {
		t := strings.Split(l, " -> ")
		transfo[t[0]] = rune([]rune(t[1])[0])
	}

	return polymer, transfo, nil
}
