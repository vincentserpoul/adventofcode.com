package day12

func Day122() (int, error) {
	network, err := getInput()
	if err != nil {
		return 0, err
	}

	allPaths := recurPath([][]*Node{}, make(map[string]int), []*Node{}, network["start"], canBeVisited22)

	// for _, path := range allPaths {
	// 	fmt.Println(strings.Join(StringPath(path), ","))
	// }

	return len(allPaths), nil
}

func canBeVisited22(n *Node, visited map[string]int) bool {
	if n.caveType == big {
		return true
	}

	count, ok := visited[n.name]
	if !ok {
		return true
	}

	if n.name == "start" || n.name == "end" {
		return false
	}

	for vn, v := range visited {
		if v == 2 && caveTypeFromString(vn) == small {
			return false
		}
	}

	return count == 1
}
