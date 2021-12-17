package day17

import (
	"math"
)

type targetArea struct {
	minX int
	maxX int
	minY int
	maxY int
}

func getInput() targetArea {
	return targetArea{
		minX: 79,
		maxX: 137,
		minY: -176,
		maxY: -117,
	}
}

func getExampleInput() targetArea {
	return targetArea{
		minX: 20,
		maxX: 30,
		minY: -10,
		maxY: -5,
	}
}

func runSteps(startVeloX, startVeloY int, ta targetArea) (bool, int, int) {
	// fmt.Println("run steps for", startVeloX, startVeloY)
	maxX, maxY := math.MinInt64, math.MinInt64
	hasReachedTargetArea := false

	x, y, stepCount := 0, 0, 0
	veloX, veloY := startVeloX, startVeloY

	for y >= ta.minY {
		if y > maxY {
			maxY = y
		}
		if x > maxX {
			maxX = x
		}

		if isInTargetArea(x, y, ta) {
			hasReachedTargetArea = true
		}

		x, y, veloX, veloY = step(x, y, veloX, veloY)
		stepCount++
	}

	return hasReachedTargetArea, maxX, maxY
}

func step(x, y, veloX, veloY int) (int, int, int, int) {
	x += veloX
	y += veloY

	if veloX < 0 {
		veloX++
	} else if veloX > 0 {
		veloX--
	}

	veloY--

	return x, y, veloX, veloY
}

func isInTargetArea(x, y int, ta targetArea) bool {
	return x <= ta.maxX &&
		x >= ta.minX &&
		y >= ta.minY &&
		y <= ta.maxY
}
