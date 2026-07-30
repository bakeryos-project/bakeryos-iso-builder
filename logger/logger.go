package logger

import "fmt"

func Info(msg string) {
	fmt.Printf("[BakeryOS ISO] [Info]: %s\n", msg)
}

func Error(msg string) {
	fmt.Printf("[BakeryOS ISO] [Error]: %s\n", msg)
}
