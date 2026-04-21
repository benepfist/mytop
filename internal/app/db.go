package app

import (
	"fmt"
	"strconv"
	"strings"
	"time"
)

type MemoryDB struct {
	status  map[string]int64
	threads []Thread
}

func NewMemoryDB() *MemoryDB {
	return &MemoryDB{
		status: map[string]int64{
			"Uptime":            12345,
			"Questions":         99999,
			"Slow_queries":      10,
			"Com_select":        50000,
			"Com_insert":        10000,
			"Com_update":        15000,
			"Com_delete":        5000,
			"Key_reads":         10,
			"Key_read_requests": 1000,
		},
		threads: []Thread{
			{ID: 1, User: "root", DB: "test", Host: "localhost:3306", Command: "Query", Time: 12, Info: "select * from users"},
			{ID: 2, User: "app", DB: "prod", Host: "app.local:45000", Command: "Sleep", Time: 3, Info: ""},
		},
	}
}

func (m *MemoryDB) Execute(query string) error {
	if strings.TrimSpace(query) == "" {
		return fmt.Errorf("empty query")
	}
	if strings.HasPrefix(strings.ToUpper(strings.TrimSpace(query)), "KILL") {
		parts := strings.Fields(query)
		if len(parts) == 2 {
			if id, err := strconv.ParseInt(parts[1], 10, 64); err == nil {
				filtered := m.threads[:0]
				for _, t := range m.threads {
					if t.ID != id {
						filtered = append(filtered, t)
					}
				}
				m.threads = filtered
			}
		}
	}
	return nil
}

func (m *MemoryDB) Hashes(query string) ([]map[string]string, error) {
	q := strings.ToUpper(strings.TrimSpace(query))
	switch {
	case strings.Contains(q, "SHOW GLOBAL STATUS") || strings.Contains(q, "SHOW STATUS"):
		rows := make([]map[string]string, 0, len(m.status))
		for k, v := range m.status {
			rows = append(rows, map[string]string{"Variable_name": k, "Value": strconv.FormatInt(v+time.Now().Unix()%11, 10)})
		}
		return rows, nil
	case strings.Contains(q, "SHOW FULL PROCESSLIST"):
		rows := make([]map[string]string, 0, len(m.threads))
		for _, t := range m.threads {
			rows = append(rows, map[string]string{
				"Id":      strconv.FormatInt(t.ID, 10),
				"User":    t.User,
				"db":      t.DB,
				"Host":    t.Host,
				"Command": t.Command,
				"Time":    strconv.FormatInt(t.Time, 10),
				"Info":    t.Info,
			})
		}
		return rows, nil
	case strings.Contains(q, "SHOW VARIABLES"):
		return []map[string]string{
			{"Variable_name": "version", "Value": "8.0"},
			{"Variable_name": "have_query_cache", "Value": "NO"},
		}, nil
	case strings.Contains(q, "SHOW INNODB STATUS"):
		return []map[string]string{{"Status": "InnoDB status output"}}, nil
	case strings.HasPrefix(q, "EXPLAIN"):
		return []map[string]string{{"id": "1", "select_type": "SIMPLE", "table": "users", "type": "ALL"}}, nil
	default:
		return []map[string]string{}, nil
	}
}
