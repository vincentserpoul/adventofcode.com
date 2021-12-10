package day10

import (
	"sort"
)

func Day102() (int, error) {
	lines, err := getInput()
	if err != nil {
		return 0, err
	}

	var addedRunes [][]rune
	for _, l := range lines {
		stack := []rune{}
		correct := true
	runeLoop:
		for _, r := range l {
			switch r {
			case '(', '[', '{', '<':
				stack = append(stack, r)
			case ')', ']', '}', '>':
				top := stack[len(stack)-1]
				if matching(top) == r {
					stack = stack[:len(stack)-1]
				} else {
					correct = false
					break runeLoop
				}
			}
		}
		if !correct {
			continue
		}
		currAddedRunes := []rune{}
		for len(stack) >= 1 {
			top := stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			currAddedRunes = append(currAddedRunes, matching(top))
		}

		addedRunes = append(addedRunes, currAddedRunes)
	}

	scores := make([]int, len(addedRunes))
	for i, rs := range addedRunes {
		scores[i] = calculatePoints(rs)
	}
	sort.Ints(scores)

	midIdx := len(scores) / 2

	return scores[midIdx], nil
}

func calculatePoints(rs []rune) int {
	score := 0
	for _, r := range rs {
		score *= 5
		score += pointsAdd(r)
	}

	return score
}

func pointsAdd(r rune) int {
	switch r {
	case ')':
		return 1
	case ']':
		return 2
	case '}':
		return 3
	case '>':
		return 4
	default:
		return 0
	}
}
