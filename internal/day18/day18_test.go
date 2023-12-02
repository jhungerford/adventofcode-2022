package day18

import (
	"slices"
	"testing"
)

func TestPart1(t *testing.T) {
	t.Parallel()

	tests := []struct {
		file        string
		want        int
		removeCubes []Cube
	}{
		{"day18_sample.txt", 64, nil},
		{"day18_sample2.txt", 96, nil},
		{"day18_sample2.txt", 92, []Cube{{3, 1, 0}}},
	}

	for _, tt := range tests {
		cubes, err := LoadCubes(tt.file)
		if err != nil {
			t.Fatalf("failed to load cubes from %s: %v", tt.file, err)
		}

		for _, remove := range tt.removeCubes {
			cubes = slices.DeleteFunc(cubes, func(c Cube) bool {
				return c == remove
			})
		}

		if got := Part1(cubes); got != tt.want {
			t.Errorf("Part1(%s) = %v, want %v", tt.file, got, tt.want)
		}
	}
}

func TestPart2(t *testing.T) {
	t.Parallel()

	tests := []struct {
		file        string
		want        int
		removeCubes []Cube
	}{
		{"day18_sample.txt", 58, nil},
		{"day18_sample2.txt", 74, nil},
		{"day18_sample2.txt", 92, []Cube{{3, 1, 0}}},
	}

	for _, tt := range tests {
		cubes, err := LoadCubes(tt.file)
		if err != nil {
			t.Fatalf("failed to load cubes from %s: %v", tt.file, err)
		}

		for _, remove := range tt.removeCubes {
			cubes = slices.DeleteFunc(cubes, func(c Cube) bool {
				return c == remove
			})
		}

		if got := Part2(cubes); got != tt.want {
			t.Errorf("Part2(%s, remove %+v) = %v, want %v", tt.file, tt.removeCubes, got, tt.want)
		}
	}
}

func Test_hasBoundingCubes(t *testing.T) {
	t.Parallel()

	cubes, err := LoadCubes("day18_sample2.txt")
	if err != nil {
		t.Fatalf("failed to load cubes from: %v", err)
	}

	cube := Cube{x: 2, y: 3, z: 0}

	if hasBounds := cube.hasBoundingCubes(toAxes(cubes)); !hasBounds {
		t.Errorf("Cube %+v should have bounds", cube)
	}
}
