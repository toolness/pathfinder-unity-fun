using System;
using UnityEngine;

public class PFCanvas {
    private IntPtr handle;

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
        PFPluginExports.queue_canvas_for_rendering(handle);
        handle = IntPtr.Zero;
        GL.IssuePluginEvent(PFPluginExports.get_render_event_func(), 1);
    }

    ~PFCanvas() {
        if (handle != IntPtr.Zero) {
            PF.PFCanvasDestroy(handle);
        }
    }
}
