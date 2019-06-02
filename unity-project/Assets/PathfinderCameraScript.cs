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
    private static extern IntPtr PFPathClone(IntPtr handle);

    [DllImport("GfxPluginPathfinder")]
    private static extern void PFPathClosePath(IntPtr handle);

    [DllImport("GfxPluginPathfinder")]
    private static extern void PFPathMoveTo(IntPtr handle, ref PFPoint2DF to);

    [DllImport("GfxPluginPathfinder")]
    private static extern void PFPathLineTo(IntPtr handle, ref PFPoint2DF to);

    public PFPath(PFPath targetToClone = null) {
        if (targetToClone != null) {
            handle = PFPathClone(targetToClone.handle);
        } else {
            handle = PFPathCreate();
        }
    }

    public void MoveTo(Vector2 to) {
        var point = new PFPoint2DF(to.x, to.y);
        PFPathMoveTo(handle, ref point);
    }

    public void LineTo(Vector2 to) {
        var point = new PFPoint2DF(to.x, to.y);
        PFPathLineTo(handle, ref point);
    }

    public PFPath Clone() {
        return new PFPath(this);
    }

    public void ClosePath() {
        PFPathClosePath(handle);
    }

    ~PFPath() {
        PFPathDestroy(handle);
    }
}

public class PathfinderCameraScript : MonoBehaviour
{
    [DllImport("GfxPluginPathfinder")]
    private static extern IntPtr get_render_event_func();

    // Start is called before the first frame update
    void Start()
    {
        // Draw roof.
        var path = new PFPath();
        path.MoveTo(new Vector2(50.0f, 140.0f));
        path.LineTo(new Vector2(150.0f, 60.0f));
        path.LineTo(new Vector2(250.0f, 140.0f));
        path.ClosePath();

        // This is temporary code just to make sure calls don't crash.
        path.Clone();
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
