package day13

import (
	"fmt"
	"math"
)

func Day132() (int, error) {
	dots, instructions, err := getInput()
	if err != nil {
		return 0, err
	}

	leftDots := dots
	for _, i := range instructions {
		leftDots = applyInst(leftDots, i)
	}

	displayGrid(leftDots)

	return len(dots), nil
}

func displayGrid(dots []dot) {
	minX, maxX := math.MaxInt32, math.MinInt32
	minY, maxY := math.MaxInt32, math.MinInt32

	for _, v := range dots {
		if v.x < minX {
			minX = v.x
		}
		if v.x > maxX {
			maxX = v.x
		}
		if v.y < minY {
			minY = v.y
		}
		if v.y > maxY {
			maxY = v.y
		}
	}

	for y := minY; y <= maxY; y++ {
		for x := minX; x <= maxX; x++ {
			if isDot(dots, x, y) {
				fmt.Printf("#")
			} else {
				fmt.Printf(".")
			}
		}
		fmt.Println()
	}
}

func isDot(dots []dot, x, y int) bool {
	for _, d := range dots {
		if d.x == x && d.y == y {
			return true
		}
	}

	return false
}
