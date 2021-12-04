package day4

import (
	"fmt"
)

func Day41() (int, error) {
	grids, selection, err := getInput()
	if err != nil {
		return 0, err
	}

	// loop through selection until we have a row or col equal to 5
	for _, i := range selection {
		for _, g := range grids {
			isCompleted, err := g.draw(i)
			if err != nil {
				return 0, fmt.Errorf("draw err: %v", err)
			}
			if isCompleted {
				return g.sumNonSelected() * i, nil
			}
		}
	}

	return 0, fmt.Errorf("no completed grid")
}
