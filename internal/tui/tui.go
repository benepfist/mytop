package tui

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
	"time"

	"mytop/internal/app"
)

type TUI struct {
	cfg   app.Config
	state *app.RuntimeState
	db    app.DB
	in    *bufio.Reader
}

func New() *TUI {
	return &TUI{in: bufio.NewReader(os.Stdin)}
}

func (t *TUI) Init(cfg app.Config, state *app.RuntimeState, db app.DB) error {
	t.cfg = cfg
	t.state = state
	t.db = db
	return nil
}

func (t *TUI) Run() error {
	if t.cfg.Help {
		fmt.Println(app.PrintHelp(t.cfg))
		return nil
	}
	cycles := 1
	if !t.cfg.BatchMode {
		cycles = 5
	}
	for i := 0; i < cycles; i++ {
		if !t.state.Paused {
			if err := t.runMode(); err != nil {
				return err
			}
		}
		if t.cfg.BatchMode {
			break
		}
		if err := t.readCommand(); err != nil {
			return err
		}
		time.Sleep(time.Duration(t.cfg.Delay) * time.Second)
	}
	return nil
}

func (t *TUI) runMode() error {
	switch t.cfg.Mode {
	case app.ModeTop:
		return t.getData()
	case app.ModeQPS:
		return t.getQPS()
	case app.ModeCmd:
		return t.getCmdSummary()
	case app.ModeInnoDB:
		return t.getInnoDBStatus()
	case app.ModeStatus:
		return t.getShowStatus()
	default:
		t.cfg.Mode = app.ModeTop
		return t.getData()
	}
}

func (t *TUI) getData() error {
	statusRows, _ := t.db.Hashes("SHOW GLOBAL STATUS")
	processRows, _ := t.db.Hashes("SHOW FULL PROCESSLIST")
	status := toMap(statusRows)
	now := time.Now()
	uptime := atoi64(status["Uptime"])
	questions := atoi64(status["Questions"])
	slow := atoi64(status["Slow_queries"])
	delta := questions - t.state.StatusPrev["Questions"]
	elapsed := now.Sub(t.state.PrevAt).Seconds()
	if t.state.PrevAt.IsZero() || elapsed <= 0 {
		elapsed = float64(t.cfg.Delay)
	}
	qps := float64(delta) / elapsed
	if qps < 0 {
		qps = 0
	}

	if t.cfg.Header {
		fmt.Printf("Uptime %s | Questions %s | QPS %.2f | Slow/s %.2f\n",
			app.MakeShort(uptime, t.cfg.LongNums),
			app.MakeShort(questions, t.cfg.LongNums),
			qps,
			float64(slow)/elapsed,
		)
	}

	threads := parseThreads(processRows, t.state)
	threads = app.FilterThreads(threads, t.state, t.cfg.Idle)
	app.SortThreads(threads, t.cfg.SortDesc)
	for _, th := range threads {
		fmt.Printf("%5d %-8s %-8s %-16s %-8s %4d %s\n", th.ID, th.User, th.DB, app.NormalizeHost(th.Host, t.cfg.Resolve), th.Command, th.Time, oneLine(th.Info))
	}

	t.state.StatusPrev["Questions"] = questions
	t.state.PrevAt = now
	return nil
}

func (t *TUI) getQPS() error {
	rows, err := t.db.Hashes("SHOW STATUS LIKE \"Questions\"")
	if err != nil {
		return err
	}
	m := toMap(rows)
	q := atoi64(m["Questions"])
	delta := q - t.state.StatusPrev["Questions"]
	fmt.Printf("Questions=%d Delta=%d\n", q, delta)
	t.state.StatusPrev["Questions"] = q
	return nil
}

