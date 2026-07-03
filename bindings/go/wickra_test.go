package wickra

import (
	"encoding/json"
	"strings"
	"testing"
)

const spec = `{"dataset_ref":"m","symbol":"AAA","panels":[{"kind":"footprint","price_bin":1.0,"bucket_ms":60000}]}`

func trade(ts int, price, qty float64, side string) map[string]any {
	return map[string]any{"ts": ts, "price": price, "qty": qty, "side": side}
}

func TestVersion(t *testing.T) {
	if Version() == "" {
		t.Fatal("empty version")
	}
}

func TestFrameRoundtrip(t *testing.T) {
	x, err := New(spec)
	if err != nil {
		t.Fatal(err)
	}
	defer x.Close()

	dataset := map[string]any{"trades": []map[string]any{
		trade(1000, 100.4, 2.0, "buy"),
		trade(1400, 101.8, 0.5, "buy"),
	}}
	load, err := json.Marshal(map[string]any{"cmd": "load", "dataset": dataset})
	if err != nil {
		t.Fatal(err)
	}
	if _, err := x.Command(string(load)); err != nil {
		t.Fatal(err)
	}

	raw, err := x.Command(`{"cmd":"frame"}`)
	if err != nil {
		t.Fatal(err)
	}
	var frame struct {
		Symbol   string `json:"symbol"`
		CursorTS int64  `json:"cursor_ts"`
		Panels   []struct {
			Kind string `json:"kind"`
		} `json:"panels"`
	}
	if err := json.Unmarshal([]byte(raw), &frame); err != nil {
		t.Fatal(err)
	}
	if frame.Symbol != "AAA" {
		t.Fatalf("expected symbol AAA, got %q", frame.Symbol)
	}
	if frame.CursorTS != 1400 {
		t.Fatalf("expected cursor_ts 1400, got %d", frame.CursorTS)
	}
	if len(frame.Panels) != 1 || frame.Panels[0].Kind != "footprint" {
		t.Fatalf("expected one footprint panel, got %+v", frame.Panels)
	}
}

func TestInvalidSpec(t *testing.T) {
	if _, err := New("not json"); err == nil {
		t.Fatal("expected an error for an invalid spec")
	}
}

func TestUnknownCommandIsInBandError(t *testing.T) {
	x, err := New(spec)
	if err != nil {
		t.Fatal(err)
	}
	defer x.Close()

	// An unknown command is not a hard error: the C ABI returns a length and the
	// error surfaces in-band as {"ok":false,...} JSON.
	raw, err := x.Command(`{"cmd":"nope"}`)
	if err != nil {
		t.Fatalf("unexpected hard error: %v", err)
	}
	if !strings.Contains(raw, `"ok":false`) {
		t.Fatalf("expected an in-band error, got: %s", raw)
	}
}
