package app

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"maps"
	"os"
	"os/exec"
	"regexp"
	"runtime"
	"slices"
	"sort"
	"strconv"
	"strings"
	"time"
)

type Mode string

const (
	ModeTop    Mode = "top"
	ModeQPS    Mode = "qps"
	ModeCmd    Mode = "cmd"
	ModeInnoDB Mode = "innodb"
	ModeStatus Mode = "status"
)

type Config struct {
	BatchMode bool
	Color     bool
	DB        string
	Delay     int
	Header    bool
	Help      bool
	Host      string
	Idle      bool
	LongNums  bool
	Mode      Mode
	Pass      string
	Port      int
	Prompt    bool
	Resolve   bool
	Socket    string
	SortDesc  bool
	User      string
}

type Thread struct {
	ID      int64
	User    string
	DB      string
	Host    string
	Command string
	Time    int64
	Info    string
}

type RuntimeState struct {
	QCache     map[int64]string
	UCache     map[int64]string
	DBCache    map[int64]string
	StatusPrev map[string]int64
	PrevAt     time.Time
	Paused     bool
	FilterUser *regexp.Regexp
	FilterDB   *regexp.Regexp
	FilterHost *regexp.Regexp
}

type DB interface {
	Hashes(query string) ([]map[string]string, error)
	Execute(query string) error
}

type ConfigLoader interface {
	Load(args []string) (Config, error)
}

type UI interface {
	Init(cfg Config, state *RuntimeState, db DB) error
	Run() error
}

type App struct {
	cfgLoader ConfigLoader
	ui        UI
	db        DB
}

func New(cfgLoader ConfigLoader, ui UI, db DB) *App {
	return &App{cfgLoader: cfgLoader, ui: ui, db: db}
}

func (a *App) Run(args []string) error {
	cfg, err := a.cfgLoader.Load(args)
	if err != nil {
		return fmt.Errorf("load config: %w", err)
	}

	state := &RuntimeState{
		QCache:     map[int64]string{},
		UCache:     map[int64]string{},
		DBCache:    map[int64]string{},
		StatusPrev: map[string]int64{},
		FilterUser: regexp.MustCompile(".*"),
		FilterDB:   regexp.MustCompile(".*"),
		FilterHost: regexp.MustCompile(".*"),
	}

	if err := a.ui.Init(cfg, state, a.db); err != nil {
		return fmt.Errorf("init ui: %w", err)
	}

	if err := a.ui.Run(); err != nil {
		return fmt.Errorf("run ui: %w", err)
	}

	return nil
}

func StringOrRegex(input string) (*regexp.Regexp, error) {
	if strings.TrimSpace(input) == "" {
		return regexp.Compile(".*")
	}
	if strings.HasPrefix(input, "/") && strings.HasSuffix(input, "/") && len(input) >= 2 {
		return regexp.Compile(input[1 : len(input)-1])
	}
	return regexp.Compile("^" + regexp.QuoteMeta(input) + "$")
}

func FilterThreads(threads []Thread, state *RuntimeState, showIdle bool) []Thread {
	result := make([]Thread, 0, len(threads))
	for _, t := range threads {
		if !showIdle {
			cmd := strings.ToLower(t.Command)
			if cmd == "sleep" || cmd == "binlog dump" {
				continue
			}
		}
		if !state.FilterUser.MatchString(t.User) || !state.FilterDB.MatchString(t.DB) || !state.FilterHost.MatchString(t.Host) {
			continue
		}
		result = append(result, t)
	}
	return result
}

func SortThreads(threads []Thread, desc bool) {
	sort.SliceStable(threads, func(i, j int) bool {
		if desc {
			return threads[i].Time > threads[j].Time
		}
		return threads[i].Time < threads[j].Time
	})
}

