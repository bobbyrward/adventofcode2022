package day02

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

const testInput = `1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
`

func TestDay01(t *testing.T) {
	elves, err := parseInventories(testInput)
	if assert.NoError(t, err) {
		if assert.Len(t, elves, 5) {
			assert.Len(t, elves[0], 3)
			assert.Equal(t, elves[0].Calories(), 6000)

			assert.Len(t, elves[1], 1)
			assert.Equal(t, elves[1].Calories(), 4000)

			assert.Len(t, elves[2], 2)
			assert.Equal(t, elves[2].Calories(), 11000)

			assert.Len(t, elves[3], 3)
			assert.Equal(t, elves[3].Calories(), 24000)

			assert.Len(t, elves[4], 1)
			assert.Equal(t, elves[4].Calories(), 10000)

			assert.Equal(t, findHighestCalories(elves), 24000)
			assert.Equal(t, findTop3HighestCalories(elves), []int{24000, 11000, 10000})
			assert.Equal(t, sum(findTop3HighestCalories(elves)), 45000)
		}
	}
}
