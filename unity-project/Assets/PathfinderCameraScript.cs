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

    public static PFPoint2DF FromVector(Vector2 v) {
        return new PFPoint2DF(v.x, v.y);
    }
}

[Serializable]
[StructLayout(LayoutKind.Sequential)]
struct PFPoint2DI {
    public Int32 x;
    public Int32 y;

    public PFPoint2DI(Int32 x, Int32 y) {
        this.x = x;
        this.y = y;
    }

    public static PFPoint2DI FromVector(Vector2Int v) {
        return new PFPoint2DI(v.x, v.y);
    }
}

[Serializable]
[StructLayout(LayoutKind.Sequential)]
struct PFRectF {
    public PFPoint2DF origin;
    public PFPoint2DF lower_right;

    public PFRectF(PFPoint2DF origin, PFPoint2DF lower_right) {
        this.origin = origin;
        this.lower_right = lower_right;
    }

    public static PFRectF FromRect(Rect r) {
        var origin = new PFPoint2DF(r.xMin, r.yMin);
        var lower_right = new PFPoint2DF(r.xMax, r.yMax);
        return new PFRectF(origin, lower_right);
    }
}

class PFCanvas {
    private IntPtr handle;

    [DllImport("GfxPluginPathfinder")]
    private static extern IntPtr PFCanvasCreate(ref PFPoint2DF size);

    [DllImport("GfxPluginPathfinder")]
    private static extern void PFCanvasDestroy(IntPtr handle);

    [DllImport("GfxPluginPathfinder")]
    private static extern void PFCanvasSetLineWidth(IntPtr handle, float new_line_width);

    [DllImport("GfxPluginPathfinder")]
    private static extern void PFCanvasStrokeRect(IntPtr handle, ref PFRectF rect);

    [DllImport("GfxPluginPathfinder")]
    private static extern void PFCanvasFillRect(IntPtr handle, ref PFRectF rect);

    [DllImport("GfxPluginPathfinder")]
    private static extern void PFCanvasStrokePath(IntPtr handle, IntPtr pathHandle);

    [DllImport("GfxPluginPathfinder")]
    private static extern void queue_canvas_for_rendering(IntPtr handle);

    public PFCanvas(Vector2 size) {
        var pfSize = PFPoint2DF.FromVector(size);
        handle = PFCanvasCreate(ref pfSize);
    }

    private void EnsureHandleIsValid() {
        if (handle == IntPtr.Zero) {
            throw new Exception("Canvas has already been consumed!");
        }
    }

    public void SetLineWidth(float width) {
        EnsureHandleIsValid();
        PFCanvasSetLineWidth(handle, width);
    }

    public void StrokeRect(Rect rect) {
        EnsureHandleIsValid();
        var pfRect = PFRectF.FromRect(rect);
        PFCanvasStrokeRect(handle, ref pfRect);
    }

    public void FillRect(Rect rect) {
        EnsureHandleIsValid();
        var pfRect = PFRectF.FromRect(rect);
        PFCanvasFillRect(handle, ref pfRect);
    }

    public void StrokePath(PFPath path) {
        EnsureHandleIsValid();
        var pathHandleToConsume = path.PrepareToConsume();
        PFCanvasStrokePath(handle, pathHandleToConsume);
    }

    public void QueueForRendering() {
        queue_canvas_for_rendering(handle);
        handle = IntPtr.Zero;
    }

    ~PFCanvas() {
        if (handle != IntPtr.Zero) {
            PFCanvasDestroy(handle);
        }
    }
}

class PFPath {
    internal IntPtr handle;

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

    internal IntPtr PrepareToConsume() {
        var oldHandle = handle;
        handle = PFPathClone(handle);
        return oldHandle;
    }

    public void MoveTo(Vector2 to) {
        var point = PFPoint2DF.FromVector(to);
        PFPathMoveTo(handle, ref point);
    }

    public void LineTo(Vector2 to) {
        var point = PFPoint2DF.FromVector(to);
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
    }

    public void OnPostRender() {
        // Make a canvas. We're going to draw a house.
        var canvas = new PFCanvas(new Vector2(Screen.width, Screen.height));

        canvas.SetLineWidth(10.0f);

        // Draw walls.
        canvas.StrokeRect(new Rect(75.0f, 140.0f, 150.0f, 110.0f));

        // Draw door.
        canvas.FillRect(new Rect(130.0f, 190.0f, 40.0f, 60.0f));

        // Draw roof.
        var path = new PFPath();
        path.MoveTo(new Vector2(50.0f, 140.0f));
        path.LineTo(new Vector2(150.0f, 60.0f));
        path.LineTo(new Vector2(250.0f, 140.0f));
        path.ClosePath();
        canvas.StrokePath(path);

        canvas.QueueForRendering();

        GL.IssuePluginEvent(get_render_event_func(), 1);

        // This is temporary code just to make sure calls don't crash.
        path.Clone();
    }

    // Update is called once per frame
    void Update()
    {
        if (Input.GetKey("escape")) {
            Application.Quit();
        }
    }
}
