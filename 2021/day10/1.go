package day10

func Day101() (int, error) {
	lines, err := getInput()
	if err != nil {
		return 0, err
	}

	illegalPoints := 0
	for _, l := range lines {
		stack := []rune{}
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
					illegalPoints += pointsIllegal(r)
					break runeLoop
				}
			}
		}
	}

	return illegalPoints, nil
}

func pointsIllegal(r rune) int {
	switch r {
	case ')':
		return 3
	case ']':
		return 57
	case '}':
		return 1197
	case '>':
		return 25137
	default:
		return 0
	}
}
