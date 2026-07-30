package repo

import (
	"fmt"
	"os"
	"path/filepath"

	"github.com/bakeryos-project/bakeryos-iso-builder/helpers"
	"github.com/bakeryos-project/bakeryos-iso-builder/logger"
)

func SyncTestingRepo() error {
	var err error
	err = helpers.CopyDir(
		helpers.TestingPackagesDir,
		helpers.TestingRepoDir,
	)

	if err != nil {
		return err
	}

	return err
}
func BuildTestingRepoDB() error {
	pattern := filepath.Join(helpers.TestingRepoDir, "*.pkg.tar.zst")
	pkgFiles, err := filepath.Glob(pattern)
	if err != nil {
		return err
	}

	if len(pkgFiles) > 0 {
		dbTempFile := helpers.TestingRepoDBFile + ".tar.zst"
		finalDBFile := helpers.TestingRepoDBFile

		command := append([]string{"repo-add", dbTempFile}, pkgFiles...)
		exitCode := helpers.RunCommandAndStream(command, "", "")
		if exitCode != 0 {
			return fmt.Errorf("repo-add failed with exit code: %d", exitCode)
		}

		err = os.Rename(dbTempFile, finalDBFile)
		if err != nil {
			return err
		}
	} else {
		logger.Info("No packages found. Skipped!")
	}

	return nil
}
