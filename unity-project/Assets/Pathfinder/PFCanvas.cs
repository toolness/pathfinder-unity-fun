using System;
using UnityEngine;

public enum PFLineJoin : byte {
    Miter = PF.PF_LINE_JOIN_MITER,
    Bevel = PF.PF_LINE_JOIN_BEVEL,
    Round = PF.PF_LINE_JOIN_ROUND
}

public class PFCanvas {
    private IntPtr handle;

    public PFCanvas(PFCanvasFontContext fontContext, Vector2 size) {
        var pfSize = PFUnityConv.PFVector2F(size);
        handle = PF.PFCanvasCreate(fontContext.PrepareToConsume(), ref pfSize);
    }

    private void EnsureHandleIsValid() {
        if (handle == IntPtr.Zero) {
            throw new Exception("Canvas has already been consumed!");
        }
    }

    public void SetLineJoin(PFLineJoin join) {
        PF.PFCanvasSetLineJoin(handle, (byte) join);
    }

    public void SetFillStyle(PFFillStyle style) {
        PF.PFCanvasSetFillStyle(handle, style.handle);
    }

    public void SetStrokeStyle(PFFillStyle style) {
        PF.PFCanvasSetStrokeStyle(handle, style.handle);
    }

    public void SetLineWidth(float width) {
        EnsureHandleIsValid();
        PF.PFCanvasSetLineWidth(handle, width);
    }

    public void StrokeRect(Rect rect) {
        EnsureHandleIsValid();
        var pfRect = PFUnityConv.PFRectF(rect);
        PF.PFCanvasStrokeRect(handle, ref pfRect);
    }

    public void FillText(PFString str, Vector2 position) {
        var pfVector = PFUnityConv.PFVector2F(position);
        PF.PFCanvasFillText(handle, str.handle, str.len, ref pfVector);
    }

    public void StrokeText(PFString str, Vector2 position) {
        var pfVector = PFUnityConv.PFVector2F(position);
        PF.PFCanvasStrokeText(handle, str.handle, str.len, ref pfVector);
    }

    public void SetFontSize(float size) {
        PF.PFCanvasSetFontSize(handle, size);
    }

    public void FillRect(Rect rect) {
        EnsureHandleIsValid();
        var pfRect = PFUnityConv.PFRectF(rect);
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