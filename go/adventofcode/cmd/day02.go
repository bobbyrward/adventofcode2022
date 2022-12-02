/*
Copyright © 2022 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"fmt"

	"github.com/spf13/cobra"

	"github.com/bobbyrward/adventofcode2022/go/adventofcode/pkg/problems/day02"
	"github.com/bobbyrward/adventofcode2022/go/adventofcode/pkg/solver"
)

// day02Cmd represents the day02 command
var day02Cmd = &cobra.Command{
	Use: "day02",
}

// problem01Cmd represents the problem01 command
var day02problem01Cmd = &cobra.Command{
	Use: "problem01",
	RunE: func(cmd *cobra.Command, args []string) error {
		err := solver.Solve("day02", day02.NewProblem01())
		if err != nil {
			fmt.Printf("Error: %v\n", err)
		}
		return err
	},
}

// problem01Cmd represents the problem01 command
var day02problem02Cmd = &cobra.Command{
	Use: "problem02",
	RunE: func(cmd *cobra.Command, args []string) error {
		err := solver.Solve("day02", day02.NewProblem02())
		if err != nil {
			fmt.Printf("Error: %v\n", err)
		}
		return err
	},
}

func init() {
	day02Cmd.AddCommand(day02problem01Cmd)
	day02Cmd.AddCommand(day02problem02Cmd)
	rootCmd.AddCommand(day02Cmd)
}
