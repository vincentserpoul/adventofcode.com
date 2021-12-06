package day6

import (
	"os"
	"strconv"
	"strings"
)

func getInput() (school, error) {
	dat, err := os.ReadFile("./inputs/6.txt")
	if err != nil {
		return school{}, err
	}

	lines := strings.Split(string(dat), "\n")
	// cut out the last one
	lines = lines[:len(lines)-1]

	ds := strings.Split(lines[0], ",")

	var sc school
	for _, d := range ds {
		di, errI := strconv.Atoi(d)
		if errI != nil {
			return school{}, errI
		}

		sc[di]++
	}

	return sc, nil
}

type school [9]int

func live(days int) (int, error) {
	sc, err := getInput()
	if err != nil {
		return 0, err
	}
	// fmt.Println(sc)
	for day := 1; day <= days; day++ {
		// copy number of current 0
		newBorns := sc[0] // will move to 8 and 6
		for i := 0; i <= 7; i++ {
			sc[i] = sc[i+1]
		}
		sc[8] = newBorns
		sc[6] += newBorns
	}

	count := 0
	for _, cf := range sc {
		count += cf
	}

	return count, nil
}
