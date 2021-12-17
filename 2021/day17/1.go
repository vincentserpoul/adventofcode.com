package day17

import (
	"math"
)

func Day171() (int, error) {
	ta := getInput()

	absMaxY := math.MinInt64
	for startVeloX := 0; startVeloX <= 2*ta.maxX; startVeloX++ {
		for startVeloY := 2 * ta.minY; startVeloY <= -2*ta.minY; startVeloY++ {
			inTarget, _, maxY := runSteps(startVeloX, startVeloY, ta)
			if inTarget && maxY > absMaxY {
				absMaxY = maxY
			}
		}
	}

	return absMaxY, nil
}
