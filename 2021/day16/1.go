package day16

func Day161() (int, error) {
	s, err := getInput()
	if err != nil {
		return 0, err
	}

	root, _ := NewBitstream(s).parse()

	return root.VersionSum(), nil
}
