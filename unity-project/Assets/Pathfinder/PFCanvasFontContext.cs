using System;

public class PFCanvasFontContext {
    internal IntPtr handle;

    public PFCanvasFontContext() {
        handle = PF.PFCanvasFontContextCreateWithSystemSource();
    }

    ~PFCanvasFontContext() {
        PF.PFCanvasFontContextRelease(handle);
    }
}
