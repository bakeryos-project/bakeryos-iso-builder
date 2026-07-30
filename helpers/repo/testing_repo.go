package repo

import (
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
		command := append([]string{"repo-add", helpers.TestingRepoDBFile}, pkgFiles...)
		helpers.RunCommandAndStream(command, "", "")
	} else {
		logger.Info("No packages found. Skipped!")
	}

	return nil
}
