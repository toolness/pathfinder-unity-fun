using UnityEngine;

public class PFPlugin {
    public static void ReleaseResources() {
        GL.IssuePluginEvent(PFPluginExports.get_render_canvas_func(), 0);
    }
}
