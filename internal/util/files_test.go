package util

import (
	"errors"
	"fmt"
	"reflect"
	"strconv"
	"testing"
)

func Test_ReadInputLines(t *testing.T) {
	want := []string{"1", "2", "3"}

	got, err := ReadInputLines("files_sample.txt")
	if err != nil {
		t.Fatalf("failed to read input lines: %v", err)
	}

	if !reflect.DeepEqual(want, got) {
		t.Errorf("ReadInputLines() = %+v, want %+v", got, want)
	}
}

func Test_ParseInputLines(t *testing.T) {
	want := []int{1, 2, 3}

	got, err := ParseInputLines("files_sample.txt", strconv.Atoi)
	if err != nil {
		t.Fatalf("failed to parse input lines: %v", err)
	}

	if !reflect.DeepEqual(want, got) {
		t.Errorf("ParseInputLines() = %+v, want %+v", got, want)
	}
}

func Test_ParseInputLinesFailParse(t *testing.T) {
	parseErr := errors.New("no lines are valid")

	_, err := ParseInputLines("files_sample.txt", func(line string) (string, error) {
		return "", parseErr
	})

	if err == nil || !errors.Is(err, parseErr) {
		t.Fatalf("ParseInputLines should have thrown a parseErr, but err was %v", err)
	}
}

func Test_ParseInputLinesSections(t *testing.T) {
	type resultStruct struct {
		header string
		nums   []int
	}

	want := resultStruct{
		header: "header - file counts to 3",
		nums:   []int{1, 2, 3},
	}

	// file contains a header, a blank line, and numbers.
	parsers := map[string]SectionLineParser[resultStruct]{
		"header": func(line string, r *resultStruct) (string, error) {
			r.header = line

			return "parseNums", nil
		},

		"parseNums": func(line string, r *resultStruct) (string, error) {
			num, err := strconv.Atoi(line)
			if err != nil {
				return "", fmt.Errorf("failed to parse '%s' as a number: %v", line, err)
			}

			r.nums = append(r.nums, num)

			return "parseNums", nil
		},
	}

	got, err := ParseInputLinesSections("files_sample_sections.txt", "header", resultStruct{}, parsers)
	if err != nil {
		t.Fatalf("failed to parse input lines: %v", err)
	}

	if !reflect.DeepEqual(want, got) {
		t.Errorf("ParseInputLines() = %+v, want %+v", got, want)
	}
}
