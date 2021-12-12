package day12

func Day121() (int, error) {
	network, err := getInput()
	if err != nil {
		return 0, err
	}

	allPaths := recurPath([][]*Node{}, make(map[string]int), []*Node{}, network["start"], canBeVisited21)

	return len(allPaths), nil
}

func canBeVisited21(n *Node, visited map[string]int) bool {
	if n.caveType == big {
		return true
	}

	if _, ok := visited[n.name]; !ok {
		return true
	}

	return false
}
