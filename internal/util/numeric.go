package util

import "strconv"

// MustAtoi runs strconv.Atoi(s), panicing if the string isn't a number.
func MustAtoi(s string) int {
	i, err := strconv.Atoi(s)
	if err != nil {
		panic(err)
	}

	return i
}
