package main

/*
* ElfScript Brigade
*
* Advent Of Code 2024 Day 04
* Rust Solution
*
* Day 4: Ceres Search
*
* https://adventofcode.com/2024/day/4
*
 */

import (
	"fmt"
	"regexp"
	"runtime/debug"
	"strings"

	"github.com/luxedo/esb_fireplace-go"
)

type direction string

const (
	lu direction = "left-up"
	ld direction = "left-down"
	ru direction = "right-up"
	rd direction = "right-down"
)

// diagonal returns the diagonal string from the given starting point
func diagonal(lines []string, row int, column int, dir direction, length int) string {
	switch dir {
	case lu:
		if row < length-1 || column < length-1 {
			return ""
		}
	case ld:
		if row > len(lines)-length || column < 3 {
			return ""
		}
	case ru:
		if row < length-1 || column > len(lines[0])-length {
			return ""
		}
	case rd:
		if row > len(lines)-length || column > len(lines[0])-length {
			return ""
		}
	}

	runes := make([]rune, length)
	for i := 0; i < length; i++ {
		switch dir {
		case lu:
			runes = append(runes, rune(lines[row-i][column-i]))
		case ld:
			runes = append(runes, rune(lines[row+i][column-i]))
		case ru:
			runes = append(runes, rune(lines[row-i][column+i]))
		case rd:
			runes = append(runes, rune(lines[row+i][column+i]))
		}
	}

	return string(runes)
}

// down returns the string from the given starting point going down
func down(lines []string, row int, column int, length int) string {
	if row > len(lines[0])-length {
		return ""
	}

	runes := make([]rune, length)
	for i := 0; i < length; i++ {
		runes = append(runes, rune(lines[row+i][column]))
	}

	return string(runes)
}

// left returns the string from the given starting point going left
func left(lines []string, row int, column int, length int) string {
	if column < length {
		return ""
	}

	return lines[row][column-length : column]
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

// reverse returns the reversed string
func reverse(s string) string {
	runes := []rune(s)
	for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
		runes[i], runes[j] = runes[j], runes[i]
	}
	return string(runes)
}

// right returns the string from the given starting point going
func right(lines []string, row int, column int, length int) string {
	if column > len(lines[0])-length {
		return ""
	}

	return lines[row][column : column+length]
}

func search(lines []string) int {
	target := "XMAS"

	l := len(target)
	re := regexp.MustCompile(target)

	count := 0
	for r := range lines {
		for c := range lines[r] {

			// left
			if s := re.FindString(left(lines, r, c, l)); s != "" {
				count++
			}

			// right
			if s := re.FindString(right(lines, r, c, l)); s != "" {
				count++
			}

			// up
			if s := re.FindString(up(lines, r, c, l)); s != "" {
				count++
			}

			// down
			if s := re.FindString(down(lines, r, c, l)); s != "" {
				count++
			}

			// left-up
			if s := re.FindString(diagonal(lines, r, c, lu, l)); s != "" {
				count++
			}

			// left-down
			if s := re.FindString(diagonal(lines, r, c, ld, l)); s != "" {
				count++
			}

			// right-up
			if s := re.FindString(diagonal(lines, r, c, ru, l)); s != "" {
				count++
			}

			// right-down
			if s := re.FindString(diagonal(lines, r, c, rd, l)); s != "" {
				count++
			}
		}
	}

	return count
}

// up returns the string from the given starting point going up
func up(lines []string, row int, column int, length int) string {
	if row < length-1 {
		return ""
	}

	runes := make([]rune, length)
	for i := 0; i < length; i++ {
		runes = append(runes, rune(lines[row-i][column]))
	}

	return string(runes)
}

// solvePart1 solves the first part of the puzzle
func solvePart1(input_data string, args []string) (interface{}, error) {
	defer catch()
	lines := strings.Split(input_data, "\n")
	lines = removeEmptyStrings(lines)
	count := search(lines)
	return count, nil
}

// solvePart2 solves the second part of the puzzle
func solvePart2(input_data string, args []string) (interface{}, error) {
	defer catch()
	return "December", nil
}

func main() {
	// 🎅🎄❄️☃️🎁🦌
	// Bright christmas lights HERE
	esb_fireplace.V1Run(solvePart1, solvePart2)
}

func catch() {
	if r := recover(); r != nil {
		fmt.Println(r)
		debug.PrintStack()
	}
}
