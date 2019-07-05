using System;
using System.Runtime.InteropServices;

internal class PFPluginExports {
    [DllImport("GfxPluginPathfinder")]
    internal static extern IntPtr get_render_canvas_func();

    [DllImport("GfxPluginPathfinder")]
    internal static extern void queue_canvas_for_rendering(IntPtr handle, Int32 id);
}
