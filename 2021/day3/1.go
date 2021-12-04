package day3

import (
	"strconv"
)

func Day31() (int64, error) {
	bs, errB := getBinaries()
	if errB != nil {
		return 0, errB
	}

	_, gamma, epsilon, err := getMapCountGammaEpsilon(bs)
	if err != nil {
		return 0, err
	}

	gammaRate, errG := strconv.ParseInt(string(gamma), 2, 64)
	if errG != nil {
		return 0, errG
	}

	epsilonRate, errE := strconv.ParseInt(string(epsilon), 2, 64)
	if errE != nil {
		return 0, errE
	}

	return gammaRate * epsilonRate, nil
}
