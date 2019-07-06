using UnityEngine;

public class PFPlugin {
    public static void ReleaseResources() {
        GL.IssuePluginEvent(PFPluginExports.get_shutdown_func(), 0);
    }
}
