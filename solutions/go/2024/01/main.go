package main

/*
* ElfScript Brigade
*
* Advent Of Code 2024 Day 01
* Rust Solution
*
* Day 1: Historian Hysteria
*
* https://adventofcode.com/2024/day/1
*
 */

import (
	"fmt"
	"math"
	"slices"
	"strconv"
	"strings"

	"github.com/luxedo/esb_fireplace-go"
)

// calcTotalDistance calculates the total distance between two sorted slices
func calcTotalDistance(left, right []int) int {
	total := 0
	for i := 0; i < len(left); i++ {
		distance := math.Abs(float64(right[i] - left[i]))
		total += int(distance)
	}
	return total
}

// getFrequency calculates the frequency of numbers in a slice
func getFrequency(slice []int) map[int]int {
	frequency := make(map[int]int)
	for _, n := range slice {
		frequency[n]++
	}
	return frequency
}

// getIDs extracts the IDs from the input data
func getIDs(input_data string) ([]int, []int) {
	left, right := []int{}, []int{}

	// Split the input data by the new line character
	lines := strings.Split(input_data, "\n")
	for _, line := range lines {
		ids := strings.Fields(line)
		if len(ids) != 2 {
			continue
		}

		l, err := strconv.Atoi(ids[0])
		if err != nil {
			fmt.Printf("Parsing left ID caused error: %s", err)
			continue
		}

		r, err := strconv.Atoi(ids[1])
		if err != nil {
			fmt.Printf("Parsing right ID caused error: %s", err)
			continue
		}

		left = append(left, l)
		right = append(right, r)
	}

	return left, right
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

// solvePart1 solves the first part of the puzzle
func solvePart1(input_data string, args []string) (any, error) {
	// Get the IDs from the input data
	left, right := getIDs(input_data)

	// Sort slices in ascending order
	slices.Sort(left)
	slices.Sort(right)

	// Calculate the distances between the two sorted slices
	total := calcTotalDistance(left, right)

	return total, nil
}

// solvePart2 solves the second part of the puzzle
func solvePart2(input_data string, args []string) (any, error) {
	// Get the IDs from the input data
	left, right := getIDs(input_data)

	// Calculate how often the right IDs appear
	freq := getFrequency(right)

	// Calculate the similarity score
	similarity := 0
	for _, id := range left {
		count := freq[id]
		similarity += count * id
	}

	return similarity, nil
}

func main() {
	// 🎅🎄❄️☃️🎁🦌
	// Bright christmas lights HERE
	esb_fireplace.V1Run(solvePart1, solvePart2)
}
