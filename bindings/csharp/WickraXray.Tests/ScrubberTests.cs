using System.Text.Json;
using System.Text.Json.Nodes;
using Wickra.Xray;
using Xunit;

namespace WickraXray.Tests;

// The scrubber equality, through the .NET binding.
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
public class ScrubberTests
{
    // An event timestamp in the golden dataset (1000..24000 in 1000-unit steps),
    // not a gap between two: the frame over the clipped dataset takes its cursor
    // from the last event it holds, so a cut inside a gap would leave the two
    // frames on different cursors and compare nothing.
    private const long Cut = 12000;
    private const long End = 24000;

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

    private static Xray Loaded(string golden, JsonNode dataset)
    {
        string spec = File.ReadAllText(Path.Combine(golden, "specs", "multi_panel.json"));
        var xray = new Xray(spec);
        xray.Command(JsonSerializer.Serialize(new { cmd = "load", dataset }));
        return xray;
    }

    [Fact]
    public void FrameAt_EqualsATruncatedDataset()
    {
        string? golden = FindGolden();
        if (golden is null)
        {
            return; // golden fixtures not present yet
        }

        JsonNode full = JsonNode.Parse(File.ReadAllText(Path.Combine(golden, "data.json")))!;
        var clipped = new JsonObject();
        int kept = 0;
        int dropped = 0;
        foreach (var stream in full.AsObject())
        {
            var keep = new JsonArray();
            foreach (JsonNode? item in stream.Value!.AsArray())
            {
                if (item!["ts"]!.GetValue<long>() <= Cut)
                {
                    keep.Add(JsonNode.Parse(item.ToJsonString()));
                    kept++;
                }
                else
                {
                    dropped++;
                }
            }
            clipped[stream.Key] = keep;
        }

        // A cut that keeps everything or nothing would compare a frame with itself.
        Assert.True(kept > 0 && dropped > 0, $"the cut at {Cut} kept {kept} and dropped {dropped}");

        string atCut;
        using (var scrubbed = Loaded(golden, full))
        {
            atCut = scrubbed.Command($"{{\"cmd\":\"frame_at\",\"ts\":{Cut}}}");
        }

        string whole;
        using (var truncated = Loaded(golden, clipped))
        {
            whole = truncated.Command("{\"cmd\":\"frame\"}");
        }

        Assert.Equal(whole, atCut);
    }

    [Fact]
    public void FoldingToTheEnd_ReproducesTheFullFrame()
    {
        string? golden = FindGolden();
        if (golden is null)
        {
            return;
        }

        // The upper bound is where an off-by-one hides: the spec leaves to_ts
        // open, so frame's cursor is the dataset's own end.
        JsonNode full = JsonNode.Parse(File.ReadAllText(Path.Combine(golden, "data.json")))!;
        long end = full.AsObject()
            .SelectMany(stream => stream.Value!.AsArray())
            .Max(item => item!["ts"]!.GetValue<long>());
        Assert.Equal(End, end);

        using var xray = Loaded(golden, full);
        Assert.Equal(
            xray.Command("{\"cmd\":\"frame\"}"),
            xray.Command($"{{\"cmd\":\"frame_at\",\"ts\":{End}}}"));
    }

    [Fact]
    public void AMidpointFrame_ReportsItsOwnCursor()
    {
        string? golden = FindGolden();
        if (golden is null)
        {
            return;
        }

        JsonNode full = JsonNode.Parse(File.ReadAllText(Path.Combine(golden, "data.json")))!;
        using var xray = Loaded(golden, full);
        JsonNode frame = JsonNode.Parse(xray.Command($"{{\"cmd\":\"frame_at\",\"ts\":{Cut}}}"))!;
        Assert.Equal(Cut, frame["cursor_ts"]!.GetValue<long>());
    }
}
