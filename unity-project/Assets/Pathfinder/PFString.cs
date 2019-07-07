using System;
using System.Text;
using System.Runtime.InteropServices;

/// <summary>
/// This class encapsulates a string that's been prepared for sending to
/// Pathfinder, which expects UTF-8 encoded strings.
///
/// It can be transparently converted from a `string`, so anything
/// that takes a `PFString` can also take a `string`. But it might be useful
/// to cache `PFString` instances for efficiency.
/// </summary>
public class PFString {
    private const string ZERO_WIDTH_SPACE = "\u200b";
    private Byte[] encodedBytes;
    internal IntPtr handle;

    public PFString(string source) {
        if (source.Length == 0) {
            /// Right now the Pathfinder C API really doesn't like it if we pass
            /// it empty strings, possibly because it assumes that anything with
            /// a length of 0 is a null-terminated string (which isn't the case for us),
            /// so we need to avoid passing it empty strings. For now we'll just
            /// replace empty strings with zero-width spaces.
            source = ZERO_WIDTH_SPACE;
        }

        var utf8 = new UTF8Encoding(false);
        encodedBytes = utf8.GetBytes(source);

        // This is really inefficient because we're copying the byte buffer.
        // Ideally we could use C#'s unsafe keyword, which would obviate the copy,
        // but I'm not sure how portable that is right now, so I'm playing it safe.
        handle = Marshal.AllocHGlobal(encodedBytes.Length);
        Marshal.Copy(encodedBytes, 0, handle, encodedBytes.Length);
    }

    public UIntPtr len {
        get { return (UIntPtr) encodedBytes.Length; }
    }

    public static implicit operator PFString(string s) {
        return new PFString(s);
    }

    ~PFString() {
        Marshal.FreeHGlobal(handle);
    }
}
