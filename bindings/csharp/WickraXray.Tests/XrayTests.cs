using System.Text.Json;
using Wickra.Xray;
using Xunit;

namespace WickraXray.Tests;

public class XrayTests
{
    private const string Spec =
        "{\"dataset_ref\":\"m\",\"symbol\":\"AAA\",\"panels\":[{\"kind\":\"footprint\"," +
        "\"price_bin\":1.0,\"bucket_ms\":60000}]}";

    private static object Trade(long ts, double price, double qty, string side) =>
        new { ts, price, qty, side };

    [Fact]
    public void Version_IsNonEmpty()
    {
        Assert.False(string.IsNullOrEmpty(Xray.Version()));
    }

    [Fact]
    public void Frame_Roundtrip()
    {
        using var xray = new Xray(Spec);
        string load = JsonSerializer.Serialize(new
        {
            cmd = "load",
            dataset = new
            {
                trades = new[]
                {
                    Trade(1000, 100.4, 2.0, "buy"),
                    Trade(1400, 101.8, 0.5, "buy"),
                },
            },
        });
        xray.Command(load);

        string raw = xray.Command("{\"cmd\":\"frame\"}");
        using JsonDocument frame = JsonDocument.Parse(raw);

        Assert.Equal("AAA", frame.RootElement.GetProperty("symbol").GetString());
        Assert.Equal(1400, frame.RootElement.GetProperty("cursor_ts").GetInt64());
        JsonElement panels = frame.RootElement.GetProperty("panels");
        Assert.Equal(1, panels.GetArrayLength());
        Assert.Equal("footprint", panels[0].GetProperty("kind").GetString());
    }

    [Fact]
    public void InvalidSpec_Throws()
    {
        Assert.Throws<ArgumentException>(() => new Xray("not json"));
    }

    [Fact]
    public void UnknownCommand_IsInBandError()
    {
        using var xray = new Xray(Spec);
        // An unknown command is not a hard error: the ABI returns a length and the
        // error surfaces in-band as {"ok":false,...} JSON.
        string raw = xray.Command("{\"cmd\":\"nope\"}");
        Assert.Contains("\"ok\":false", raw);
    }
}
