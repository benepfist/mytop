package app

import "testing"

func TestStringOrRegex(t *testing.T) {
	re, err := StringOrRegex("/foo.*/")
	if err != nil || !re.MatchString("foobar") {
		t.Fatalf("expected regex to match")
	}
	re, _ = StringOrRegex("admin")
	if !re.MatchString("admin") || re.MatchString("administrator") {
		t.Fatalf("expected exact match")
	}
}

func TestMakeShort(t *testing.T) {
	if got := MakeShort(1500, false); got != "1.5k" {
		t.Fatalf("got %s", got)
	}
	if got := MakeShort(1200, true); got != "1,200" {
		t.Fatalf("got %s", got)
	}
}
