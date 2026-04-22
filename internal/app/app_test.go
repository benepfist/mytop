package app

import (
	"bytes"
	"errors"
	"io"
	"regexp"
	"strings"
	"testing"
)

type stubLoader struct {
	cfg Config
	err error
}

func (s stubLoader) Load(_ []string) (Config, error) { return s.cfg, s.err }

type stubUI struct {
	initErr error
	runErr  error
	inited  bool
	run     bool
}

func (s *stubUI) Init(_ Config, _ *RuntimeState, _ DB) error { s.inited = true; return s.initErr }
func (s *stubUI) Run() error                                { s.run = true; return s.runErr }

type stubDB struct{}

func (stubDB) Hashes(string) ([]map[string]string, error) { return nil, nil }
func (stubDB) Execute(string) error                        { return nil }

type errReader struct{}

func (errReader) Read([]byte) (int, error) { return 0, errors.New("boom") }

func TestAppRun(t *testing.T) {
	u := &stubUI{}
	a := New(stubLoader{cfg: Config{}}, u, stubDB{})
	if err := a.Run(nil); err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !u.inited || !u.run {
		t.Fatalf("expected ui init and run")
	}

	a = New(stubLoader{err: errors.New("bad cfg")}, &stubUI{}, stubDB{})
	if err := a.Run(nil); err == nil || !strings.Contains(err.Error(), "load config") {
		t.Fatalf("expected load config error, got %v", err)
	}

	a = New(stubLoader{cfg: Config{}}, &stubUI{initErr: errors.New("bad init")}, stubDB{})
	if err := a.Run(nil); err == nil || !strings.Contains(err.Error(), "init ui") {
		t.Fatalf("expected init ui error, got %v", err)
	}

	a = New(stubLoader{cfg: Config{}}, &stubUI{runErr: errors.New("bad run")}, stubDB{})
	if err := a.Run(nil); err == nil || !strings.Contains(err.Error(), "run ui") {
		t.Fatalf("expected run ui error, got %v", err)
	}
}

func TestStringOrRegex(t *testing.T) {
	re, err := StringOrRegex("/foo.*/")
	if err != nil || !re.MatchString("foobar") {
		t.Fatalf("expected regex to match")
	}
	re, _ = StringOrRegex("admin")
	if !re.MatchString("admin") || re.MatchString("administrator") {
		t.Fatalf("expected exact match")
	}
	re, _ = StringOrRegex("   ")
	if !re.MatchString("anything") {
		t.Fatalf("empty input should match all")
	}
}

func TestThreadHelpers(t *testing.T) {
	state := &RuntimeState{
		FilterUser: regexp.MustCompile("^alice$"),
		FilterDB:   regexp.MustCompile("^prod$"),
		FilterHost: regexp.MustCompile("^api"),
	}
	threads := []Thread{{ID: 2, User: "alice", DB: "prod", Host: "api.local", Command: "Query", Time: 9}, {ID: 1, User: "alice", DB: "prod", Host: "api.local", Command: "Sleep", Time: 1}, {ID: 3, User: "bob", DB: "prod", Host: "api.local", Command: "Query", Time: 3}}
	filtered := FilterThreads(threads, state, false)
	if len(filtered) != 1 || filtered[0].ID != 2 {
		t.Fatalf("unexpected filtered: %+v", filtered)
	}

	SortThreads(threads, false)
	if threads[0].ID != 1 {
		t.Fatalf("expected asc sort")
	}
	SortThreads(threads, true)
	if threads[0].ID != 2 {
		t.Fatalf("expected desc sort")
	}
}

func TestUtilityFunctions(t *testing.T) {
	if got := NormalizeHost("db.prod.local:3306", false); got != "db.prod" {
		t.Fatalf("got %s", got)
	}
	if got := NormalizeHost("db.prod.local:3306", true); got != "db.prod.local" {
		t.Fatalf("got %s", got)
	}
	if Sum(1, 2, 3) != 6 {
		t.Fatalf("sum mismatch")
	}
	if Commify(-1234567) != "-1,234,567" {
		t.Fatalf("commify mismatch")
	}
	if got := MakeShort(1500, false); got != "1.5k" {
		t.Fatalf("got %s", got)
	}
	if got := MakeShort(1200, true); got != "1,200" {
		t.Fatalf("got %s", got)
	}
}

func TestTerminalAndPromptHelpers(t *testing.T) {
	var b bytes.Buffer
	Clear(&b)
	if b.Len() == 0 {
		t.Fatalf("expected clear output")
	}

	t.Setenv("PATH", "")
	if FindProg("definitely-not-installed") != "" {
		t.Fatalf("expected missing program")
	}
	if Pager() != "" {
		t.Fatalf("expected no pager in empty PATH")
	}

	out := &bytes.Buffer{}
	line, err := PromptLine(strings.NewReader("secret\n"), out, "Password")
	if err != nil || line != "secret" {
		t.Fatalf("prompt failed: %q %v", line, err)
	}
	if _, err := PromptLine(errReader{}, io.Discard, "Password"); err == nil {
		t.Fatalf("expected prompt read error")
	}
}

func TestHelpDumpAndDelay(t *testing.T) {
	if !strings.Contains(PrintHelp(Config{Color: true}), "\033[1;36m") {
		t.Fatalf("expected colorized help")
	}
	if strings.Contains(PrintHelp(Config{Color: false}), "\033[1;36m") {
		t.Fatalf("expected plain help")
	}
	dump := DumpConfig(Config{BatchMode: true, Color: true, DB: "prod", Delay: 2, Header: true, Host: "localhost", Idle: true, Mode: ModeTop, Port: 3306, Resolve: true, Socket: "/tmp/mysql.sock", User: "root"})
	if !strings.Contains(dump, "db=prod") || !strings.Contains(dump, "socket=/tmp/mysql.sock") {
		t.Fatalf("unexpected dump: %s", dump)
	}
	cfg := Config{Delay: 5}
	CmdSetDelay(&cfg, 0)
	if cfg.Delay != 1 {
		t.Fatalf("expected delay floor")
	}
}
