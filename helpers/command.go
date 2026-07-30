package helpers

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"os/exec"
)

func RunCommandAndStream(command interface{}, cwd string, logFilePath string) int {
	var cmd *exec.Cmd

	switch c := command.(type) {
	case string:
		cmd = exec.Command("sh", "-c", c)
	case []string:
		if len(c) == 0 {
			fmt.Println("An error occurred while executing command: empty command slice")
			return -1
		}
		cmd = exec.Command(c[0], c[1:]...)
	default:
		fmt.Println("An error occurred while executing command: invalid command type")
		return -1
	}

	if cwd != "" {
		cmd.Dir = cwd
	}

	var logFile *os.File
	var err error
	if logFilePath != "" {
		logFile, err = os.OpenFile(logFilePath, os.O_CREATE|os.O_APPEND|os.O_WRONLY, 0644)
		if err != nil {
			fmt.Printf("An error occurred while opening log file: %s\n", err)
			return -1
		}
		defer logFile.Close()
	}

	stdoutPipe, err := cmd.StdoutPipe()
	if err != nil {
		fmt.Printf("An error occurred while creating stdout pipe: %s\n", err)
		return -1
	}
	cmd.Stderr = cmd.Stdout

	if err := cmd.Start(); err != nil {
		fmt.Printf("An error occurred while starting command: %s\n", err)
		return -1
	}

	reader := bufio.NewReader(stdoutPipe)
	for {
		b, err := reader.ReadByte()
		if err != nil {
			if err == io.EOF {
				break
			}
			break
		}

		os.Stdout.Write([]byte{b})
		if logFile != nil {
			logFile.Write([]byte{b})
		}
	}

	if err := cmd.Wait(); err != nil {
		if exitError, ok := err.(*exec.ExitError); ok {
			return exitError.ExitCode()
		}
		fmt.Printf("An error occurred while waiting for command: %s\n", err)
		return -1
	}

	return 0
}
