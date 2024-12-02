package main

/*
* ElfScript Brigade
*
* Advent Of Code 2024 Day 02
* Rust Solution
*
* Day 2: Red-Nosed Reports
*
* https://adventofcode.com/2024/day/2
*
 */

import (
	"math"
	"strconv"
	"strings"

	"github.com/luxedo/esb_fireplace-go"
)

// checkSafety checks if the given report is safe
func checkSafety(report []int) bool {
	var inc, dec bool

	for i := 0; i < len(report)-1; i++ {
		diff := report[i] - report[i+1]
		switch {
		case diff == 0:
			return false
		case math.Abs(float64(diff)) > 3:
			return false
		case diff > 0:
			inc = true
		case diff < 0:
			dec = true
		}

		if inc && dec {
			return false
		}
	}

	return true
}

func copyReport(report []int) []int {
	cpy := make([]int, len(report))
	copy(cpy, report)
	return cpy
}

// parseInputs converts the input data to a more useful format
func parseInputs(input_data string) [][]int {
	lines := strings.Split(input_data, "\n")
	lines = removeEmptyStrings(lines)

	data := make([][]int, len(lines))
	for i, line := range lines {
		values := strings.Fields(line)
		data[i] = stringsToInts(values)
	}

	return data
}

// removeEmptyStrings removes empty strings from a slice of strings
func removeEmptyStrings(slice []string) []string {
	// Remove empty strings from the slice
	var result []string
	for _, str := range slice {
		if str != "" && str != " " {
			result = append(result, str)
		}
	}
	return result
}

func removeIndex(s []int, index int) []int {
	return append(s[:index], s[index+1:]...)
}

func sanitizeReport(report []int) []int {
	var inc, dec bool

	for i := 0; i < len(report)-1; i++ {
		diff := report[i] - report[i+1]
		switch {
		case diff == 0:
			return append(report[:i], report[i+1:]...)
		case math.Abs(float64(diff)) > 3:
			return append(report[:i], report[i+1:]...)
		case diff > 0:
			inc = true
		case diff < 0:
			dec = true
		}

		if inc && dec {
			return append(report[:i], report[i+1:]...)
		}
	}

	return report
}

// stringsToInts converts a slice of strings to a slice of ints
func stringsToInts(strings []string) []int {
	ints := make([]int, len(strings))
	for i, s := range strings {
		ints[i], _ = strconv.Atoi(s)
	}
	return ints
}

// solvePart1 solves the first part of the puzzle
func solvePart1(input_data string, args []string) (interface{}, error) {
	// parse input data into separate reports
	reports := parseInputs(input_data)

	// Count the number of reports that are considered "safe"
	count := 0
	for _, report := range reports {
		if checkSafety(report) {
			count++
		}
	}

	return count, nil
}

// solvePart2 solves the second part of the puzzle
func solvePart2(input_data string, args []string) (interface{}, error) {
	// parse input data into separate reports
	reports := parseInputs(input_data)

	// Count the number of reports that are considered "safe"
	count := 0
	for _, report := range reports {
		// Check if report is already safe
		if checkSafety(report) {
			count++
			continue
		}

		// Try removing one element at a time and check if the report is safe
		for i := 0; i < len(report); i++ {
			cpy := copyReport(report)
			cpy = removeIndex(cpy, i)
			if checkSafety(cpy) {
				count++
				break
			}
		}
	}

	return count, nil
}

func main() {
	// 🎅🎄❄️☃️🎁🦌
	// Bright christmas lights HERE
	esb_fireplace.V1Run(solvePart1, solvePart2)
}
