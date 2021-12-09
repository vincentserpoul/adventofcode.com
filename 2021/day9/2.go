package day9

import (
	"sort"
)

func Day92() (int, error) {
	grid, err := getInput()
	if err != nil {
		return 0, err
	}

	uf := NewUF(grid)
	colWidth := len(grid[0])

	// for each cell, just union with the ones around which are not 9
	for r, row := range grid {
		for c, cell := range row {
			if cell == 9 {
				continue
			}

			// otherwise, look t r b l
			// top
			if r-1 >= 0 {
				k := r - 1
				l := c
				if uf.heads[coordsToLine(colWidth, k, l)] != -1 {
					uf.union(coordsToLine(colWidth, k, l), coordsToLine(colWidth, r, c))
				}
			}
			// right
			if c+1 <= colWidth-1 {
				k := r
				l := c + 1
				if uf.heads[coordsToLine(colWidth, k, l)] != -1 {
					uf.union(coordsToLine(colWidth, k, l), coordsToLine(colWidth, r, c))
				}
			}
			// bottom
			if r+1 <= len(grid)-1 {
				k := r + 1
				l := c
				if uf.heads[coordsToLine(colWidth, k, l)] != -1 {
					uf.union(coordsToLine(colWidth, k, l), coordsToLine(colWidth, r, c))
				}
			}
			// left
			if c-1 >= 0 {
				k := r
				l := c - 1
				if uf.heads[coordsToLine(colWidth, k, l)] != -1 {
					uf.union(coordsToLine(colWidth, k, l), coordsToLine(colWidth, r, c))
				}
			}
		}
	}

	// final find
	for i, v := range uf.heads {
		if v != -1 {
			uf.find(i)
		}
	}

	countMap := make(map[int]int)
	for _, v := range uf.heads {
		if v == -1 {
			continue
		}
		if _, ok := countMap[v]; !ok {
			countMap[v] = 0
		}
		countMap[v]++
	}

	// put in an array and sort it
	var final []int
	for _, v := range countMap {
		final = append(final, v)
	}
	sort.Ints(final)

	return final[len(final)-1] * final[len(final)-2] * final[len(final)-3], nil
}

func coordsToLine(colWidth, r, c int) int {
	return colWidth*r + c
}

type UF struct {
	heads []int
	total int
}

func NewUF(grid [][]int) *UF {
	colWidth := len(grid[0])
	heads := make([]int, colWidth*len(grid))
	total := 0

	// make itself head
	for r, row := range grid {
		for c, cell := range row {
			if cell == 9 {
				heads[coordsToLine(colWidth, r, c)] = -1
				continue
			}

			heads[coordsToLine(colWidth, r, c)] = coordsToLine(colWidth, r, c)
			total++
		}
	}

	return &UF{
		heads,
		total,
	}
}

func (uf *UF) find(i int) int {
	if uf.heads[i] != i {
		uf.heads[i] = uf.find(uf.heads[i])
	}

	return uf.heads[i]
}

func (uf *UF) union(i, j int) {
	hi := uf.find(i)
	hj := uf.find(j)
	if hj == hi {
		return
	}

	uf.total--
	// otherwise, find the longest and add it
	if hi > hj {
		uf.heads[hj] = hi

		return
	}

	uf.heads[hi] = hj
}
