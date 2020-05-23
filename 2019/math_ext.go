package adventofcode

// Least common multiple (multi-args)
func lcmm(args []int) int {
	if l := len(args); l < 0 {
		return 0
	} else if l := len(args); l < 2 {
		return args[0]
	} else if l == 2 {
		return lcm(args[0], args[1])
	}

	r := lcm(args[0], args[1])
	for i := 2; i < len(args); i++ {
		r = lcm(r, args[i])
	}
	return r
}

// Least common multiple
func lcm(a, b int) int {
	return (a / gcd(a, b)) * b
}

// Greatest common divider
func gcdm(arr []int) int {
	if len(arr) < 2 {
		return -1 //err
	} else if len(arr) == 2 {
		return gcd(arr[0], arr[1])
	}

	return gcdm(append([]int{gcd(arr[0], arr[1])}, arr[2:]...))
}

func gcd(a, b int) int {
	if b == 0 {
		return a
	}

	return gcd(b, a%b)
}
