package day3

import (
	"fmt"
	"strconv"
)

func Day32() (int64, error) {
	bs, errB := getBinaries()
	if errB != nil {
		return 0, errB
	}

	oxyBs := make([]string, len(bs))
	copy(oxyBs, bs)
	_, gammaOxy, _, errOxy := getMapCountGammaEpsilon(bs)
	if errOxy != nil {
		return 0, errOxy
	}
	oxyIdx := 0
	for len(oxyBs) > 1 {
		oxyBs = filter(oxyBs, oxyIdx, gammaOxy)
		_, gammaOxy, _, errOxy = getMapCountGammaEpsilon(oxyBs)
		if errOxy != nil {
			return 0, errOxy
		}
		oxyIdx++
	}
	oxyRate, errO := strconv.ParseInt(oxyBs[0], 2, 64)
	if errO != nil {
		return 0, errO
	}

	co2Bs := make([]string, len(bs))
	copy(co2Bs, bs)
	_, _, epsilonCO2, errCO2 := getMapCountGammaEpsilon(co2Bs)
	if errCO2 != nil {
		return 0, errCO2
	}
	co2Idx := 0
	for len(co2Bs) > 1 {
		co2Bs = filter(co2Bs, co2Idx, epsilonCO2)
		_, _, epsilonCO2, errCO2 = getMapCountGammaEpsilon(co2Bs)
		if errCO2 != nil {
			return 0, errCO2
		}
		co2Idx++
	}
	co2Rate, errC := strconv.ParseInt(co2Bs[0], 2, 64)
	if errC != nil {
		return 0, errC
	}

	return co2Rate * oxyRate, nil
}

func filter(bs []string, idx int, ref []rune) []string {
	if idx > len(ref)-1 {
		panic(string(ref) + " idx " + fmt.Sprintf("%d", idx) + " not cool")
	}
	var res []string
	for _, v := range bs {
		vr := []rune(v)
		if vr[idx] == ref[idx] {
			res = append(res, v)
		}
	}

	return res
}
