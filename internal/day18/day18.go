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
	// cubeAxes is used to quickly look up whether a cube is on the outside, enclosedCubes
	cubeAxes := toAxes(cubes)
	enclosedCubes := map[Cube]interface{}{}

	// Lava cubes are enclosed
	for _, cube := range cubes {
		enclosedCubes[cube] = nil
	}

	// Check each cube's neighbor to determine if it's on the inside of a droplet.
	for _, cube := range cubes {
		for _, neighbor := range cube.neighbors() {
			if newEnclosed, ok := neighbor.isEnclosed(enclosedCubes, cubeAxes); ok {
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
func (c Cube) isEnclosed(enclosedCubes map[Cube]interface{}, cubeAxes axes) (map[Cube]interface{}, bool) {
	if _, ok := enclosedCubes[c]; ok {
		return nil, true
	}

	// Start with this cube, and work outwards.
	toCheck := []Cube{c}
	checked := map[Cube]interface{}{c: nil}

	for len(toCheck) > 0 {
		var check Cube

		check, toCheck = toCheck[0], toCheck[1:]

		if !check.hasBoundingCubes(cubeAxes) {
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
func (c Cube) hasBoundingCubes(cubeAxes axes) bool {
	// On each plane, the cube has other cube on either side if it isn't the last cube in the plane.
	zs, found := cubeAxes.xyPos[c.xy()]
	if !found {
		return false
	}

	if zi := sort.SearchInts(zs, c.z); zi == 0 || zi == len(zs) {
		return false
	}

	ys, found := cubeAxes.xzPos[c.xz()]
	if !found {
		return false
	}

	if yi := sort.SearchInts(ys, c.y); yi == 0 || yi == len(ys) {
		return false
	}

	xs, found := cubeAxes.yzPos[c.yz()]
	if !found {
		return false
	}

	if xi := sort.SearchInts(xs, c.x); xi == 0 || xi == len(xs) {
		return false
	}

	return true
}

// xy returns the xy axis of this cube.
func (c Cube) xy() xy {
	return xy{x: c.x, y: c.y}
}

// xz returns the xz axis of this cube.
func (c Cube) xz() xz {
	return xz{x: c.x, z: c.z}
}

// yz returns the yz axis of this cube.
func (c Cube) yz() yz {
	return yz{y: c.y, z: c.z}
}

type xy struct {
	x, y int
}

type xz struct {
	x, z int
}

type yz struct {
	y, z int
}

// axes contains maps of planes to a sorted list of cube positions on those planes.
type axes struct {
	xyPos map[xy][]int
	xzPos map[xz][]int
	yzPos map[yz][]int
}

func toAxes(cubes []Cube) axes {
	a := axes{
		xyPos: map[xy][]int{},
		xzPos: map[xz][]int{},
		yzPos: map[yz][]int{},
	}

	for _, cube := range cubes {
		a.xyPos[cube.xy()] = append(a.xyPos[cube.xy()], cube.z)
		a.xzPos[cube.xz()] = append(a.xzPos[cube.xz()], cube.y)
		a.yzPos[cube.yz()] = append(a.yzPos[cube.yz()], cube.x)
	}

	for _, value := range a.xyPos {
		slices.Sort(value)
	}

	for _, value := range a.xzPos {
		slices.Sort(value)
	}

	for _, value := range a.yzPos {
		slices.Sort(value)
	}

	return a
}
