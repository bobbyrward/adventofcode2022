package solver

import (
	"fmt"
	"io/ioutil"

	"github.com/bobbyrward/adventofcode2022/go/adventofcode/pkg/problems"
)

func Solve(input string, problem problems.Problem) error {
	inputBytes, err := ioutil.ReadFile("../../inputs/day01.txt")
	if err != nil {
		return err
	}

	solution, err := problem.Solve(string(inputBytes))

	if err != nil {
		return err
	}

	fmt.Printf("Solution: %s\n", solution)

	return nil
}
