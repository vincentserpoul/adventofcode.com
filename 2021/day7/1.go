package day7

import "math"

func Day71() (int, error) {
	is, err := getInput()
	if err != nil {
		return 0, err
	}

	return findMinFuelP1(is), nil
}

func findMinFuelP1(crabX []int) int {
	minFuel := math.MaxInt64
	for _, i := range crabX {
		fuel := 0
		for _, j := range crabX {
			fuel += max(i, j) - minE(i, j)
		}
		if fuel < minFuel {
			minFuel = fuel
		}
	}

	return minFuel
}
