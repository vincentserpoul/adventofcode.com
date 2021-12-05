package day5

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func Day51() (int, error) {
	dat, err := os.ReadFile("./inputs/5.txt")
	if err != nil {
		return 0, err
	}

	lines := strings.Split(string(dat), "\n")
	// cut out the last one
	lines = lines[:len(lines)-1]

	grid := make(map[int]map[int]int)
	over2 := 0
	for _, s := range lines {
		hd, errH := hydroventPointsFromString(s)
		if errH != nil {
			return 0, errH
		}

		if !isLine(hd) {
			continue
		}

		var errHG error
		grid, over2, errHG = setHydroventInGrid(grid, over2, hd)
		if errHG != nil {
			return 0, errHG
		}

	}

	return over2, nil
}

func Day52() (int, error) {
	dat, err := os.ReadFile("./inputs/5.txt")
	if err != nil {
		return 0, err
	}

	lines := strings.Split(string(dat), "\n")
	// cut out the last one
	lines = lines[:len(lines)-1]

	grid := make(map[int]map[int]int)
	over2 := 0
	for _, s := range lines {
		hd, errH := hydroventPointsFromString(s)
		if errH != nil {
			return 0, errH
		}

		var errHG error
		grid, over2, errHG = setHydroventInGrid(grid, over2, hd)
		if errHG != nil {
			return 0, errHG
		}

	}

	return over2, nil
}

func isLine(hd [2][2]int) bool {
	if hd[0][0] == hd[1][0] || hd[0][1] == hd[1][1] {
		return true
	}

	return false
}

func hydroventPointsFromString(s string) ([2][2]int, error) {
	pts := strings.Split(s, " -> ")
	if len(pts) != 2 {
		return [2][2]int{}, fmt.Errorf("bad points: %s", pts)
	}

	a, errA := pointToCoords(pts[0])
	if errA != nil {
		return [2][2]int{}, errA
	}

	b, errB := pointToCoords(pts[1])
	if errB != nil {
		return [2][2]int{}, errB
	}

	return [2][2]int{a, b}, nil
}

func pointToCoords(s string) ([2]int, error) {
	as := strings.Split(s, ",")
	if len(as) != 2 {
		return [2]int{}, fmt.Errorf("no 2 points")
	}

	x, errCa := strconv.Atoi(as[0])
	if errCa != nil {
		return [2]int{}, fmt.Errorf("not a point %s: %v", as[0], errCa)
	}

	y, errCb := strconv.Atoi(as[1])
	if errCb != nil {
		return [2]int{}, fmt.Errorf("not a point %s: %v", as[1], errCb)
	}

	return [2]int{x, y}, nil
}

func setHydroventInGrid(
	grid map[int]map[int]int,
	over2 int,
	hydrovent [2][2]int,
) (map[int]map[int]int, int, error) {
	xA, yA := hydrovent[0][0], hydrovent[0][1]
	xB, yB := hydrovent[1][0], hydrovent[1][1]

	i, j := xA, yA
	for {
		if _, ok := grid[i]; !ok {
			grid[i] = make(map[int]int)
		}
		if _, ok := grid[i][j]; !ok {
			grid[i][j] = 0
		}
		grid[i][j]++
		if grid[i][j] == 2 {
			over2++
		}

		if i == xB && j == yB {
			break
		}

		if xA > xB {
			i--
		} else if xA < xB {
			i++
		}

		if yA > yB {
			j--
		} else if yA < yB {
			j++
		}
	}

	return grid, over2, nil
}
