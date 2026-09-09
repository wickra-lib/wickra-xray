package org.wickra.xray;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import org.junit.jupiter.api.Test;

// The scrubber path, through the JVM binding.
//
// "Scrub the market like a video" is the claim the frame API is built around,
// and the golden corpus cannot check it: every blessed frame is a full-window
// frame, so the cursor never sits anywhere but the end. Three properties pin it
// down, each failing on a different mistake -- an off-by-one at the upper bound,
// a window that rounds to a bucket boundary instead of to the requested
// timestamp, and a panel that carries state past the cursor and would show the
// future.
//
// The byte-exact equality between frame_at(t) and a frame over a dataset
// truncated at t is checked from Python, Node and WASM, where clipping the
// dataset is a filter over parsed JSON. This binding has no JSON dependency and
// adding one to state the same property would be a dependency for a test.
class ScrubberTest {
    // Both are event timestamps in the golden dataset, which runs 1000..24000 in
    // 1000-unit steps, so neither lands in a gap where the cursor would have
    // nothing to sit on.
    private static final long MID = 12000;
    private static final long END = 24000;

    private static final Pattern VOLUMES =
            Pattern.compile("\"(?:buy_vol|sell_vol)\":\\[([^\\]]*)\\]");

    private static Path findGolden() {
        Path dir = Path.of("").toAbsolutePath();
        for (int i = 0; i < 8 && dir != null; i++) {
            Path g = dir.resolve("golden");
            if (Files.isDirectory(g.resolve("specs"))) {
                return g;
            }
            dir = dir.getParent();
        }
        return null;
    }

    /** Total traded volume in the frame, read off the named arrays. */
    private static double volumeTotal(String frame) {
        double total = 0.0;
        Matcher matcher = VOLUMES.matcher(frame);
        while (matcher.find()) {
            String body = matcher.group(1).strip();
            if (body.isEmpty()) {
                continue;
            }
            for (String value : body.split(",")) {
                total += Double.parseDouble(value.strip());
            }
        }
        return total;
    }

    @Test
    void theCursorClipsTheWindow() throws IOException {
        Path golden = findGolden();
        assumeTrue(golden != null, "golden fixtures not present yet");

        String spec = Files.readString(golden.resolve("specs").resolve("multi_panel.json"));
        String dataset = Files.readString(golden.resolve("data.json")).strip();

        try (Xray xray = new Xray(spec)) {
            xray.command("{\"cmd\":\"load\",\"dataset\":" + dataset + "}");

            String full = xray.command("{\"cmd\":\"frame\"}");
            String atEnd = xray.command("{\"cmd\":\"frame_at\",\"ts\":" + END + "}");
            String atMid = xray.command("{\"cmd\":\"frame_at\",\"ts\":" + MID + "}");

            // The spec leaves to_ts open, so frame's cursor is the dataset's own
            // end and asking frame_at for that timestamp must not differ.
            assertEquals(full, atEnd, "frame_at(" + END + ") is not the full frame");

            assertTrue(
                    atMid.contains("\"cursor_ts\":" + MID),
                    "frame_at(" + MID + ") does not report that cursor: " + atMid);

            double midVolume = volumeTotal(atMid);
            double endVolume = volumeTotal(atEnd);
            assertTrue(
                    midVolume < endVolume,
                    "folding to " + MID + " carried " + midVolume + " of the " + endVolume
                            + " total volume; a midpoint that folds everything means the cursor is"
                            + " not clipping the window");
        }
    }
}
