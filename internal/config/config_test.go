package config

import (
	"os"
	"path/filepath"
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
}

func TestLoadConfigFile(t *testing.T) {
	dir := t.TempDir()
	p := filepath.Join(dir, ".mytop")
	_ = os.WriteFile(p, []byte("user=alice\nport=3310\n"), 0o644)
	l := NewLoader()
	l.home = dir
	cfg, err := l.Load(nil)
	if err != nil {
		t.Fatal(err)
	}
	if cfg.User != "alice" || cfg.Port != 3310 {
		t.Fatalf("unexpected cfg: %+v", cfg)
	}
}
