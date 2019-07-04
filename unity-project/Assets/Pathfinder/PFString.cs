using System;
using System.Text;
using System.Runtime.InteropServices;

public class PFString {
    private Byte[] encodedBytes;
    internal IntPtr handle;

    public PFString(string source) {
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
