package day17

func Day172() (int, error) {
	ta := getInput()

	countInTarget := 0
	for startVeloX := 0; startVeloX <= 2*ta.maxX; startVeloX++ {
		for startVeloY := 2 * ta.minY; startVeloY <= -2*ta.minY; startVeloY++ {
			inTarget, _, _ := runSteps(startVeloX, startVeloY, ta)
			if inTarget {
				countInTarget++
			}
		}
	}

	return countInTarget, nil
}
