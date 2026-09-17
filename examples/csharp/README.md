# Wickra X-Ray examples — C#

Runnable C# examples for the [Wickra X-Ray C# binding](../../bindings/csharp). The binding consumes the C ABI
library through P/Invoke, so build it once before running anything:

```bash
cargo build -p wickra-xray-c --release
```

## Run

As the CI examples job runs it, from the repository root:

```bash
dotnet run --project examples/csharp/Frame
```

## The examples

| Example | What it does |
|---------|--------------|
| `Frame/Program.cs` | A runnable .NET example: build a frame through the binding. |
