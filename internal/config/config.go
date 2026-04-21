package config

import (
	"bufio"
	"errors"
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"runtime"
	"strconv"
	"strings"

	"mytop/internal/app"
)

type Loader struct {
	home string
	in   *os.File
	out  *os.File
}

func NewLoader() *Loader {
	return &Loader{home: os.Getenv("HOME"), in: os.Stdin, out: os.Stdout}
}

func (l *Loader) defaults() app.Config {
	return app.Config{
		BatchMode: false,
		Color:     runtime.GOOS != "windows",
		DB:        "test",
		Delay:     5,
		Header:    true,
		Host:      "localhost",
		Idle:      true,
		LongNums:  false,
		Mode:      app.ModeTop,
		Pass:      "",
		Port:      3306,
		Prompt:    false,
		Resolve:   true,
		Socket:    "",
		SortDesc:  false,
		User:      "root",
	}
}

func (l *Loader) Load(args []string) (app.Config, error) {
	cfg := l.defaults()
	if err := l.loadConfigFile(&cfg); err != nil {
		return app.Config{}, err
	}
	if err := l.loadEnv(&cfg); err != nil {
		return app.Config{}, err
	}
	if err := l.loadFlags(&cfg, args); err != nil {
		return app.Config{}, err
	}
	if cfg.Host != "" && strings.Contains(cfg.Host, ":") {
		host, portText, ok := strings.Cut(cfg.Host, ":")
		if ok {
			parsedPort, err := strconv.Atoi(portText)
			if err == nil {
				cfg.Host = host
				cfg.Port = parsedPort
			}
		}
	}
	if cfg.Prompt {
		pass, err := app.PromptLine(l.in, l.out, "Password")
		if err != nil {
			return app.Config{}, err
		}
		cfg.Pass = pass
	}
	if cfg.Delay < 1 {
		cfg.Delay = 1
	}
	return cfg, nil
}

func (l *Loader) loadEnv(cfg *app.Config) error {
	if v := os.Getenv("MYTOP_HOST"); v != "" {
		cfg.Host = v
	}
	if v := os.Getenv("MYTOP_PORT"); v != "" {
		p, err := strconv.Atoi(v)
		if err != nil {
			return fmt.Errorf("invalid MYTOP_PORT: %w", err)
		}
		cfg.Port = p
	}
	if v := os.Getenv("MYTOP_USER"); v != "" {
		cfg.User = v
	}
	if v := os.Getenv("MYTOP_DB"); v != "" {
		cfg.DB = v
	}
	if v := os.Getenv("MYTOP_SOCKET"); v != "" {
		cfg.Socket = v
	}
	return nil
}

func (l *Loader) loadConfigFile(cfg *app.Config) error {
	if l.home == "" {
		return nil
	}
	path := filepath.Join(l.home, ".mytop")
	f, err := os.Open(path)
	if err != nil {
		if errors.Is(err, os.ErrNotExist) {
			return nil
		}
		return err
	}
	defer f.Close()

	s := bufio.NewScanner(f)
	for s.Scan() {
		line := strings.TrimSpace(s.Text())
		if line == "" || strings.HasPrefix(line, "#") {
			continue
		}
		k, v, ok := strings.Cut(line, "=")
		if !ok {
			continue
		}
		key := strings.ToLower(strings.TrimSpace(k))
		value := strings.TrimSpace(v)
		switch key {
		case "user":
			cfg.User = value
		case "pass", "password":
			cfg.Pass = value
		case "db", "database":
			cfg.DB = value
		case "host":
			cfg.Host = value
		case "port":
			if p, err := strconv.Atoi(value); err == nil {
				cfg.Port = p
			}
		case "socket":
			cfg.Socket = value
		case "delay":
			if p, err := strconv.Atoi(value); err == nil {
				cfg.Delay = p
			}
		case "batchmode", "batch":
			cfg.BatchMode = value == "1" || strings.EqualFold(value, "true")
		case "color":
			cfg.Color = value == "1" || strings.EqualFold(value, "true")
		case "resolve":
			cfg.Resolve = value == "1" || strings.EqualFold(value, "true")
		case "idle":
			cfg.Idle = value == "1" || strings.EqualFold(value, "true")
		}
	}
	return s.Err()
}

func (l *Loader) loadFlags(cfg *app.Config, args []string) error {
	fs := flag.NewFlagSet("mytop", flag.ContinueOnError)
	fs.BoolVar(&cfg.Color, "color", cfg.Color, "enable color")
	fs.BoolVar(&cfg.Color, "nocolor", cfg.Color, "disable color")
	fs.StringVar(&cfg.User, "u", cfg.User, "db user")
	fs.StringVar(&cfg.User, "user", cfg.User, "db user")
	fs.StringVar(&cfg.Pass, "p", cfg.Pass, "db password")
	fs.StringVar(&cfg.Pass, "pass", cfg.Pass, "db password")
	fs.StringVar(&cfg.DB, "d", cfg.DB, "database")
	fs.StringVar(&cfg.DB, "db", cfg.DB, "database")
	fs.StringVar(&cfg.Host, "h", cfg.Host, "host")
	fs.StringVar(&cfg.Host, "host", cfg.Host, "host")
	fs.IntVar(&cfg.Port, "P", cfg.Port, "port")
	fs.IntVar(&cfg.Delay, "s", cfg.Delay, "delay")
	fs.BoolVar(&cfg.BatchMode, "b", cfg.BatchMode, "batch mode")
	mode := string(cfg.Mode)
	fs.StringVar(&mode, "m", mode, "mode")
	fs.StringVar(&cfg.Socket, "S", cfg.Socket, "socket")
	fs.BoolVar(&cfg.Prompt, "prompt", cfg.Prompt, "password prompt")
	fs.BoolVar(&cfg.SortDesc, "sort", cfg.SortDesc, "sort reverse")
	fs.SetOutput(ioDiscard{})
	if err := fs.Parse(args); err != nil {
		return err
	}
	cfg.Mode = app.Mode(mode)
	if contains(args, "--nocolor") {
		cfg.Color = false
	}
	return nil
}

type ioDiscard struct{}

func (ioDiscard) Write(p []byte) (n int, err error) { return len(p), nil }

func contains(args []string, needle string) bool {
	for _, a := range args {
		if a == needle {
			return true
		}
	}
	return false
}

func BuildDSN(cfg app.Config) string {
	base := fmt.Sprintf("database=%s;mysql_read_default_group=mytop;", cfg.DB)
	if cfg.Socket != "" {
		return base + "mysql_socket=" + cfg.Socket
	}
	return base + fmt.Sprintf("host=%s;port=%d", cfg.Host, cfg.Port)
}
