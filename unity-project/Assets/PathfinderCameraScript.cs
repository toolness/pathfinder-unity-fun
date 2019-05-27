using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System;
using System.Runtime.InteropServices;

[Serializable]
[StructLayout(LayoutKind.Sequential)]
struct PFPoint2DF {
    public float x;
    public float y;

    public PFPoint2DF(float x, float y) {
        this.x = x;
        this.y = y;
    }
}

class PFPath {
    private IntPtr handle;

    [DllImport("GfxPluginPathfinder")]
    private static extern IntPtr PFPathCreate();

    [DllImport("GfxPluginPathfinder")]
    private static extern void PFPathDestroy(IntPtr handle);

    [DllImport("GfxPluginPathfinder")]
    private static extern void PFPathMoveTo(IntPtr handle, ref PFPoint2DF to);

    public PFPath() {
        handle = PFPathCreate();
    }

    private void ensureNotConsumed() {
        if (handle == IntPtr.Zero) {
            throw new Exception("Path is already consumed!");
        }
    }

    public void MoveTo(Vector2 to) {
        ensureNotConsumed();
        var point = new PFPoint2DF(to.x, to.y);
        PFPathMoveTo(handle, ref point);
    }

    ~PFPath() {
        if (handle != IntPtr.Zero) {
            PFPathDestroy(handle);
        }
    }
}

public class PathfinderCameraScript : MonoBehaviour
{
    [DllImport("GfxPluginPathfinder")]
    private static extern IntPtr get_render_event_func();

    // Start is called before the first frame update
    void Start()
    {
        // TODO: This is temporary code, remove it.
        var path = new PFPath();
        path.MoveTo(new Vector2(5.0f, 10.0f));
    }

    public void OnPostRender() {
        GL.IssuePluginEvent(get_render_event_func(), 1);
    }

    // Update is called once per frame
    void Update()
    {
        if (Input.GetKey("escape")) {
            Application.Quit();
        }
    }
}
