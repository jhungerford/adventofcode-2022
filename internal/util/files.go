package util

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"path/filepath"
)

// LineParser parses a generic type from the given line.
type LineParser[T any] func(string) (T, error)

// StringLineParser returns the given line as-is.
func StringLineParser(line string) (string, error) {
	return line, nil
}

// ReadInputLines returns the raw lines in the input file.
func ReadInputLines(filename string) ([]string, error) {
	return ParseInputLines(filename, StringLineParser)
}

// ParseInputLines applies the parseLine function to each line in the input file.
func ParseInputLines[T any](filename string, parseLine LineParser[T]) ([]T, error) {
	sectionParsers := map[string]SectionLineParser[[]T]{
		"section": func(line string, t *[]T) (string, error) {
			parsed, err := parseLine(line)
			if err != nil {
				return "", err
			}

			*t = append(*t, parsed)

			return "section", nil
		},
	}

	var parsed []T

	return ParseInputLinesSections(filename, "section", parsed, sectionParsers)
}

// SectionLineParser takes a line and result, parses the line, and returns the next section name.
type SectionLineParser[T any] func(string, *T) (string, error)

// ParseInputLinesSections parses an input file that can contain multiple sections.  sectionParsers is a map of
// section name to a SectionLineParser, which parses a line and returns the name of the section to move to.
// Ignores blank lines.
func ParseInputLinesSections[T any](
	filename, firstSection string,
	t T,
	sectionParsers map[string]SectionLineParser[T],
) (T, error) {
	file, err := findInputFile(filename)
	if err != nil {
		return t, err
	}

	defer func(file *os.File) {
		cerr := file.Close()
		if cerr != nil {
			fmt.Printf("failed to close file: %v\n", cerr)
		}
	}(file)

	currentSection := firstSection

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()

		if line == "" {
			continue
		}

		sectionParser, ok := sectionParsers[currentSection]
		if !ok {
			return t, errors.New(fmt.Sprintf("invalid section - must be in sectionParsers: %s", currentSection))
		}

		currentSection, err = sectionParser(line, &t)
		if err != nil {
			return t, fmt.Errorf("failed to parse line '%s': %w", line, err)
		}
	}

	if scanner.Err() != nil {
		return t, fmt.Errorf("failed to read lines from file: %w", err)
	}

	return t, nil
}

// findInputFile locates the given input file, checking a few likely locations.
func findInputFile(filename string) (*os.File, error) {
	pwd, err := os.Getwd()
	if err != nil {
		return nil, fmt.Errorf("failed to get working dir: %w", err)
	}

	// IntelliJ uses the project root as the working directory for commands, and the internal directory
	// where the test is run for tests.  Check a couple of likely locations for input files.
	checkFiles := []string{
		// Project root for commands, test location for tests
		filepath.Join(pwd, filename),
		// Input from the project root
		filepath.Join(pwd, "input", filename),
		// Test working directory is two levels deep.
		filepath.Join(pwd, "..", "..", filename),
		// Input from the test working directory
		filepath.Join(pwd, "..", "..", "input", filename),
	}

	var ferr error
	var file *os.File

	for _, checkFile := range checkFiles {
		file, ferr = os.Open(checkFile)
		if ferr == nil {
			break
		}
	}

	if ferr != nil {
		return nil, fmt.Errorf("failed to open file: %w\nchecked: %+v", errors.Unwrap(ferr), checkFiles)
	}

	return file, nil
}
