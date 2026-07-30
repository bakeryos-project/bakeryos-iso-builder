package cmd

import (
	"path"

	"github.com/bakeryos-project/bakeryos-iso-builder/helpers"
	"github.com/bakeryos-project/bakeryos-iso-builder/helpers/repo"
	"github.com/bakeryos-project/bakeryos-iso-builder/logger"
	"github.com/spf13/cobra"
)

// prepareCmd represents the prepare command
var prepareCmd = &cobra.Command{
	Use:   "prepare",
	Short: "A brief description of your command",
	Long: `A longer description that spans multiple lines and likely contains examples
and usage of using your command. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
	Run: func(cmd *cobra.Command, args []string) {
		logger.Info("Updating ")
		helpers.RunCommandAndStream([]string{"sudo", "pacman", "-Syu"}, helpers.CurrentWorkingDir, path.Join(helpers.BuildLogDir, "setup.log"))
		helpers.RunCommandAndStream([]string{"sudo", "pacman", "-S", "--needed", "base-devel", "squashfs-tools", "dosfstools", "mtools", "arch-install-scripts", "xorriso"}, helpers.CurrentWorkingDir, path.Join(helpers.BuildLogDir, "setup.log"))
		logger.Info("Setting up filesystem")
		err := helpers.InitFilesystem()
		if err != nil {
			logger.Error(err.Error())
			return
		}

		logger.Info("Syncing testing repository")
		err = repo.SyncTestingRepo()
		if err != nil {
			logger.Error(err.Error())
			return
		}

		logger.Info("Build testing database")
		err = repo.BuildTestingRepoDB()
		if err != nil {
			logger.Error(err.Error())
			return
		}

	},
}

func init() {
	rootCmd.AddCommand(prepareCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// prepareCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// prepareCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
