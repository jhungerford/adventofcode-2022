package day18

import (
	"github.com/jhungerford/adventofcode-2022/internal/util"
	"slices"
	"sort"
	"strings"
)

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

// Part2 counts the number of sides of cubes that are on the outside of lava droplets.
func Part2(cubes []Cube) int {
	// axes is used to quickly look up whether a cube is on the outside
	axes := toAxes(cubes)

	// enclosedCubes contains the list of cubes that are known to be enclosed.  Lava droplets are enclosed to make
	// finding other enclosed pockets and adjacent sizes easier.
	enclosedCubes := map[Cube]interface{}{}

	// Lava cubes are enclosed
	for _, cube := range cubes {
		enclosedCubes[cube] = nil
	}

	// Check each cube's neighbor to determine if it's on the inside of a droplet.
	for _, cube := range cubes {
		for _, neighbor := range cube.neighbors() {
			if newEnclosed, ok := neighbor.isEnclosed(enclosedCubes, axes); ok {
				for newEnclosedCube, _ := range newEnclosed {
					enclosedCubes[newEnclosedCube] = nil
				}
			}
		}
	}

	// Calculate the number of exposed sides.  Since lava cubes are enclosed, adjacent cubes and pockets
	// won't be included in the exposed sides.
	exposedSides := 0

	for _, cube := range cubes {
		for _, neighbor := range cube.neighbors() {
			if _, enclosed := enclosedCubes[neighbor]; !enclosed {
				exposedSides++
			}
		}
	}

	return exposedSides
}

type Cube struct {
	x, y, z int
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

// isEnclosed returns whether this cube is enclosed, returning newly discovered enclosed cubes and whether this
// cube is enclosed.  If this cube was already known to be enclosed, no new cubes are discovered.
func (c Cube) isEnclosed(enclosedCubes map[Cube]interface{}, axes map[tangent][]int) (map[Cube]interface{}, bool) {
	if _, ok := enclosedCubes[c]; ok {
		return nil, true
	}

	// Start with this cube and work outwards from its neighbors.
	toCheck := []Cube{c}
	checked := map[Cube]interface{}{c: nil}

	for len(toCheck) > 0 {
		var check Cube

		check, toCheck = toCheck[0], toCheck[1:]

		if !check.hasBoundingCubes(axes) {
			return nil, false
		}

		for _, neighbor := range check.neighbors() {
			_, alreadyEnclosed := enclosedCubes[neighbor]
			_, alreadyChecked := checked[neighbor]

			if !alreadyEnclosed && !alreadyChecked {
				toCheck = append(toCheck, neighbor)
				checked[neighbor] = nil
			}
		}
	}

	return checked, true
}

// hasBoundingCubes returns whether there's at least one cube on all sides of this cube, possibly with empty space
// between this cube and the bound.
func (c Cube) hasBoundingCubes(axes map[tangent][]int) bool {
	for _, cubeAxis := range cubeAxes {
		tan, pos := cubeAxis(c)

		otherPositions, found := axes[tan]
		if !found {
			return false
		}

		// On each axis, the cube has other cubes on either side if it isn't the last cube in the tangent.
		if i := sort.SearchInts(otherPositions, pos); i == 0 || i == len(otherPositions) {
			return false
		}
	}

	return true
}

// getTangentPosition returns this cube's tangent plane and position along the tangent line.
type getTangentPosition func(c Cube) (tangent, int)

// xy returns the xy axis and z value of this cube.
func (c Cube) xy() (tangent, int) {
	return tangent{"xy", c.x, c.y}, c.z
}

// xz returns the xz axis and y value of this cube.
func (c Cube) xz() (tangent, int) {
	return tangent{"xz", c.x, c.z}, c.y
}

// yz returns the yz axis and x value of this cube.
func (c Cube) yz() (tangent, int) {
	return tangent{"yz", c.y, c.z}, c.x

}

// tangent is a named plane and position on that plane.
type tangent struct {
	name string
	a, b int
}

// cubeAxes is a list of functions that can extract a tangent plane and position from a cube.
var cubeAxes = []getTangentPosition{Cube.xy, Cube.xz, Cube.yz}

// toAxes converts the list of cubes to a map of tangents to a list of sorted cube values along the tangent line.
// The sorted cube values are useful for quickly checking whether a cube is between other cubes along an axis.
func toAxes(cubes []Cube) map[tangent][]int {
	axes := map[tangent][]int{}

	for _, cubeAxis := range cubeAxes {
		for _, cube := range cubes {
			tan, pos := cubeAxis(cube)

			axes[tan] = append(axes[tan], pos)
		}
	}

	for _, positions := range axes {
		slices.Sort(positions)
	}

	return axes
}
