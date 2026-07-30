package helpers

func CleanUpAfterBuild() error {
	return DeleteItems([]string{
		"work",
		"out",
	})
}
