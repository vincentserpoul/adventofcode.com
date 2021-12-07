package day7

import (
	"math"
	"sort"
)

func Day72() (int, error) {
	is, err := getInput()
	if err != nil {
		return 0, err
	}

	return findMinFuelP2(is), nil
}

func findMinFuelP2(crabX []int) int {
	minFuel := math.MaxInt64
	sort.Ints(crabX)
	maxCrabX := crabX[len(crabX)-1]

	for i := 0; i <= maxCrabX; i++ {
		fuel := 0
		for _, j := range crabX {
			distance := max(i, j) - minE(i, j)
			// somme d une suite arithmetique de raison 1
			fuel += distance * (distance + 1) / 2
		}

		if fuel < minFuel {
			minFuel = fuel
		}
	}

	return minFuel
}
