package day18

import (
	"github.com/jhungerford/adventofcode-2022/internal/util"
	"strings"
)

type Cube struct {
	x, y, z int
}

// LoadCubes loads cubes from the given input file.
func LoadCubes(inputFile string) ([]Cube, error) {
	return util.ParseInputLines(inputFile, func(line string) (Cube, error) {
		// Cube looks like '2,2,2'

		parts := strings.Split(line, ",")

		return Cube{
			x: util.MustAtoi(parts[0]),
			y: util.MustAtoi(parts[1]),
			z: util.MustAtoi(parts[2]),
		}, nil
	})
}

// Part1 counts the number of sides of cubes that aren't connected to other cubes.
func Part1(cubes []Cube) int {
	cubeMap := make(map[Cube]interface{}, len(cubes))
	for _, cube := range cubes {
		cubeMap[cube] = nil
	}

	notConnected := 0

	for _, cube := range cubes {
		for _, neighbor := range cube.neighbors() {
			if _, hasNeighbor := cubeMap[neighbor]; !hasNeighbor {
				notConnected++
			}
		}
	}

	return notConnected
}

// neighbors returns the cubes to each side of this cube.
func (c Cube) neighbors() []Cube {
	moves := []Cube{
		{-1, 0, 0},
		{1, 0, 0},
		{0, -1, 0},
		{0, 1, 0},
		{0, 0, -1},
		{0, 0, 1},
	}

	neighbors := make([]Cube, 0, len(moves))

	for _, move := range moves {
		neighbors = append(neighbors, Cube{
			x: c.x + move.x,
			y: c.y + move.y,
			z: c.z + move.z,
		})
	}

	return neighbors
}
