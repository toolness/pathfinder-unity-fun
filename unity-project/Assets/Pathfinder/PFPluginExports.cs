using System;
using System.Runtime.InteropServices;

public class PFPluginExports {
    [DllImport("GfxPluginPathfinder")]
    public static extern IntPtr get_render_event_func();

    [DllImport("GfxPluginPathfinder")]
    public static extern void queue_canvas_for_rendering(IntPtr handle);
}
