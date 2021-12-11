package day11

func Day112() (int, error) {
	lines, err := getInput()
	if err != nil {
		return 0, err
	}

	flashCount := 0
	for step := 1; step <= 1000; step++ {
		var q [][2]int
		// add +1
		for i, line := range lines {
			for j := range line {
				lines[i][j]++
				if lines[i][j] > 9 {
					q = append(q, [2]int{i, j})
					// fmt.Println("queueing", [2]int{i, j})
				}
			}
		}

		// for each in the q, flash around, set to 0 && set over9
		flashed := make(map[string][2]int)
		for len(q) >= 1 {
			curr := q[0]
			q = q[1:]

			if _, ok := flashed[nLvlToKey(curr)]; ok {
				continue
			}

			// flash
			flashed[nLvlToKey(curr)] = curr
			// fmt.Println("flashing", curr)

			// flash around
			i, j := curr[0], curr[1]
			for _, octo := range [][2]int{
				{i - 1, j},
				{i - 1, j + 1},
				{i, j + 1},
				{i + 1, j + 1},
				{i + 1, j},
				{i + 1, j - 1},
				{i, j - 1},
				{i - 1, j - 1},
			} {
				if octo[0] >= 0 && octo[0] <= len(lines)-1 &&
					octo[1] >= 0 && octo[1] <= len(lines[0])-1 {
					lines[octo[0]][octo[1]]++
					_, ok := flashed[nLvlToKey(octo)]
					if !ok && lines[octo[0]][octo[1]] > 9 {
						q = append(q, octo)
						// fmt.Println("queueing", octo)
					}
				}
			}
		}

		// put all over 9 to 0
		for _, o := range flashed {
			lines[o[0]][o[1]] = 0
		}

		if len(flashed) == len(lines)*len(lines[0]) {
			return step, nil
		}
	}

	// for _, v := range lines {
	// 	fmt.Println(v)
	// }

	return flashCount, nil
}
