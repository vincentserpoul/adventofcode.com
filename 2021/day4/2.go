package day4

import (
	"fmt"
)

func Day42() (int, error) {
	grids, selection, err := getInput()
	if err != nil {
		return 0, err
	}

	completedGrids := make(map[int]struct{})

	// loop through selection until we have a row or col equal to 5
	for _, i := range selection {
		for j, g := range grids {
			if _, ok := completedGrids[j]; ok {
				continue
			}
			isCompleted, err := g.draw(i)
			if err != nil {
				return 0, fmt.Errorf("draw err: %v", err)
			}
			if isCompleted {
				completedGrids[j] = struct{}{}
				if len(completedGrids) == len(grids) {
					return grids[j].sumNonSelected() * i, nil
				}
			}

		}
	}

	return 0, fmt.Errorf("no completed grid")
}
