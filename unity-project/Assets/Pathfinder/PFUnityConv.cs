using UnityEngine;

internal class PFUnityConv {
    internal static PFRectF PFRectF(Rect r) {
        var origin = new PFVector2F(r.xMin, r.yMin);
        var lower_right = new PFVector2F(r.xMax, r.yMax);
        return new PFRectF(origin, lower_right);
    }

    internal static PFVector2I PFVector2I(Vector2Int v) {
        return new PFVector2I(v.x, v.y);
    }

    internal static PFVector2F PFVector2F(Vector2 v) {
        return new PFVector2F(v.x, v.y);
    }

    internal static PFColorU PFColorU(Color32 c) {
        return new PFColorU(c.r, c.g, c.b, c.a);
    }
}
