/*
Copyright © 2022 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"fmt"

	"github.com/spf13/cobra"

	"github.com/bobbyrward/adventofcode2022/go/adventofcode/pkg/problems/day01"
	"github.com/bobbyrward/adventofcode2022/go/adventofcode/pkg/solver"
)

// day01Cmd represents the day01 command
var day01Cmd = &cobra.Command{
	Use: "day01",
}

// problem01Cmd represents the problem01 command
var day01problem01Cmd = &cobra.Command{
	Use: "problem01",
	RunE: func(cmd *cobra.Command, args []string) error {
		err := solver.Solve("day01", day01.NewProblem01())
		if err != nil {
			fmt.Printf("Error: %v\n", err)
		}
		return err
	},
}

// problem01Cmd represents the problem01 command
var day01problem02Cmd = &cobra.Command{
	Use: "problem02",
	RunE: func(cmd *cobra.Command, args []string) error {
		err := solver.Solve("day01", day01.NewProblem02())
		if err != nil {
			fmt.Printf("Error: %v\n", err)
		}
		return err
	},
}

func init() {
	day01Cmd.AddCommand(day01problem01Cmd)
	day01Cmd.AddCommand(day01problem02Cmd)
	rootCmd.AddCommand(day01Cmd)
}
