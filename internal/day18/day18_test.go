package day18

import "testing"

func TestPart1(t *testing.T) {
	t.Parallel()

	cubes, err := LoadCubes("day18_sample.txt")
	if err != nil {
		t.Fatalf("failed to load cubes: %v", err)
	}

	want := 64

	if got := Part1(cubes); got != want {
		t.Errorf("Part1() = %v, want %v", got, want)
	}
}
