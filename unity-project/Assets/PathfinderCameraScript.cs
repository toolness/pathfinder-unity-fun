using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System;
using System.Runtime.InteropServices;

struct PFUnity {
    public static PFRectF PFRectF(Rect r) {
        var origin = new PFVector2F(r.xMin, r.yMin);
        var lower_right = new PFVector2F(r.xMax, r.yMax);
        return new PFRectF(origin, lower_right);
    }

    public static PFVector2I PFVector2I(Vector2Int v) {
        return new PFVector2I(v.x, v.y);
    }

    public static PFVector2F PFVector2F(Vector2 v) {
        return new PFVector2F(v.x, v.y);
    }
}

class PFCanvasFontContext {
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

class PFCanvas {
    private IntPtr handle;

    [DllImport("GfxPluginPathfinder")]
    private static extern void queue_canvas_for_rendering(IntPtr handle);

    public PFCanvas(PFCanvasFontContext fontContext, Vector2 size) {
        var pfSize = PFUnity.PFVector2F(size);
        handle = PF.PFCanvasCreate(fontContext.PrepareToConsume(), ref pfSize);
    }

    private void EnsureHandleIsValid() {
        if (handle == IntPtr.Zero) {
            throw new Exception("Canvas has already been consumed!");
        }
    }

    public void SetLineWidth(float width) {
        EnsureHandleIsValid();
        PF.PFCanvasSetLineWidth(handle, width);
    }

    public void StrokeRect(Rect rect) {
        EnsureHandleIsValid();
        var pfRect = PFUnity.PFRectF(rect);
        PF.PFCanvasStrokeRect(handle, ref pfRect);
    }

    public void FillRect(Rect rect) {
        EnsureHandleIsValid();
        var pfRect = PFUnity.PFRectF(rect);
        PF.PFCanvasFillRect(handle, ref pfRect);
    }

    public void StrokePath(PFPath path) {
        EnsureHandleIsValid();
        var pathHandleToConsume = path.PrepareToConsume();
        PF.PFCanvasStrokePath(handle, pathHandleToConsume);
    }

    public void FillPath(PFPath path) {
        EnsureHandleIsValid();
        var pathHandleToConsume = path.PrepareToConsume();
        PF.PFCanvasFillPath(handle, pathHandleToConsume);
    }

    public void QueueForRendering() {
        queue_canvas_for_rendering(handle);
        handle = IntPtr.Zero;
    }

    ~PFCanvas() {
        if (handle != IntPtr.Zero) {
            PF.PFCanvasDestroy(handle);
        }
    }
}

class PFPath {
    internal IntPtr handle;

    public PFPath(PFPath targetToClone = null) {
        if (targetToClone != null) {
            handle = PF.PFPathClone(targetToClone.handle);
        } else {
            handle = PF.PFPathCreate();
        }
    }

    internal IntPtr PrepareToConsume() {
        var oldHandle = handle;
        handle = PF.PFPathClone(handle);
        return oldHandle;
    }

    public void MoveTo(Vector2 to) {
        var pfTo = PFUnity.PFVector2F(to);
        PF.PFPathMoveTo(handle, ref pfTo);
    }

    public void LineTo(Vector2 to) {
        var pfTo = PFUnity.PFVector2F(to);
        PF.PFPathLineTo(handle, ref pfTo);
    }

    public void QuadraticCurveTo(Vector2 ctrl, Vector2 to) {
        var pfCtrl = PFUnity.PFVector2F(ctrl);
        var pfTo = PFUnity.PFVector2F(to);
        PF.PFPathQuadraticCurveTo(handle, ref pfCtrl, ref pfTo);
    }

    public void BezierCurveTo(Vector2 ctrl0, Vector2 ctrl1, Vector2 to) {
        var pfCtrl0 = PFUnity.PFVector2F(ctrl0);
        var pfCtrl1 = PFUnity.PFVector2F(ctrl1);
        var pfTo = PFUnity.PFVector2F(to);
        PF.PFPathBezierCurveTo(handle, ref pfCtrl0, ref pfCtrl1, ref pfTo);
    }

    public PFPath Clone() {
        return new PFPath(this);
    }

    public void ClosePath() {
        PF.PFPathClosePath(handle);
    }

    ~PFPath() {
        PF.PFPathDestroy(handle);
    }
}

public class PathfinderCameraScript : MonoBehaviour
{
    [DllImport("GfxPluginPathfinder")]
    private static extern IntPtr get_render_event_func();

    private PFCanvasFontContext fontContext;

    // Start is called before the first frame update
    void Start()
    {
        fontContext = new PFCanvasFontContext();
    }

    public void OnPostRender() {
        // Make a canvas. We're going to draw a house.
        var canvas = new PFCanvas(fontContext, new Vector2(Screen.width, Screen.height));

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
