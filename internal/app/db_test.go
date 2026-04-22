package app

import "testing"

func TestMemoryDBExecuteAndHashes(t *testing.T) {
	db := NewMemoryDB()
	if err := db.Execute(" "); err == nil {
		t.Fatalf("expected error on empty query")
	}
	if err := db.Execute("KILL 2"); err != nil {
		t.Fatalf("kill failed: %v", err)
	}
	rows, err := db.Hashes("SHOW FULL PROCESSLIST")
	if err != nil {
		t.Fatal(err)
	}
	if len(rows) != 1 {
		t.Fatalf("expected one thread after kill, got %d", len(rows))
	}

	cases := []string{
		"SHOW GLOBAL STATUS",
		"SHOW STATUS LIKE \"Questions\"",
		"SHOW VARIABLES",
		"SHOW INNODB STATUS",
		"EXPLAIN select 1",
		"unknown",
	}
	for _, q := range cases {
		if _, err := db.Hashes(q); err != nil {
			t.Fatalf("hashes(%q): %v", q, err)
		}
	}
}
