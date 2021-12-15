package day15

import (
	"container/heap"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func getInput() ([][]int, error) {
	dat, err := os.ReadFile("./inputs/15.txt")
	if err != nil {
		return nil, err
	}

	lines := strings.Split(string(dat), "\n")
	// cut out the last one
	lines = lines[:len(lines)-1]

	grid := make([][]int, len(lines))
	for row, l := range lines {
		grid[row] = make([]int, len(l))
		for col, r := range []rune(l) {
			grid[row][col] = int(r - '0')
		}
	}

	return grid, nil
}

func heuristicCost(r, c, targetR, targetC int) int {
	return targetR - r + targetC - c
}

// An Item is something we manage in a priority queue.
type Item struct {
	coords    []int // The value of the item; arbitrary.
	costSoFar int
	priority  int // The priority of the item in the queue.
	index     int // The index of the item in the heap.
}

// A PriorityQueue implements heap.Interface and holds Items.
type PriorityQueue []*Item

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i].priority < pq[j].priority
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}

func (pq *PriorityQueue) Push(x interface{}) {
	n := len(*pq)
	item := x.(*Item)
	item.index = n
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil  // avoid memory leak
	item.index = -1 // for safety
	*pq = old[0 : n-1]
	return item
}

func keyS(i, j int) string {
	return strconv.Itoa(i) + "-" + strconv.Itoa(j)
}

// This example creates a PriorityQueue with some items, adds and manipulates an item,
// and then removes the items in priority order.
func shortestPathRisk(grid [][]int, gridExpansion int) int {
	targetRow, targetCol := gridTarget(grid, gridExpansion)
	visited := make(map[string]struct{})
	pq := make(PriorityQueue, 0)
	heap.Init(&pq)
	heap.Push(&pq, &Item{
		coords: []int{0, 0},
	})

	for len(pq) >= 1 {
		// take elem
		currP := heap.Pop(&pq).(*Item)

		visited[keyS(currP.coords[0], currP.coords[1])] = struct{}{}
		// fmt.Printf("visited %d, %d\n", currP.coords[0], currP.coords[1])

		if currP.coords[0] == targetRow && currP.coords[1] == targetCol {
			return currP.costSoFar
		}

		// find all possible positions and push them to the Q
		row := currP.coords[0]
		col := currP.coords[1]
		for _, c := range [][]int{
			{row - 1, col},
			{row, col + 1},
			{row + 1, col},
			{row, col - 1},
		} {
			if _, ok := visited[keyS(c[0], c[1])]; ok {
				continue
			}

			risk, err := gridVal(grid, gridExpansion, c[0], c[1])
			if err != nil {
				continue
			}

			newPos := &Item{
				coords:    c,
				costSoFar: currP.costSoFar + risk,
				priority:  currP.costSoFar + risk + heuristicCost(c[0], c[1], targetRow, targetCol),
			}

			heap.Push(&pq, newPos)
		}
	}

	return 0
}

func gridTarget(grid [][]int, gridExpansion int) (int, int) {
	return len(grid)*gridExpansion - 1, len(grid[0])*gridExpansion - 1
}

func gridVal(grid [][]int, gridExpansion, row, col int) (int, error) {
	if row < 0 || col < 0 ||
		row > len(grid)*gridExpansion-1 || col > len(grid[0])*gridExpansion-1 {
		return 0, fmt.Errorf("%d, %d not in grid max %d, %d", row, col, len(grid)*gridExpansion-1, len(grid[0])*gridExpansion-1)
	}

	if row <= len(grid)-1 && col <= len(grid[0])-1 {
		return grid[row][col], nil
	}

	sourceRow := row % len(grid)
	sourceCol := col % len(grid)
	sourceVal := grid[sourceRow][sourceCol]

	valToBeAdded := row/len(grid) + col/len(grid[0])

	res := sourceVal + valToBeAdded
	if res > 9 {
		return res % 9, nil
	}

	return res, nil
}