func (t *TUI) getCmdSummary() error {
	rows, err := t.db.Hashes("SHOW GLOBAL STATUS LIKE 'Com_%'")
	if err != nil {
		return err
	}
	total := int64(0)
	type row struct {
		n string
		v int64
		d int64
	}
	buf := []row{}
	for _, r := range rows {
		n := r["Variable_name"]
		if !strings.HasPrefix(n, "Com_") {
			continue
		}
		v := atoi64(r["Value"])
		old := t.state.StatusPrev[n]
		buf = append(buf, row{n: strings.ReplaceAll(strings.TrimPrefix(n, "Com_"), "_", " "), v: v, d: v - old})
		total += v
		t.state.StatusPrev[n] = v
	}
	sort.Slice(buf, func(i, j int) bool { return buf[i].v > buf[j].v })
	for _, r := range buf {
		p := pct(r.v, total)
		dp := pct(r.d, total)
		fmt.Printf("%-24s total=%8d (%5.1f%%) delta=%6d (%5.1f%%)\n", r.n, r.v, p, r.d, dp)
	}
	return nil
}

func (t *TUI) getShowStatus() error {
	rows, err := t.db.Hashes("SHOW GLOBAL STATUS")
	if err != nil {
		return err
	}
	for _, r := range rows {
		n := r["Variable_name"]
		if strings.HasPrefix(n, "Com_") {
			continue
		}
		v, err := strconv.ParseInt(r["Value"], 10, 64)
		if err != nil {
			continue
		}
		d := v - t.state.StatusPrev[n]
		t.state.StatusPrev[n] = v
		if d != 0 && t.cfg.Color {
			fmt.Printf("\033[33m%-32s total=%10d delta=%8d\033[0m\n", n, v, d)
		} else {
			fmt.Printf("%-32s total=%10d delta=%8d\n", n, v, d)
		}
	}
	return nil
}

func (t *TUI) getShowVariables() error {
	rows, err := t.db.Hashes("SHOW VARIABLES")
	if err != nil {
		return err
	}
	var b strings.Builder
	for _, r := range rows {
		b.WriteString(fmt.Sprintf("%s: %s\n", r["Variable_name"], r["Value"]))
	}
	fmt.Print(b.String())
	return nil
}

func (t *TUI) getInnoDBStatus() error {
	rows, err := t.db.Hashes("SHOW INNODB STATUS")
	if err != nil {
		return err
	}
	for _, r := range rows {
		if s := r["Status"]; s != "" {
			fmt.Println(s)
		}
	}
	return nil
}

func (t *TUI) fullQueryInfo(id int64) {
	if q, ok := t.state.QCache[id]; ok {
		fmt.Println(q)
		return
	}
	fmt.Println("*** Invalid id. ***")
}

func (t *TUI) explain(id int64) error {
	q, ok := t.state.QCache[id]
	if !ok {
		fmt.Println("*** Invalid id. ***")
		return nil
	}
	db := t.state.DBCache[id]
	if db != "" {
		_ = t.db.Execute("USE " + db)
	}
	rows, err := t.db.Hashes("EXPLAIN " + q)
	if err != nil {
		return err
	}
	printTable(rows)
	return nil
}

func printTable(rows []map[string]string) {
	for _, r := range rows {
		for k, v := range r {
			fmt.Printf("%s: %s\n", k, v)
		}
		fmt.Println("---")
	}
}

