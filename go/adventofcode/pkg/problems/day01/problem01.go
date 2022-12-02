package day01

import (
	"fmt"
	"strconv"
	"strings"
)

type Problem01 struct{}

func NewProblem01() *Problem01 {
	return &Problem01{}
}

type Problem02 struct{}

func NewProblem02() *Problem02 {
	return &Problem02{}
}

type elf []int

func sum(x []int) int {
	s := 0

	for _, i := range x {
		s += i
	}

	return s

}

func (e elf) Calories() int {
	return sum(e)
}

func (e elf) Inventory() []int {
	return e
}

func parseInventories(input string) ([]elf, error) {
	var elves []elf
	currentElf := elf{}

	for _, line := range strings.Split(strings.TrimSpace(input), "\n") {
		if line != "" {
			calories, err := strconv.Atoi(line)
			if err != nil {
				return nil, err
			}

			currentElf = append(currentElf, calories)
		} else {
			elves = append(elves, currentElf)
			currentElf = elf{}
		}
	}

	if len(currentElf) != 0 {
		elves = append(elves, currentElf)
	}

	return elves, nil
}

func findHighestCalories(inventories []elf) int {
	max := 0

	for _, e := range inventories {
		c := e.Calories()

		if c > max {
			max = c
		}
	}

	return max
}

func findTop3HighestCalories(inventories []elf) []int {
	max := []int{0, 0, 0}

	for _, e := range inventories {
		c := e.Calories()

		if c > max[0] {
			max[2] = max[1]
			max[1] = max[0]
			max[0] = c
			continue
		}

		if c > max[1] {
			max[2] = max[1]
			max[1] = c
			continue
		}

		if c > max[2] {
			max[2] = c
			continue
		}
	}

	return max
}

func (p *Problem01) Solve(input string) (string, error) {
	inventories, err := parseInventories(input)

	return fmt.Sprintf("%d", findHighestCalories(inventories)), err
}

func (p *Problem02) Solve(input string) (string, error) {
	inventories, err := parseInventories(input)

	return fmt.Sprintf("%d", sum(findTop3HighestCalories(inventories))), err
}
