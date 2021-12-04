package day1

import (
	"os"
	"strconv"
	"strings"
)

func getMeasurements() ([]int, error) {
	dat, err := os.ReadFile("./inputs/1.txt")
	if err != nil {
		return []int{}, err
	}
	measurements := strings.Split(string(dat), "\n")

	// cut out the last one
	measurements = measurements[:len(measurements)-1]

	measurementsC := make([]int, len(measurements))

	for i, v := range measurements {
		m, errC := strconv.Atoi(v)
		if errC != nil {
			return []int{}, errC
		}

		measurementsC[i] = m
	}

	return measurementsC, nil
}
