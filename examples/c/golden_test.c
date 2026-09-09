/* Cross-language golden parity, from C.
 *
 * Build the X-Ray from each committed golden/specs/*.json, load the shared
 * golden dataset and read back the frame, and assert the response equals
 * golden/expected/<spec>.json byte-for-byte. The ABI returns the core's compact
 * command output verbatim, so byte equality is the exact cross-language parity
 * check -- the same one Python, Node, Go, C#, Java, R and WASM make.
 *
 * Until this existed the C ABI was the only reach with no test at all: the two
 * examples beside it print a frame and exit zero, which passes whatever the
 * numbers say. Six of the ten language reaches go through this ABI, so a fault
 * here is a fault in all of them.
 *
 * C has no directory API that is portable between POSIX and Windows, so the
 * spec list is globbed by CMake at configure time and written into
 * golden_specs.h. That keeps the property the other bindings get from a runtime
 * glob: a spec added to the corpus is covered here without editing this file.
 * A hand-maintained list would silently skip it, which is the failure this
 * whole corpus exists to prevent.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "wickra_xray.h"

#include "golden_specs.h" /* GOLDEN_DIR, GOLDEN_SPECS, GOLDEN_SPEC_COUNT */

/* Read a whole file. Caller frees. Returns NULL and reports on failure. */
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

/* Trim ASCII whitespace in place and return the start of the trimmed text. */
static char *trim(char *text) {
    while (*text == ' ' || *text == '\n' || *text == '\r' || *text == '\t') {
        text++;
    }
    size_t len = strlen(text);
    while (len > 0) {
        char last = text[len - 1];
        if (last != ' ' && last != '\n' && last != '\r' && last != '\t') {
            break;
        }
        text[--len] = '\0';
    }
    return text;
}

static char *join(const char *a, const char *b, const char *c) {
    size_t len = strlen(a) + strlen(b) + strlen(c) + 1;
    char *out = (char *)malloc(len);
    if (out) {
        snprintf(out, len, "%s%s%s", a, b, c);
    }
    return out;
}

/* Apply one command through the two-call length protocol. Caller frees. */
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

int main(void) {
    if (GOLDEN_SPEC_COUNT == 0) {
        fprintf(stderr, "no golden specs were configured; this would test nothing\n");
        return 1;
    }

    char *dataset_raw = slurp(GOLDEN_DIR "/data.json");
    if (!dataset_raw) {
        return 1;
    }
    char *dataset = trim(dataset_raw);
    char *load = join("{\"cmd\":\"load\",\"dataset\":", dataset, "}");
    if (!load) {
        free(dataset_raw);
        return 1;
    }

    int failures = 0;
    for (size_t i = 0; i < GOLDEN_SPEC_COUNT; i++) {
        const char *name = GOLDEN_SPECS[i];

        char *spec_path = join(GOLDEN_DIR "/specs/", name, "");
        char *expected_path = join(GOLDEN_DIR "/expected/", name, "");
        char *spec = spec_path ? slurp(spec_path) : NULL;
        char *expected_raw = expected_path ? slurp(expected_path) : NULL;
        free(spec_path);
        free(expected_path);
        if (!spec || !expected_raw) {
            fprintf(stderr, "%s: missing spec or expected file\n", name);
            free(spec);
            free(expected_raw);
            failures++;
            continue;
        }
        char *expected = trim(expected_raw);

        WickraXray *xray = wickra_xray_new(spec);
        if (!xray) {
            fprintf(stderr, "%s: spec rejected\n", name);
            free(spec);
            free(expected_raw);
            failures++;
            continue;
        }

        char *loaded = run(xray, load);
        char *got_raw = loaded ? run(xray, "{\"cmd\":\"frame\"}") : NULL;
        if (got_raw) {
            char *got = trim(got_raw);
            if (strcmp(got, expected) != 0) {
                fprintf(stderr, "%s: mismatch\n  expected: %s\n  got:      %s\n",
                        name, expected, got);
                failures++;
            }
        } else {
            fprintf(stderr, "%s: could not read back a frame\n", name);
            failures++;
        }

        free(got_raw);
        free(loaded);
        wickra_xray_free(xray);
        free(spec);
        free(expected_raw);
    }

    free(load);
    free(dataset_raw);

    if (failures > 0) {
        fprintf(stderr, "%d of %zu golden specs did not match\n", failures, GOLDEN_SPEC_COUNT);
        return 1;
    }
    printf("all %zu golden specs are byte-identical from C\n", GOLDEN_SPEC_COUNT);
    return 0;
}