func (t *TUI) readCommand() error {
	fmt.Print("cmd> ")
	line, _ := t.in.ReadString('\n')
	cmd := strings.TrimSpace(line)
	if cmd == "" {
		return nil
	}
	switch cmd {
	case "t":
		t.cfg.Mode = app.ModeTop
	case "m":
		t.cfg.Mode = app.ModeQPS
	case "c":
		t.cfg.Mode = app.ModeCmd
	case "I":
		t.cfg.Mode = app.ModeInnoDB
	case "S":
		t.cfg.Mode = app.ModeStatus
	case "?":
		fmt.Println(app.PrintHelp(t.cfg))
	case "q":
		return app.CmdQuit()
	case "H":
		t.cfg.Header = !t.cfg.Header
	case "o":
		t.cfg.SortDesc = !t.cfg.SortDesc
	case "i":
		t.cfg.Idle = !t.cfg.Idle
	case "p":
		t.state.Paused = !t.state.Paused
	case "R":
		t.cfg.Resolve = !t.cfg.Resolve
	case "F":
		t.state.FilterUser, _ = app.StringOrRegex("")
		t.state.FilterDB, _ = app.StringOrRegex("")
		t.state.FilterHost, _ = app.StringOrRegex("")
	case "u", "d", "h", "k", "K", "f", "e", "s":
		return t.handleInputCommand(cmd)
	case "r":
		return t.db.Execute("FLUSH STATUS")
	case "V":
		return t.getShowVariables()
	case "#":
		fmt.Printf("debug: mode=%s threads=%d\n", t.cfg.Mode, len(t.state.QCache))
	case "D":
		fmt.Println(app.DumpConfig(t.cfg))
	}
	return nil
}

func (t *TUI) handleInputCommand(cmd string) error {
	fmt.Print("value> ")
	line, _ := t.in.ReadString('\n')
	line = strings.TrimSpace(line)
	switch cmd {
	case "u":
		re, err := app.StringOrRegex(line)
		if err != nil {
			fmt.Println("invalid regex")
			return nil
		}
		t.state.FilterUser = re
	case "d":
		re, err := app.StringOrRegex(line)
		if err != nil {
			fmt.Println("invalid regex")
			return nil
		}
		t.state.FilterDB = re
	case "h":
		re, err := app.StringOrRegex(line)
		if err != nil {
			fmt.Println("invalid regex")
			return nil
		}
		t.state.FilterHost = re
	case "k":
		id, err := strconv.ParseInt(line, 10, 64)
		if err != nil {
			fmt.Println("*** Invalid id. ***")
			return nil
		}
		return t.db.Execute(fmt.Sprintf("KILL %d", id))
	case "K":
		for id, u := range t.state.UCache {
			if u == line {
				_ = t.db.Execute(fmt.Sprintf("KILL %d", id))
			}
		}
	case "f":
		id, err := strconv.ParseInt(line, 10, 64)
		if err != nil {
			fmt.Println("*** Invalid id. ***")
			return nil
		}
		t.fullQueryInfo(id)
	case "e":
		id, err := strconv.ParseInt(line, 10, 64)
		if err != nil {
			fmt.Println("*** Invalid id. ***")
			return nil
		}
		return t.explain(id)
	case "s":
		delay, err := strconv.Atoi(line)
		if err != nil {
			fmt.Println("invalid delay")
			return nil
		}
		app.CmdSetDelay(&t.cfg, delay)
	}
	return nil
}

func parseThreads(rows []map[string]string, state *app.RuntimeState) []app.Thread {
	threads := make([]app.Thread, 0, len(rows))
	for _, r := range rows {
		id := atoi64(r["Id"])
		th := app.Thread{
			ID:      id,
			User:    r["User"],
			DB:      r["db"],
			Host:    r["Host"],
			Command: r["Command"],
			Time:    atoi64(r["Time"]),
			Info:    r["Info"],
		}
		threads = append(threads, th)
		state.QCache[id] = th.Info
		state.UCache[id] = th.User
		state.DBCache[id] = th.DB
	}
	return threads
}

func toMap(rows []map[string]string) map[string]string {
	m := map[string]string{}
	for _, r := range rows {
		k := r["Variable_name"]
		if k != "" {
			m[k] = r["Value"]
		}
	}
	return m
}

func atoi64(v string) int64 {
	n, _ := strconv.ParseInt(v, 10, 64)
	return n
}

func pct(v, total int64) float64 {
	if total == 0 {
		return 0
	}
	return float64(v) / float64(total) * 100
}

func oneLine(v string) string {
	return strings.ReplaceAll(strings.TrimSpace(v), "\n", " ")
}
