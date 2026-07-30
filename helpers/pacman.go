package helpers

import (
	"fmt"
	"path/filepath"

	"gopkg.in/ini.v1"
)

func BuildPacmanConfig() error {
	cfg, err := ini.Load(PacmanConfigFile)
	if err != nil {
		return err
	}

	if FileExists(TestingRepoDBFile) {
		testingSec, err := cfg.NewSection("testing")
		if err != nil {
			return err
		}

		_, err = testingSec.NewKey("SigLevel", "Never")
		if err != nil {
			return err
		}

		absPath, err := filepath.Abs(TestingRepoDir)
		if err != nil {
			return err
		}

		_, err = testingSec.NewKey("Server", fmt.Sprintf("file://%s", absPath))
		if err != nil {
			return err
		}
	}

	err = cfg.SaveTo(BuildPacmanConfigFile)
	if err != nil {
		return err
	}

	return nil
}
