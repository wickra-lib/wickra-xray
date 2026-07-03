/* A minimal C example: build a frame through the wickra-xray C ABI. */
#include <stdio.h>
#include <stdlib.h>

#include "wickra_xray.h"

static const char *SPEC =
    "{\"dataset_ref\":\"m\",\"symbol\":\"AAA\",\"panels\":["
    "{\"kind\":\"footprint\",\"price_bin\":1.0,\"bucket_ms\":60000}]}";

static const char *LOAD =
    "{\"cmd\":\"load\",\"dataset\":{\"trades\":["
    "{\"ts\":1000,\"price\":100.4,\"qty\":2.0,\"side\":\"buy\"},"
    "{\"ts\":1400,\"price\":101.8,\"qty\":0.5,\"side\":\"sell\"}]}}";

static const char *FRAME = "{\"cmd\":\"frame\"}";

/* Length-out protocol: learn the length, then read into a caller buffer.
   Returns a malloc'd NUL-terminated string the caller must free, or NULL. */
static char *run(WickraXray *xray, const char *cmd) {
    int len = wickra_xray_command(xray, cmd, NULL, 0);
    if (len < 0) {
        fprintf(stderr, "command failed: code %d\n", len);
        return NULL;
    }
    char *buf = (char *)malloc((size_t)len + 1);
    if (!buf) {
        return NULL;
    }
    wickra_xray_command(xray, cmd, buf, (size_t)len + 1);
    return buf;
}

int main(void) {
    WickraXray *xray = wickra_xray_new(SPEC);
    if (!xray) {
        fprintf(stderr, "failed to build xray\n");
        return 1;
    }

    char *loaded = run(xray, LOAD);
    if (!loaded) {
        wickra_xray_free(xray);
        return 1;
    }
    char *frame = run(xray, FRAME);
    if (!frame) {
        free(loaded);
        wickra_xray_free(xray);
        return 1;
    }

    printf("wickra-xray %s\n", wickra_xray_version());
    printf("loaded: %s\n", loaded);
    printf("frame: %s\n", frame);

    free(frame);
    free(loaded);
    wickra_xray_free(xray);
    return 0;
}
