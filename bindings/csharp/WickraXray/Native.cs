using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

namespace Wickra.Xray;

/// <summary>Raw P/Invoke surface for the wickra-xray C ABI.</summary>
internal static partial class Native
{
    internal const string Lib = "wickra_xray";

    /// <summary>Build an xray from a spec JSON (NUL-terminated UTF-8). Null on error.</summary>
    [LibraryImport(Lib)]
    [UnmanagedCallConv(CallConvs = [typeof(CallConvCdecl)])]
    internal static partial IntPtr wickra_xray_new(byte[] specUtf8);

    /// <summary>Free an xray handle.</summary>
    [LibraryImport(Lib)]
    [UnmanagedCallConv(CallConvs = [typeof(CallConvCdecl)])]
    internal static partial void wickra_xray_free(IntPtr handle);

    /// <summary>
    /// Apply a command JSON (NUL-terminated UTF-8), writing the response into a
    /// caller-owned buffer. Returns the response length, or a negative error code.
    /// </summary>
    [LibraryImport(Lib)]
    [UnmanagedCallConv(CallConvs = [typeof(CallConvCdecl)])]
    internal static partial int wickra_xray_command(IntPtr handle, byte[] cmdUtf8, byte[]? outBuf, nuint cap);

    /// <summary>The library version as a static NUL-terminated string.</summary>
    [LibraryImport(Lib)]
    [UnmanagedCallConv(CallConvs = [typeof(CallConvCdecl)])]
    internal static partial IntPtr wickra_xray_version();
}
