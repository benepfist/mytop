package main

import (
	"os"
	"testing"
)

func TestMainBatchMode(t *testing.T) {
	oldArgs := os.Args
	defer func() { os.Args = oldArgs }()
	os.Args = []string{"mytop", "-b", "-s", "1"}
	main()
}
