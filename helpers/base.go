package helpers

import (
	"log"
	"os"
	"path"
)

var CurrentWorkingDir string
var WorkDir string
var OutDir string
var PacmanConfigFile string
var BuildDir string
var BuildLogDir string
var BuildPacmanConfigFile string
var PackageCacheDir string
var TestingRepoDir string
var TestingRepoDBFile string
var TestingPackagesDir string
var SkelDir string

func init() {
	var err error
	CurrentWorkingDir, err = os.Getwd()
	if err != nil {
		log.Fatal(err)
		return
	}

	WorkDir = path.Join(CurrentWorkingDir, "work")
	OutDir = path.Join(CurrentWorkingDir, "out")
	PacmanConfigFile = path.Join(CurrentWorkingDir, "pacman.conf")

	BuildDir = path.Join(CurrentWorkingDir, "build")
	BuildLogDir = path.Join(BuildDir, "logs")
	BuildPacmanConfigFile = path.Join(BuildDir, "pacman.conf")

	PackageCacheDir = path.Join(BuildDir, "packages", "cache")
	TestingRepoDir = path.Join(BuildDir, "repo", "testing")
	TestingRepoDBFile = path.Join(TestingRepoDir, "testing.db")

	TestingPackagesDir = path.Join(CurrentWorkingDir, "testing", "packages")
	SkelDir = path.Join(CurrentWorkingDir, "airootfs", "etc", "skel")

	err = os.MkdirAll(BuildDir, 0755)
	err = os.MkdirAll(BuildLogDir, 0755)
	err = os.MkdirAll(PackageCacheDir, 0755)
	err = os.MkdirAll(TestingPackagesDir, 0755)
}

func InitFilesystem() error {
	var err error

	err = os.MkdirAll(BuildDir, 0755)
	if err != nil {
		return err
	}

	err = os.MkdirAll(BuildLogDir, 0755)
	if err != nil {
		return err
	}

	err = os.MkdirAll(PackageCacheDir, 0755)
	if err != nil {
		return err
	}

	err = os.MkdirAll(TestingPackagesDir, 0755)
	if err != nil {
		return err
	}

	return nil
}
