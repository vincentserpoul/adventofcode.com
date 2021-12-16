package day16

func Day162() (int, error) {
	s, err := getInput()
	if err != nil {
		return 0, err
	}

	root, _ := NewBitstream(s).parse()

	return root.Eval(), nil
}
