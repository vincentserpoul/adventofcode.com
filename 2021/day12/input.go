package day12

import (
	"os"
	"strings"
	"unicode"
)

type caveType int

const (
	small caveType = iota + 1
	big
)

type Node struct {
	caveType    caveType
	name        string
	connections map[string]*Node
}

func (n *Node) String() string {
	return n.name
}

func StringPath(p []*Node) []string {
	res := make([]string, len(p))
	for i, n := range p {
		res[i] = n.String()
	}
	return res
}

func NewNode(n string) *Node {
	return &Node{
		caveType:    caveTypeFromString(n),
		name:        n,
		connections: make(map[string]*Node),
	}
}

func (n *Node) connect(c *Node) {
	n.connections[c.name] = c
}

func caveTypeFromString(s string) caveType {
	if unicode.IsLower([]rune(s)[0]) {
		return small
	}

	return big
}

type Network map[string]*Node

func recurPath(
	allPaths [][]*Node,
	visited map[string]int,
	currPath []*Node,
	n *Node,
	canBeVisited func(*Node, map[string]int) bool,
) [][]*Node {
	newVisited := copyMap(visited)
	if _, ok := newVisited[n.name]; !ok {
		newVisited[n.name] = 0
	}
	newVisited[n.name]++

	currPath = append(currPath, n)

	if n.name == "end" {
		allPaths = append(allPaths, currPath)

		return allPaths
	}

	// find all possible next conn
	for _, pn := range n.connections {
		if !canBeVisited(pn, newVisited) {
			continue
		}

		newCurrPath := make([]*Node, len(currPath))
		copy(newCurrPath, currPath)
		allPaths = recurPath(allPaths, newVisited, newCurrPath, pn, canBeVisited)
	}

	return allPaths
}

func copyMap(m map[string]int) map[string]int {
	cop := make(map[string]int)
	for k, v := range m {
		cop[k] = v
	}

	return cop
}

func getInput() (Network, error) {
	dat, err := os.ReadFile("./inputs/12.txt")
	if err != nil {
		return nil, err
	}

	lines := strings.Split(string(dat), "\n")
	// cut out the last one
	lines = lines[:len(lines)-1]

	net := make(map[string]*Node)
	for _, l := range lines {
		// extract both nodes
		ns := strings.Split(l, "-")
		leftN := ns[0]
		rightN := ns[1]

		// create them if the don't exist
		if _, ok := net[leftN]; !ok {
			net[leftN] = NewNode(leftN)
		}
		if _, ok := net[rightN]; !ok {
			net[rightN] = NewNode(rightN)
		}

		net[leftN].connect(net[rightN])
		net[rightN].connect(net[leftN])
	}

	return net, nil
}