func NormalizeHost(host string, resolve bool) string {
	if host == "" {
		return host
	}
	if strings.Contains(host, ":") {
		host = strings.Split(host, ":")[0]
	}
	if !resolve {
		parts := strings.Split(host, ".")
		if len(parts) > 2 {
			return strings.Join(parts[:2], ".")
		}
		return host
	}
	return host
}

func Sum(vals ...int64) int64 {
	var total int64
	for _, v := range vals {
		total += v
	}
	return total
}

func Commify(n int64) string {
	s := strconv.FormatInt(n, 10)
	neg := strings.HasPrefix(s, "-")
	if neg {
		s = s[1:]
	}
	for i := len(s) - 3; i > 0; i -= 3 {
		s = s[:i] + "," + s[i:]
	}
	if neg {
		return "-" + s
	}
	return s
}

func MakeShort(n int64, long bool) string {
	if long {
		return Commify(n)
	}
	type unit struct {
		name string
		size int64
	}
	units := []unit{{"T", 1_000_000_000_000}, {"G", 1_000_000_000}, {"M", 1_000_000}, {"k", 1_000}}
	for _, u := range units {
		if n >= u.size {
			return fmt.Sprintf("%.1f%s", float64(n)/float64(u.size), u.name)
		}
	}
	return strconv.FormatInt(n, 10)
}

func Clear(w io.Writer) {
	if runtime.GOOS == "windows" {
		_, _ = fmt.Fprint(w, strings.Repeat("\n", 40))
		return
	}
	_, _ = fmt.Fprint(w, "\033[H\033[2J")
}

func FindProg(name string) string {
	path, err := exec.LookPath(name)
	if err != nil {
		return ""
	}
	return path
}

func Pager() string {
	for _, p := range []string{"less", "more"} {
		if found := FindProg(p); found != "" {
			return found
		}
	}
	return ""
}

func PromptLine(r io.Reader, w io.Writer, label string) (string, error) {
	_, _ = fmt.Fprintf(w, "%s: ", label)
	line, err := bufio.NewReader(r).ReadString('\n')
	if err != nil && !errors.Is(err, io.EOF) {
		return "", err
	}
	return strings.TrimSpace(line), nil
}

func PrintHelp(cfg Config) string {
	lines := []string{
		"mytop-go shortcuts:",
		"t/m/c/I/S/q/?  - modes + quit + help",
		"H o i p R      - header/sort/idle/pause/resolve",
		"u d h F        - user/db/host filters + reset",
		"k K r          - kill thread / kill user / flush status",
		"f e V          - full query / explain / show variables",
		"s # D          - set delay / debug / dump config",
		"https://jeremy.zawodny.com/mysql/mytop/",
	}
	if !cfg.Color {
		return strings.Join(lines, "\n")
	}
	return "\033[1;36m" + strings.Join(lines, "\n") + "\033[0m"
}

func DumpConfig(cfg Config) string {
	pairs := map[string]string{
		"batchmode": strconv.FormatBool(cfg.BatchMode),
		"color":     strconv.FormatBool(cfg.Color),
		"db":        cfg.DB,
		"delay":     strconv.Itoa(cfg.Delay),
		"header":    strconv.FormatBool(cfg.Header),
		"host":      cfg.Host,
		"idle":      strconv.FormatBool(cfg.Idle),
		"mode":      string(cfg.Mode),
		"port":      strconv.Itoa(cfg.Port),
		"resolve":   strconv.FormatBool(cfg.Resolve),
		"socket":    cfg.Socket,
		"user":      cfg.User,
	}
	keys := slices.Sorted(maps.Keys(pairs))
	out := make([]string, 0, len(keys))
	for _, k := range keys {
		out = append(out, fmt.Sprintf("%s=%s", k, pairs[k]))
	}
	return strings.Join(out, "\n")
}

func CmdSetDelay(cfg *Config, delay int) {
	if delay < 1 {
		delay = 1
	}
	cfg.Delay = delay
}

func CmdQuit() error {
	os.Exit(0)
	return nil
}
