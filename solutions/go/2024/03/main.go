package main

/*
* ElfScript Brigade
*
* Advent Of Code 2024 Day 03
* Rust Solution
*
* Day 3: Mull It Over
*
* https://adventofcode.com/2024/day/3
*
 */

import (
	"regexp"
	"strconv"
	"strings"

	"github.com/luxedo/esb_fireplace-go"
)

// calculateMemory calculates the memory value from the input string
func calculateMemory(memory string) int {
	re := regexp.MustCompile(`\d+`)
	numbers := re.FindAllString(memory, -1)
	first, _ := strconv.Atoi(numbers[0])
	second, _ := strconv.Atoi(numbers[1])
	return (first * second)
}

// restoreMemory restores the memory from the corrupted string
func restoreMemory(corrupted string) []string {
	re := regexp.MustCompile(`do(?:n't)*\(\)|mul\(\d+,\d+\)`)
	return re.FindAllString(corrupted, -1)
}

// sanitizeMemory removes the disabled memory from the memory slice
func sanitizeMemory(memory []string) []string {
	enabled := true

	sanitized := []string{}
	for _, m := range memory {
		switch m {
		case "do()":
			enabled = true
			continue
		case "don't()":
			enabled = false
		}

		if !enabled {
			continue
		}

		sanitized = append(sanitized, m)
	}

	return sanitized
}

// solvePart1 solves the first part of the puzzle
func solvePart1(input_data string, args []string) (interface{}, error) {
	lines := strings.Split(input_data, "\n")

	restored := []string{}
	for _, line := range lines {
		memory := restoreMemory(line)
		restored = append(restored, memory...)
	}

	total := 0
	for _, m := range restored {
		sum := calculateMemory(m)
		total += sum
	}

	return total, nil
}

// solvePart2 solves the second part of the puzzle
func solvePart2(input_data string, args []string) (interface{}, error) {
	lines := strings.Split(input_data, "\n")

	restored := []string{}
	for _, line := range lines {
		memory := restoreMemory(line)
		restored = append(restored, memory...)
	}

	restored = sanitizeMemory(restored)

	total := 0
	for _, m := range restored {
		sum := calculateMemory(m)
		total += sum
	}

	return total, nil
}

func main() {
	// 🎅🎄❄️☃️🎁🦌
	// Bright christmas lights HERE
	esb_fireplace.V1Run(solvePart1, solvePart2)
}
