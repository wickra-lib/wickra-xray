package org.wickra.xray;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.Test;

class XrayTest {
    private static final String SPEC =
            "{\"dataset_ref\":\"m\",\"symbol\":\"AAA\",\"panels\":[{\"kind\":\"footprint\","
                    + "\"price_bin\":1.0,\"bucket_ms\":60000}]}";

    private static String trade(int ts, String price, String qty) {
        return "{\"ts\":" + ts + ",\"price\":" + price + ",\"qty\":" + qty + ",\"side\":\"buy\"}";
    }

    @Test
    void versionIsNonEmpty() {
        assertFalse(Xray.version().isEmpty());
    }

    @Test
    void frameRoundtrip() {
        try (Xray xray = new Xray(SPEC)) {
            String load = "{\"cmd\":\"load\",\"dataset\":{\"trades\":["
                    + trade(1000, "100.4", "2.0") + ","
                    + trade(1400, "101.8", "0.5") + "]}}";
            xray.command(load);
            String raw = xray.command("{\"cmd\":\"frame\"}");
            assertTrue(raw.contains("\"symbol\":\"AAA\""), raw);
            assertTrue(raw.contains("\"cursor_ts\":1400"), raw);
            assertTrue(raw.contains("\"kind\":\"footprint\""), raw);
        }
    }

    @Test
    void invalidSpecThrows() {
        assertThrows(IllegalArgumentException.class, () -> new Xray("not json"));
    }

    @Test
    void unknownCommandIsInBandError() {
        try (Xray xray = new Xray(SPEC)) {
            // An unknown command is not a hard error: the ABI returns a length and
            // the error surfaces in-band as {"ok":false,...} JSON.
            String raw = xray.command("{\"cmd\":\"nope\"}");
            assertEquals(true, raw.contains("\"ok\":false"), raw);
        }
    }
}
