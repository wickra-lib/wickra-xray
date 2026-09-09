/* The scrubber path, from C.
 *
 * "Scrub the market like a video" is the claim the frame API is built around,
 * and the golden corpus cannot check it: every fixture is a full-window frame.
 * Three properties pin the cursor down instead, and each one fails on a
 * different mistake:
 *
 *   frame_at(end) == frame          an off-by-one at the upper bound, or a
 *                                   cursor that is not the dataset's end when
 *                                   the spec leaves to_ts open
 *   frame_at(mid).cursor_ts == mid  a window that rounds to a bucket boundary
 *                                   instead of to the requested timestamp
 *   frame_at(mid) < frame_at(end)   a panel that carries state past the cursor,
 *                                   which would make the scrub show the future
 *
 * The byte-exact equality between `frame_at(t)` and a frame over a dataset
 * truncated at `t` is checked from Python and Node, where clipping the dataset
 * is a list comprehension rather than string surgery in a language with no JSON
 * library. Both halves are the same property; they are tested where each is
 * cheap to state correctly.
 *
 * The properties below are the ones docs/STREAMING.md states for streaming through time.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "wickra_xray.h"

#include "golden_specs.h" /* GOLDEN_DIR */

/* Both are event timestamps in the golden dataset (which runs 1000..24000 in
 * 1000-unit steps), so neither lands in a gap where the cursor would have
 * nothing to sit on. */
#define MID_TEXT "12000"
#define END_TEXT "24000"

static char *slurp(const char *path) {
    FILE *file = fopen(path, "rb");
    if (!file) {
        fprintf(stderr, "cannot open %s\n", path);
        return NULL;
    }
    if (fseek(file, 0, SEEK_END) != 0) {
        fclose(file);
        return NULL;
    }
    long size = ftell(file);
    if (size < 0 || fseek(file, 0, SEEK_SET) != 0) {
        fclose(file);
        return NULL;
    }
    char *buf = (char *)malloc((size_t)size + 1);
    if (!buf) {
        fclose(file);
        return NULL;
    }
    size_t got = fread(buf, 1, (size_t)size, file);
    fclose(file);
    buf[got] = '\0';
    return buf;
}

static char *run(WickraXray *xray, const char *cmd) {
    int len = wickra_xray_command(xray, cmd, NULL, 0);
    if (len < 0) {
        fprintf(stderr, "command failed with code %d\n", len);
        return NULL;
    }
    char *out = (char *)malloc((size_t)len + 1);
    if (!out) {
        return NULL;
    }
    if (wickra_xray_command(xray, cmd, out, (size_t)len + 1) < 0) {
        free(out);
        return NULL;
    }
    return out;
}

/* Sum every number in the frame's footprint volume arrays. A frame folded to a
 * midpoint has strictly less traded volume in it than the full one, and reading
 * that off the serialized form needs no JSON parser: both arrays are named, and
 * what is compared is one total against another produced the same way. */
static double volume_total(const char *frame) {
    static const char *KEYS[] = {"\"buy_vol\":[", "\"sell_vol\":["};
    double total = 0.0;
    for (size_t k = 0; k < sizeof(KEYS) / sizeof(KEYS[0]); k++) {
        const char *at = frame;
        while ((at = strstr(at, KEYS[k])) != NULL) {
            at += strlen(KEYS[k]);
            while (*at != ']' && *at != '\0') {
                char *end = NULL;
                double value = strtod(at, &end);
                if (end == at) {
                    at++; /* a separator */
                    continue;
                }
                total += value;
                at = end;
            }
        }
    }
    return total;
}

int main(void) {
    char *spec = slurp(GOLDEN_DIR "/specs/multi_panel.json");
    char *dataset = slurp(GOLDEN_DIR "/data.json");
    if (!spec || !dataset) {
        free(spec);
        free(dataset);
        return 1;
    }

    size_t load_len = strlen(dataset) + 64;
    char *load = (char *)malloc(load_len);
    WickraXray *xray = load ? wickra_xray_new(spec) : NULL;
    if (!xray) {
        fprintf(stderr, "spec rejected, or out of memory\n");
        free(load);
        free(spec);
        free(dataset);
        return 1;
    }
    snprintf(load, load_len, "{\"cmd\":\"load\",\"dataset\":%s}", dataset);

    int failures = 0;
    char *loaded = run(xray, load);
    char *full = loaded ? run(xray, "{\"cmd\":\"frame\"}") : NULL;
    char *at_end = full ? run(xray, "{\"cmd\":\"frame_at\",\"ts\":" END_TEXT "}") : NULL;
    char *at_mid = at_end ? run(xray, "{\"cmd\":\"frame_at\",\"ts\":" MID_TEXT "}") : NULL;

    if (!at_mid) {
        fprintf(stderr, "the dataset did not load, or a frame could not be read back\n");
        failures = 1;
    } else {
        if (strcmp(full, at_end) != 0) {
            fprintf(stderr,
                    "frame_at(" END_TEXT ") is not the full frame\n  frame:    %s\n  frame_at: %s\n",
                    full, at_end);
            failures++;
        }
        if (strstr(at_mid, "\"cursor_ts\":" MID_TEXT) == NULL) {
            fprintf(stderr, "frame_at(" MID_TEXT ") does not report that cursor:\n  %s\n", at_mid);
            failures++;
        }
        double mid_volume = volume_total(at_mid);
        double end_volume = volume_total(at_end);
        if (!(mid_volume < end_volume)) {
            fprintf(stderr,
                    "folding to " MID_TEXT " carried %.6f of the %.6f total volume; a midpoint "
                    "that folds everything means the cursor is not clipping the window\n",
                    mid_volume, end_volume);
            failures++;
        }
        if (failures == 0) {
            printf("scrubber holds from C: frame_at(" END_TEXT ") == frame, "
                   "frame_at(" MID_TEXT ") stops at its cursor (%.1f of %.1f volume)\n",
                   mid_volume, end_volume);
        }
    }

    free(at_mid);
    free(at_end);
    free(full);
    free(loaded);
    wickra_xray_free(xray);
    free(load);
    free(spec);
    free(dataset);
    return failures == 0 ? 0 : 1;
}
