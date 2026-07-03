using System.Text.Json;
using Wickra.Xray;
using Xunit;

namespace WickraXray.Tests;

// Cross-language golden parity: build the xray from each committed
// golden/specs/*.json, load the shared golden/data.json and read back the
// frame, then assert it equals golden/expected/<spec>.json byte-for-byte. The
// binding returns the core's compact command_json string verbatim, so byte
// equality is the exact cross-language parity check. The fixtures arrive in a
// later phase; until then the test skips cleanly.
public class GoldenTests
{
    private static string? FindGolden()
    {
        string? dir = AppContext.BaseDirectory;
        for (int i = 0; i < 10 && dir is not null; i++)
        {
            string g = Path.Combine(dir, "golden");
            if (Directory.Exists(Path.Combine(g, "specs")))
            {
                return g;
            }
            dir = Path.GetDirectoryName(dir);
        }
        return null;
    }

    [Fact]
    public void GoldenFrames_AreByteIdentical()
    {
        string? golden = FindGolden();
        if (golden is null)
        {
            return; // golden fixtures not present yet
        }

        string dataset = File.ReadAllText(Path.Combine(golden, "data.json"));
        using JsonDocument data = JsonDocument.Parse(dataset);

        foreach (string specPath in Directory.GetFiles(Path.Combine(golden!, "specs"), "*.json"))
        {
            string spec = File.ReadAllText(specPath);
            string name = Path.GetFileName(specPath);
            string expected = File.ReadAllText(Path.Combine(golden!, "expected", name)).TrimEnd();

            using var xray = new Xray(spec);
            string load = JsonSerializer.Serialize(new { cmd = "load", dataset = data.RootElement });
            xray.Command(load);
            string raw = xray.Command("{\"cmd\":\"frame\"}");
            Assert.Equal(expected, raw.TrimEnd());
        }
    }
}
