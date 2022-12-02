package problems

type Problem interface {
	Solve(string) (string, error)
}
