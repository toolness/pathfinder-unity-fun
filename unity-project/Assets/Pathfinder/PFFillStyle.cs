using System;
using UnityEngine;

public class PFFillStyle {
    internal IntPtr handle;

    private PFFillStyle(IntPtr handle) {
        this.handle = handle;
    }

    public static PFFillStyle CreateColor(Color32 c) {
        var color = PFUnityConv.PFColorU(c);
        return new PFFillStyle(PF.PFFillStyleCreateColor(ref color));
    }

    ~PFFillStyle() {
        PF.PFFillStyleDestroy(handle);
    }
}
