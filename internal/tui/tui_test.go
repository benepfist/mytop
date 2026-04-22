package tui

import (
	"bufio"
	"bytes"
	"fmt"
	"io"
	"os"
	"regexp"
	"strings"
	"testing"
	"time"

	"mytop/internal/app"
)

type fakeDB struct {
	rows map[string][]map[string]string
	err  map[string]error
	exec []string
}

func (f *fakeDB) Hashes(query string) ([]map[string]string, error) {
	if e := f.err[query]; e != nil {
		return nil, e
	}
	if r, ok := f.rows[query]; ok {
		return r, nil
	}
	return []map[string]string{}, nil
}

func (f *fakeDB) Execute(query string) error {
	f.exec = append(f.exec, query)
	if e := f.err[query]; e != nil {
		return e
	}
	return nil
}

func captureOutput(t *testing.T, fn func()) string {
	t.Helper()
	old := os.Stdout
	r, w, _ := os.Pipe()
	os.Stdout = w
	fn()
	_ = w.Close()
	os.Stdout = old
	out, _ := io.ReadAll(r)
	return string(out)
}

func baseState() *app.RuntimeState {
	all := regexp.MustCompile(".*")
	return &app.RuntimeState{QCache: map[int64]string{}, UCache: map[int64]string{}, DBCache: map[int64]string{}, StatusPrev: map[string]int64{}, FilterUser: all, FilterDB: all, FilterHost: all}
}

func TestRunAndModes(t *testing.T) {
	db := &fakeDB{rows: map[string][]map[string]string{
		"SHOW GLOBAL STATUS":    {{"Variable_name": "Uptime", "Value": "10"}, {"Variable_name": "Questions", "Value": "20"}, {"Variable_name": "Slow_queries", "Value": "2"}},
		"SHOW FULL PROCESSLIST": {{"Id": "1", "User": "root", "db": "test", "Host": "localhost:3306", "Command": "Query", "Time": "2", "Info": "select 1"}},
		"SHOW STATUS LIKE \"Questions\"": {{"Variable_name": "Questions", "Value": "25"}},
		"SHOW GLOBAL STATUS LIKE 'Com_%'": {{"Variable_name": "Com_select", "Value": "4"}},
		"SHOW INNODB STATUS": {{"Status": "ok"}},
	}, err: map[string]error{}}

	tu := New()
	state := baseState()
	cfg := app.Config{BatchMode: true, Mode: app.ModeTop, Delay: 1, Header: true, Resolve: true, Idle: true}
	if err := tu.Init(cfg, state, db); err != nil {
		t.Fatal(err)
	}
	out := captureOutput(t, func() {
		if err := tu.Run(); err != nil {
			t.Fatalf("run failed: %v", err)
		}
	})
	if !strings.Contains(out, "Uptime") {
		t.Fatalf("expected top output, got %q", out)
	}

	for _, mode := range []app.Mode{app.ModeQPS, app.ModeCmd, app.ModeInnoDB, app.ModeStatus, "unknown"} {
		tu.cfg.Mode = mode
		_ = captureOutput(t, func() { _ = tu.runMode() })
	}
}

func TestReadCommandAndHandleInput(t *testing.T) {
	db := &fakeDB{rows: map[string][]map[string]string{
		"SHOW VARIABLES": {{"Variable_name": "version", "Value": "8.0"}},
		"EXPLAIN select 1": {{"id": "1", "type": "ALL"}},
	}, err: map[string]error{}}
	tu := New()
	st := baseState()
	st.QCache[1] = "select 1"
	st.DBCache[1] = "test"
	st.UCache[1] = "root"
	_ = tu.Init(app.Config{Mode: app.ModeTop, Delay: 1, Header: true, Resolve: true, Idle: true}, st, db)

	commands := "t\nm\nc\nI\nS\n?\nH\no\ni\np\nR\nF\nr\nV\n#\nD\n"
	tu.in = bufio.NewReader(strings.NewReader(commands))
	_ = captureOutput(t, func() {
		for i := 0; i < 16; i++ {
			if err := tu.readCommand(); err != nil {
				t.Fatalf("readCommand failed: %v", err)
			}
		}
	})

	inputs := []struct {
		cmd   string
		value string
	}{
		{"u", "alice\n"}, {"d", "prod\n"}, {"h", "api\n"}, {"k", "10\n"}, {"K", "root\n"}, {"f", "1\n"}, {"e", "1\n"}, {"s", "0\n"},
	}
	for _, in := range inputs {
		tu.in = bufio.NewReader(strings.NewReader(in.value))
		_ = captureOutput(t, func() {
			if err := tu.handleInputCommand(in.cmd); err != nil {
				t.Fatalf("%s failed: %v", in.cmd, err)
			}
		})
	}

	// invalid inputs
	for _, in := range []struct{ cmd, value string }{{"u", "/[/\n"}, {"k", "x\n"}, {"e", "x\n"}, {"s", "x\n"}} {
		tu.in = bufio.NewReader(strings.NewReader(in.value))
		_ = captureOutput(t, func() { _ = tu.handleInputCommand(in.cmd) })
	}
}

func TestHelpers(t *testing.T) {
	st := baseState()
	threads := parseThreads([]map[string]string{{"Id": "7", "User": "u", "db": "d", "Host": "h", "Command": "Query", "Time": "3", "Info": "select\n1"}}, st)
	if len(threads) != 1 || st.QCache[7] == "" {
		t.Fatalf("parseThreads failed")
	}
	m := toMap([]map[string]string{{"Variable_name": "Questions", "Value": "9"}, {"x": "y"}})
	if m["Questions"] != "9" {
		t.Fatalf("toMap failed")
	}
	if atoi64("bad") != 0 || fmt.Sprintf("%.1f", pct(1, 4)) != "25.0" || pct(1, 0) != 0 {
		t.Fatalf("numeric helpers failed")
	}
	if oneLine(" a\nb ") != "a b" {
		t.Fatalf("oneLine failed")
	}

	db := &fakeDB{rows: map[string][]map[string]string{"EXPLAIN select 1": {{"id": "1"}}}, err: map[string]error{}}
	tu := &TUI{cfg: app.Config{}, state: st, db: db}
	out := captureOutput(t, func() { tu.fullQueryInfo(999) })
	if !strings.Contains(out, "Invalid id") {
		t.Fatalf("expected invalid id")
	}
	st.QCache[1] = "select 1"
	st.DBCache[1] = "test"
	if err := tu.explain(1); err != nil {
		t.Fatalf("explain failed: %v", err)
	}
	if len(db.exec) == 0 || db.exec[0] != "USE test" {
		t.Fatalf("expected USE statement")
	}

	buf := &bytes.Buffer{}
	_ = captureOutput(t, func() { printTable([]map[string]string{{"a": "b"}}) })
	// make sure state timing branch gets touched
	tu.cfg = app.Config{Delay: 1, Header: true, Resolve: true, Idle: true}
	tu.state = baseState()
	tu.state.PrevAt = time.Now().Add(-time.Second)
	_ = buf
}
