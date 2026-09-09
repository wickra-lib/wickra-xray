package wickra

// The scrubber equality, through the Go binding.
//
// frame_at(t) over the whole dataset must return exactly what frame returns over
// a dataset that ends at t. That equality is what "scrub the market like a
// video" means, and no golden fixture can check it: every blessed frame is a
// full-window frame, so the cursor never sits anywhere but the end.
//
// The two routes are genuinely different inside the core -- one clips a loaded
// window, the other never sees the later events -- so a window bound that is
// inclusive on the wrong side, a bucket that rounds outward, or a panel that
// keeps state past the cursor separates them.
//
// The properties below are the ones docs/STREAMING.md states for streaming through time.

import (
	"encoding/json"
	"os"
	"path/filepath"
	"testing"
)

// An event timestamp in the golden dataset (1000..24000 in 1000-unit steps), not
// a gap between two: the frame over the clipped dataset takes its cursor from
// the last event it holds, so a cut inside a gap would leave the two frames on
// different cursors and compare nothing.
const scrubCut = 12000

type event struct {
	TS int64 `json:"ts"`
}

// loadedXray returns an Xray with the multi_panel spec and dataset loaded. The
// caller closes it.
func loadedXray(t *testing.T, g string, dataset any) *Xray {
	t.Helper()
	specJSON, err := os.ReadFile(filepath.Join(g, "specs", "multi_panel.json"))
	if err != nil {
		t.Fatal(err)
	}
	x, err := New(string(specJSON))
	if err != nil {
		t.Fatal(err)
	}
	load, err := json.Marshal(map[string]any{"cmd": "load", "dataset": dataset})
	if err != nil {
		x.Close()
		t.Fatal(err)
	}
	if _, err := x.Command(string(load)); err != nil {
		x.Close()
		t.Fatal(err)
	}
	return x
}

func rawDataset(t *testing.T, g string) map[string][]json.RawMessage {
	t.Helper()
	blob, err := os.ReadFile(filepath.Join(g, "data.json"))
	if err != nil {
		t.Fatal(err)
	}
	var streams map[string][]json.RawMessage
	if err := json.Unmarshal(blob, &streams); err != nil {
		t.Fatal(err)
	}
	return streams
}

func TestFrameAtEqualsATruncatedDataset(t *testing.T) {
	g := goldenDir()
	if g == "" {
		t.Skip("golden fixtures not present yet")
	}
	full := rawDataset(t, g)

	clipped := make(map[string][]json.RawMessage, len(full))
	kept, dropped := 0, 0
	for stream, events := range full {
		keep := make([]json.RawMessage, 0, len(events))
		for _, raw := range events {
			var e event
			if err := json.Unmarshal(raw, &e); err != nil {
				t.Fatal(err)
			}
			if e.TS <= scrubCut {
				keep = append(keep, raw)
				kept++
			} else {
				dropped++
			}
		}
		clipped[stream] = keep
	}
	// A cut that keeps everything or nothing would compare a frame with itself.
	if kept == 0 || dropped == 0 {
		t.Fatalf("the cut at %d kept %d and dropped %d events", scrubCut, kept, dropped)
	}

	scrubbed := loadedXray(t, g, full)
	atCut, err := scrubbed.Command(`{"cmd":"frame_at","ts":12000}`)
	scrubbed.Close()
	if err != nil {
		t.Fatal(err)
	}

	truncated := loadedXray(t, g, clipped)
	whole, err := truncated.Command(`{"cmd":"frame"}`)
	truncated.Close()
	if err != nil {
		t.Fatal(err)
	}

	if atCut != whole {
		t.Fatalf("scrubbing to %d is not the same as ending there\n  scrubbed:  %s\n  truncated: %s",
			scrubCut, atCut, whole)
	}
}

func TestFoldingToTheEndReproducesTheFullFrame(t *testing.T) {
	g := goldenDir()
	if g == "" {
		t.Skip("golden fixtures not present yet")
	}
	full := rawDataset(t, g)

	// The upper bound is where an off-by-one hides: the spec leaves to_ts open,
	// so frame's cursor is the dataset's own end.
	var end int64
	for _, events := range full {
		for _, raw := range events {
			var e event
			if err := json.Unmarshal(raw, &e); err != nil {
				t.Fatal(err)
			}
			if e.TS > end {
				end = e.TS
			}
		}
	}

	x := loadedXray(t, g, full)
	defer x.Close()
	atEnd, err := x.Command(`{"cmd":"frame_at","ts":24000}`)
	if err != nil {
		t.Fatal(err)
	}
	whole, err := x.Command(`{"cmd":"frame"}`)
	if err != nil {
		t.Fatal(err)
	}
	if end != 24000 {
		t.Fatalf("the golden dataset ends at %d, so this test asks for the wrong timestamp", end)
	}
	if atEnd != whole {
		t.Fatalf("frame_at(%d) is not the full frame", end)
	}
}

func TestAMidpointFrameReportsItsOwnCursor(t *testing.T) {
	g := goldenDir()
	if g == "" {
		t.Skip("golden fixtures not present yet")
	}
	x := loadedXray(t, g, rawDataset(t, g))
	defer x.Close()
	raw, err := x.Command(`{"cmd":"frame_at","ts":12000}`)
	if err != nil {
		t.Fatal(err)
	}
	var frame struct {
		CursorTS int64 `json:"cursor_ts"`
	}
	if err := json.Unmarshal([]byte(raw), &frame); err != nil {
		t.Fatal(err)
	}
	if frame.CursorTS != scrubCut {
		t.Fatalf("frame_at(%d) reports cursor %d", scrubCut, frame.CursorTS)
	}
}
