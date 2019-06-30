using UnityEngine;
using System;

public class PFPath {
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
        var pfTo = PFUnityConv.PFVector2F(to);
        PF.PFPathMoveTo(handle, ref pfTo);
    }

    public void LineTo(Vector2 to) {
        var pfTo = PFUnityConv.PFVector2F(to);
        PF.PFPathLineTo(handle, ref pfTo);
    }

    public void QuadraticCurveTo(Vector2 ctrl, Vector2 to) {
        var pfCtrl = PFUnityConv.PFVector2F(ctrl);
        var pfTo = PFUnityConv.PFVector2F(to);
        PF.PFPathQuadraticCurveTo(handle, ref pfCtrl, ref pfTo);
    }

    public void BezierCurveTo(Vector2 ctrl0, Vector2 ctrl1, Vector2 to) {
        var pfCtrl0 = PFUnityConv.PFVector2F(ctrl0);
        var pfCtrl1 = PFUnityConv.PFVector2F(ctrl1);
        var pfTo = PFUnityConv.PFVector2F(to);
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
