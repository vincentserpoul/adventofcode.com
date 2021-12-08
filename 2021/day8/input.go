package day8

import (
	"os"
	"sort"
	"strings"
)

func getInput() ([]*Pattern, error) {
	dat, err := os.ReadFile("./inputs/8.txt")
	if err != nil {
		return nil, err
	}

	lines := strings.Split(string(dat), "\n")
	// cut out the last one
	lines = lines[:len(lines)-1]

	var ps []*Pattern
	for _, l := range lines {
		p := strings.Split(l, " | ")
		allSegments := strings.Split(p[0], " ")
		for r := 0; r <= len(allSegments)-1; r++ {
			rs := []rune(allSegments[r])
			sort.Slice(rs, func(i, j int) bool {
				return rs[i] < rs[j]
			})
			allSegments[r] = string(rs)
		}
		display := strings.Split(p[1], " ")
		for r := 0; r <= len(display)-1; r++ {
			rs := []rune(display[r])
			sort.Slice(rs, func(i, j int) bool {
				return rs[i] < rs[j]
			})
			display[r] = string(rs)
		}
		ps = append(ps, &Pattern{
			allSegments: [10]string{
				allSegments[0],
				allSegments[1],
				allSegments[2],
				allSegments[3],
				allSegments[4],
				allSegments[5],
				allSegments[6],
				allSegments[7],
				allSegments[8],
				allSegments[9],
			},
			display: [4]string{
				display[0],
				display[1],
				display[2],
				display[3],
			},
		})
	}

	return ps, nil
}

type Pattern struct {
	allSegments [10]string
	display     [4]string
}

func (p Pattern) Count1748() int {
	count := 0

	for _, v := range p.display {
		switch len(v) {
		case 2:
			count++
		case 3:
			count++
		case 4:
			count++
		case 7:
			count++
		default:

		}
	}

	return count

}

func stringToRuneMap(s string) map[rune]struct{} {
	m := make(map[rune]struct{})
	for _, r := range s {
		m[r] = struct{}{}
	}

	return m
}

func Intersect(m1, m2 map[rune]struct{}) map[rune]struct{} {
	m := make(map[rune]struct{})
	for r := range m1 {
		if _, ok := m2[r]; ok {
			m[r] = struct{}{}
		}
	}

	return m
}

func Contains(m1, m2 map[rune]struct{}) bool {
	for r := range m2 {
		if _, ok := m1[r]; !ok {
			return false
		}
	}

	return true
}

func Add(m1, m2 map[rune]struct{}) map[rune]struct{} {
	m := make(map[rune]struct{})
	for r := range m1 {
		m[r] = struct{}{}
	}
	for r := range m2 {
		m[r] = struct{}{}
	}

	return m
}

func Minus(m1, m2 map[rune]struct{}) map[rune]struct{} {
	m := make(map[rune]struct{})
	for k, v := range m1 {
		m[k] = v
	}

	// remove from m
	for k := range m2 {
		delete(m, k)
	}

	return m
}

func (p Pattern) Display() int {
	match := make(map[int]string)

	lenSymbol := make(map[int][]string)
	// find 1 4 7 8
	for _, v := range p.allSegments {
		switch len(v) {
		case 2:
			match[1] = v
		case 3:
			match[7] = v
		case 4:
			match[4] = v
		case 5:
			lenSymbol[5] = append(lenSymbol[5], v)
		case 6:
			lenSymbol[6] = append(lenSymbol[6], v)
		case 7:
			match[8] = v
		default:

		}
	}

	// deduct 9
	for i, s := range lenSymbol[6] {
		if Contains(stringToRuneMap(s), stringToRuneMap(match[4])) {
			match[9] = s
			lenSymbol[6] = append(lenSymbol[6][:i], lenSymbol[6][i+1:]...)
			break
		}
	}

	// deduct 0 and 6
	for _, s := range lenSymbol[6] {
		x := Minus(stringToRuneMap(s), stringToRuneMap(match[1]))
		switch len(x) {
		case 4:
			match[0] = s
		case 5:
			match[6] = s
		}
	}

	// deduct 2
	for i, s := range lenSymbol[5] {
		if len(Minus(stringToRuneMap(match[9]), stringToRuneMap(s))) == 2 {
			match[2] = s
			lenSymbol[5] = append(lenSymbol[5][:i], lenSymbol[5][i+1:]...)
			break
		}
	}

	// deduct 3 and 5
	for _, s := range lenSymbol[5] {
		x := Minus(stringToRuneMap(s), stringToRuneMap(match[1]))
		switch len(x) {
		case 3:
			match[3] = s
		case 4:
			match[5] = s
		default:

		}
	}

	matchInvert := make(map[string]int)
	for i, s := range match {
		matchInvert[s] = i
	}

	return 1000*matchInvert[p.display[0]] +
		100*matchInvert[p.display[1]] +
		10*matchInvert[p.display[2]] +
		1*matchInvert[p.display[3]]
}
