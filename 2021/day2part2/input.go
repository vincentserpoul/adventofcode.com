package day2part2

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type CommandType int

const (
	Forward CommandType = iota + 1
	Down
	Up
)

func NewCommandType(d string) (CommandType, error) {
	switch d {
	case "forward":
		return Forward, nil
	case "down":
		return Down, nil
	case "up":
		return Up, nil
	default:
		return 0, fmt.Errorf("%s is not a CommandType", d)
	}
}

func ApplyCommand(pos, depth, aim int, c Command) (int, int, int) {
	endPos, endDepth, endAim := pos, depth, aim

	switch c.commandType {
	case Forward:
		endPos += c.unitCount
		endDepth += aim * c.unitCount
	case Up:
		endAim -= c.unitCount
	case Down:
		endAim += c.unitCount
	}

	return endPos, endDepth, endAim
}

type Command struct {
	commandType CommandType
	unitCount   int
}

func NewCommand(c string) (Command, error) {
	cmdA := strings.Split(c, " ")
	if len(cmdA) != 2 {
		return Command{}, fmt.Errorf("command not valid %s", c)
	}

	commandType, errD := NewCommandType(cmdA[0])
	if errD != nil {
		return Command{}, errD
	}

	unitCount, errA := strconv.Atoi(cmdA[1])
	if errA != nil {
		return Command{}, errA
	}

	return Command{commandType, unitCount}, nil
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
