package day13

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type dot struct {
	x int
	y int
}

func NewDotFromString(s string) dot {
	c := strings.Split(s, ",")
	x, _ := strconv.Atoi(c[0])
	y, _ := strconv.Atoi(c[1])

	return dot{x, y}
}

type along int

func NewAlongFromString(s string) along {
	switch s {
	case "x":
		return alongX
	case "y":
		return alongY
	default:
		panic("not along")
	}
}

const (
	alongX along = iota + 1
	alongY
)

type inst struct {
	along along
	val   int
}

func NewFromString(s string) (inst, error) {
	if len(s) < 12 {
		return inst{}, fmt.Errorf("not an inst")
	}

	if s[:11] != "fold along " {
		return inst{}, fmt.Errorf("not an inst")
	}

	i := strings.Split(s, "fold along ")
	in := strings.Split(i[1], "=")

	a := NewAlongFromString(in[0])
	val, err := strconv.Atoi(in[1])
	if err != nil {
		panic("NaN")
	}

	return inst{along: a, val: val}, nil
}

func getInput() ([]dot, []inst, error) {
	dat, err := os.ReadFile("./inputs/13.txt")
	if err != nil {
		return nil, nil, err
	}

	lines := strings.Split(string(dat), "\n")
	// cut out the last one
	lines = lines[:len(lines)-1]

	instructions := []inst{}
	dots := []dot{}
	for _, l := range lines {
		if strings.TrimSpace(l) == "" {
			continue
		}
		ins, err := NewFromString(l)
		if err == nil {
			instructions = append(instructions, ins)
			continue
		}

		// coord
		d := NewDotFromString(l)
		dots = append(dots, d)
	}

	return dots, instructions, nil
}

func applyInst(dots []dot, i inst) []dot {
	switch i.along {
	case alongX:
		return applyInstFoldX(dots, i.val)
	case alongY:
		return applyInstFoldY(dots, i.val)
	default:
		panic("not an inst")
	}
}

func applyInstFoldX(dots []dot, val int) []dot {
	// find all dots that are beyond that limit
	targetDots := []dot{}
	keptDots := []dot{}
	for _, d := range dots {
		if d.x > val {
			targetDots = append(targetDots, d)
		} else {
			keptDots = append(keptDots, d)
		}
	}

	// for all target dots, do the symetry with x
	for _, d := range targetDots {
		newX := d.x - 2*(d.x-val)

		// append this dot to the target dots if it doesnt exist
		found := false
		for _, v := range keptDots {
			if v.x == newX && v.y == d.y {
				found = true
				break
			}
		}
		if !found {
			keptDots = append(keptDots, dot{x: newX, y: d.y})
		}
	}

	return keptDots
}

func applyInstFoldY(dots []dot, val int) []dot {
	// find all dots that are beyond that limit
	targetDots := []dot{}
	keptDots := []dot{}
	for _, d := range dots {
		if d.y > val {
			targetDots = append(targetDots, d)
		} else {
			keptDots = append(keptDots, d)
		}
	}

	// for all target dots, do the symetry with x
	for _, d := range targetDots {
		newY := d.y - 2*(d.y-val)

		// append this dot to the target dots if it doesnt exist
		found := false
		for _, v := range keptDots {
			if v.x == d.x && v.y == newY {
				found = true
				break
			}
		}
		if !found {
			keptDots = append(keptDots, dot{x: d.x, y: newY})
		}
	}

	return keptDots
}
