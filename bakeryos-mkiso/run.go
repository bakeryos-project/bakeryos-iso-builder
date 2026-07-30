package bakeryos_mkiso

import (
	_ "embed"
	"fmt"
	"os"
	"path/filepath"

	"github.com/bakeryos-project/bakeryos-iso-builder/helpers"
)

//go:embed run.sh
var bashScript string

func RunMkIso() error {

	tmpFile, err := os.CreateTemp("", "bakeryos-run-*.sh")
	if err != nil {
		return err
	}
	defer os.Remove(tmpFile.Name())

	if _, err := tmpFile.WriteString(bashScript); err != nil {
		tmpFile.Close()
		return err
	}
	tmpFile.Close()

	if err := os.Chmod(tmpFile.Name(), 0755); err != nil {
		return err
	}

	err = helpers.BuildPacmanConfig()
	if err != nil {
		return err
	}

	cmdArgs := append(
		[]string{tmpFile.Name()},
		"-v",
		"-C",
		helpers.BuildPacmanConfigFile,
		"-w",
		helpers.WorkDir,
		"-o",
		helpers.OutDir,
		".",
	)
	exitCode := helpers.RunCommandAndStream(
		append([]string{"sudo", "bash"}, cmdArgs...),
		helpers.CurrentWorkingDir,
		filepath.Join(helpers.BuildLogDir, "build.log"),
	)

	if exitCode != 0 {
		return fmt.Errorf("Code: %d", exitCode)
	}

	return nil
}
