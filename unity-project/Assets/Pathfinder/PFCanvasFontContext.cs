using System;

public class PFCanvasFontContext {
    internal IntPtr handle;

    public PFCanvasFontContext() {
        handle = PF.PFCanvasFontContextCreateWithSystemSource();
    }

    internal IntPtr PrepareToConsume() {
        var oldHandle = handle;
        handle = PF.PFCanvasFontContextClone(handle);
        return oldHandle;
    }

    ~PFCanvasFontContext() {
        PF.PFCanvasFontContextDestroy(handle);
    }
}
