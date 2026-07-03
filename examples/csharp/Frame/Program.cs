// A runnable .NET example: build a frame through the binding.
//
//   cargo build --release -p wickra-xray-c
//   dotnet run --project examples/csharp/Frame

using System.Text.Json;
using Wickra.Xray;

const string spec =
    "{\"dataset_ref\":\"m\",\"symbol\":\"AAA\",\"panels\":[" +
    "{\"kind\":\"footprint\",\"price_bin\":1.0,\"bucket_ms\":60000}]}";

const string load =
    "{\"cmd\":\"load\",\"dataset\":{\"trades\":[" +
    "{\"ts\":1000,\"price\":100.4,\"qty\":2.0,\"side\":\"buy\"}," +
    "{\"ts\":1400,\"price\":101.8,\"qty\":0.5,\"side\":\"sell\"}]}}";

using var xray = new Xray(spec);
xray.Command(load);
string response = xray.Command("{\"cmd\":\"frame\"}");
using JsonDocument frame = JsonDocument.Parse(response);

Console.WriteLine($"wickra-xray {Xray.Version()}");
Console.WriteLine(response);
Console.WriteLine($"  panels: {frame.RootElement.GetProperty("panels").GetArrayLength()}");
