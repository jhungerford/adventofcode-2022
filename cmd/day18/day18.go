package main

import (
	"fmt"
	"github.com/jhungerford/adventofcode-2022/internal/day18"
)

func main() {
	cubes, err := day18.LoadCubes("input/day18.txt")
	if err != nil {
		fmt.Printf("failed to load cubes: %v", err)
		return
	}

	fmt.Println("Part 1:", day18.Part1(cubes))
}
