package day2part1

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Direction int

const (
	Forward Direction = iota + 1
	Down
	Up
)

func NewDirection(d string) (Direction, error) {
	switch d {
	case "forward":
		return Forward, nil
	case "down":
		return Down, nil
	case "up":
		return Up, nil
	default:
		return 0, fmt.Errorf("%s is not a direction", d)
	}
}

func ApplyCommand(pos, depth int, c Command) (int, int) {
	endPos, endDepth := pos, depth

	switch c.direction {
	case Forward:
		endPos += c.unitCount
	case Up:
		endDepth -= c.unitCount
	case Down:
		endDepth += c.unitCount
	}

	return endPos, endDepth
}

type Command struct {
	direction Direction
	unitCount int
}

func NewCommand(c string) (Command, error) {
	cmdA := strings.Split(c, " ")
	if len(cmdA) != 2 {
		return Command{}, fmt.Errorf("command not valid %s", c)
	}

	direction, errD := NewDirection(cmdA[0])
	if errD != nil {
		return Command{}, errD
	}
	unitCount, errA := strconv.Atoi(cmdA[1])
	if errA != nil {
		return Command{}, errA
	}

	return Command{direction, unitCount}, nil
}

func getCommands() ([]Command, error) {
	dat, err := os.ReadFile("./inputs/2.txt")
	if err != nil {
		return []Command{}, err
	}
	commands := strings.Split(string(dat), "\n")

	// cut out the last one
	commands = commands[:len(commands)-1]

	commandsC := make([]Command, len(commands))

	for i, v := range commands {
		c, errC := NewCommand(v)
		if errC != nil {
			return []Command{}, errC
		}

		commandsC[i] = c
	}

	return commandsC, nil
}
