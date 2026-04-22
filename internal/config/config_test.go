package config

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestBuildDSN(t *testing.T) {
	cfg, err := NewLoader().Load([]string{"-h", "db.local:3307", "-d", "prod"})
	if err != nil {
		t.Fatal(err)
	}
	dsn := BuildDSN(cfg)
	if dsn != "database=prod;mysql_read_default_group=mytop;host=db.local;port=3307" {
		t.Fatalf("unexpected dsn: %s", dsn)
	}
	cfg.Socket = "/tmp/mysql.sock"
	if got := BuildDSN(cfg); got != "database=prod;mysql_read_default_group=mytop;mysql_socket=/tmp/mysql.sock" {
		t.Fatalf("unexpected socket dsn: %s", got)
	}
}

func TestLoadConfigFileAndEnvAndFlags(t *testing.T) {
	dir := t.TempDir()
	p := filepath.Join(dir, ".mytop")
	_ = os.WriteFile(p, []byte("user=alice\nport=3310\ndelay=0\ncolor=false\n#comment\ninvalid\n"), 0o644)

	t.Setenv("MYTOP_HOST", "env.local")
	t.Setenv("MYTOP_PORT", "3311")
	t.Setenv("MYTOP_USER", "env-user")
	t.Setenv("MYTOP_DB", "env-db")
	t.Setenv("MYTOP_SOCKET", "/tmp/sock")

	l := NewLoader()
	l.home = dir
	cfg, err := l.Load([]string{"--nocolor", "-s", "0", "-m", "cmd", "-b", "-h", "flag.local:3308"})
	if err != nil {
		t.Fatal(err)
	}
	if cfg.User != "env-user" || cfg.DB != "env-db" || cfg.Host != "flag.local" || cfg.Port != 3308 {
		t.Fatalf("unexpected cfg: %+v", cfg)
	}
	if cfg.Delay != 1 || cfg.Color || !cfg.BatchMode || string(cfg.Mode) != "cmd" {
		t.Fatalf("unexpected cfg values: %+v", cfg)
	}
}

func TestLoaderErrorsAndPrompt(t *testing.T) {
	t.Setenv("MYTOP_PORT", "invalid")
	if _, err := NewLoader().Load(nil); err == nil || !strings.Contains(err.Error(), "invalid MYTOP_PORT") {
		t.Fatalf("expected env error, got %v", err)
	}

	t.Setenv("MYTOP_PORT", "")
	l := NewLoader()
	l.in = os.Stdin
	l.out = os.Stdout
	if _, err := l.Load([]string{"-not-a-flag"}); err == nil {
		t.Fatalf("expected flags parse error")
	}

	l = NewLoader()
	l.in = os.NewFile(0, "/dev/stdin")
	l.out = os.Stdout
	// explicit prompt behavior with controlled input
	r, w, _ := os.Pipe()
	_, _ = w.Write([]byte("secret\n"))
	_ = w.Close()
	l.in = r
	cfg, err := l.Load([]string{"--prompt"})
	if err != nil {
		t.Fatalf("prompt load error: %v", err)
	}
	if cfg.Pass != "secret" {
		t.Fatalf("expected prompted password")
	}
}

func TestContainsAndIoDiscard(t *testing.T) {
	if !contains([]string{"a", "--nocolor"}, "--nocolor") || contains([]string{"a"}, "--nocolor") {
		t.Fatalf("contains failed")
	}
	if n, err := (ioDiscard{}).Write([]byte("x")); err != nil || n != 1 {
		t.Fatalf("ioDiscard write failed")
	}
}
