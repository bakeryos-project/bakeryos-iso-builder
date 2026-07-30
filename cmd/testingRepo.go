package cmd

import (
	"github.com/bakeryos-project/bakeryos-iso-builder/helpers/repo"
	"github.com/bakeryos-project/bakeryos-iso-builder/logger"
	"github.com/spf13/cobra"
)

// testingRepoCmd represents the testingRepo command
var testingRepoCmd = &cobra.Command{
	Use:   "testing-repo",
	Short: "A brief description of your command",
	Long: `A longer description that spans multiple lines and likely contains examples
and usage of using your command. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
	Run: func(cmd *cobra.Command, args []string) {
		syncVal, err := cmd.Flags().GetBool("sync")
		if err != nil {
			return
		}

		buildVal, err := cmd.Flags().GetBool("build")
		if err != nil {
			return
		}

		if syncVal {
			logger.Info("Syncing testing repository")
			err = repo.SyncTestingRepo()
			if err != nil {
				logger.Error(err.Error())
				return
			}
		}

		if buildVal {
			logger.Info("Build testing database")
			err = repo.BuildTestingRepoDB()
			if err != nil {
				logger.Error(err.Error())
				return
			}
		}
	},
}

func init() {
	rootCmd.AddCommand(testingRepoCmd)
	testingRepoCmd.Flags().BoolP("sync", "s", false, "Sync testing repo packages")
	testingRepoCmd.Flags().BoolP("build", "b", false, "Build database")
}
