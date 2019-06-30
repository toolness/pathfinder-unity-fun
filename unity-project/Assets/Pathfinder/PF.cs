// This file has been auto-generated, please do not edit it.

using System;
using System.Runtime.InteropServices;

[Serializable]
[StructLayout(LayoutKind.Sequential)]
internal struct PFTextMetrics {
    internal float width;

    internal PFTextMetrics(float width) {
        this.width = width;
    }
}

[Serializable]
[StructLayout(LayoutKind.Sequential)]
internal struct PFColorF {
    internal float r;
    internal float g;
    internal float b;
    internal float a;

    internal PFColorF(float r, float g, float b, float a) {
        this.r = r;
        this.g = g;
        this.b = b;
        this.a = a;
    }
}

[Serializable]
[StructLayout(LayoutKind.Sequential)]
internal struct PFColorU {
    internal byte r;
    internal byte g;
    internal byte b;
    internal byte a;

    internal PFColorU(byte r, byte g, byte b, byte a) {
        this.r = r;
        this.g = g;
        this.b = b;
        this.a = a;
    }
}

[Serializable]
[StructLayout(LayoutKind.Sequential)]
internal struct PFVector2F {
    internal float x;
    internal float y;

    internal PFVector2F(float x, float y) {
        this.x = x;
        this.y = y;
    }
}

[Serializable]
[StructLayout(LayoutKind.Sequential)]
internal struct PFVector2I {
    internal Int32 x;
    internal Int32 y;

    internal PFVector2I(Int32 x, Int32 y) {
        this.x = x;
        this.y = y;
    }
}

[Serializable]
[StructLayout(LayoutKind.Sequential)]
internal struct PFRectF {
    internal PFVector2F origin;
    internal PFVector2F lower_right;

    internal PFRectF(PFVector2F origin, PFVector2F lower_right) {
        this.origin = origin;
        this.lower_right = lower_right;
    }
}

[Serializable]
[StructLayout(LayoutKind.Sequential)]
internal struct PFRectI {
    internal PFVector2I origin;
    internal PFVector2I lower_right;

    internal PFRectI(PFVector2I origin, PFVector2I lower_right) {
        this.origin = origin;
        this.lower_right = lower_right;
    }
}

[Serializable]
[StructLayout(LayoutKind.Sequential)]
internal struct PFBuildOptions {
    internal UInt32 placeholder;

    internal PFBuildOptions(UInt32 placeholder) {
        this.placeholder = placeholder;
    }
}

internal class PF {
    internal const byte PF_LINE_CAP_BUTT = 0;

    internal const byte PF_LINE_CAP_SQUARE = 1;

    internal const byte PF_LINE_CAP_ROUND = 2;

    internal const byte PF_LINE_JOIN_MITER = 0;

    internal const byte PF_LINE_JOIN_BEVEL = 1;

    internal const byte PF_LINE_JOIN_ROUND = 2;

    internal const byte PF_ARC_DIRECTION_CW = 0;

    internal const byte PF_ARC_DIRECTION_CCW = 1;

    internal const byte PF_RENDERER_OPTIONS_FLAGS_HAS_BACKGROUND_COLOR = 1;

    [DllImport("GfxPluginPathfinder")]
    internal static extern IntPtr /* CanvasRenderingContext2D */ PFCanvasCreate(IntPtr /* CanvasFontContext */ font_context, ref PFVector2F size);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFCanvasDestroy(IntPtr /* CanvasRenderingContext2D */ canvas);

    [DllImport("GfxPluginPathfinder")]
    internal static extern IntPtr /* CanvasFontContext */ PFCanvasFontContextCreateWithSystemSource();

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFCanvasFontContextDestroy(IntPtr /* CanvasFontContext */ font_context);

    [DllImport("GfxPluginPathfinder")]
    internal static extern IntPtr /* CanvasFontContext */ PFCanvasFontContextClone(IntPtr /* CanvasFontContext */ font_context);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFCanvasFillRect(IntPtr /* CanvasRenderingContext2D */ canvas, ref PFRectF rect);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFCanvasStrokeRect(IntPtr /* CanvasRenderingContext2D */ canvas, ref PFRectF rect);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFCanvasFillText(IntPtr /* CanvasRenderingContext2D */ canvas, IntPtr /* c_char */ str, UIntPtr string_len, ref PFVector2F position);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFCanvasStrokeText(IntPtr /* CanvasRenderingContext2D */ canvas, IntPtr /* c_char */ str, UIntPtr string_len, ref PFVector2F position);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFCanvasMeasureText(IntPtr /* CanvasRenderingContext2D */ canvas, IntPtr /* c_char */ str, UIntPtr string_len, ref PFTextMetrics out_text_metrics);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFCanvasSetLineWidth(IntPtr /* CanvasRenderingContext2D */ canvas, float new_line_width);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFCanvasSetLineCap(IntPtr /* CanvasRenderingContext2D */ canvas, byte new_line_cap);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFCanvasSetLineJoin(IntPtr /* CanvasRenderingContext2D */ canvas, byte new_line_join);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFCanvasSetMiterLimit(IntPtr /* CanvasRenderingContext2D */ canvas, float new_miter_limit);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFCanvasSetLineDash(IntPtr /* CanvasRenderingContext2D */ canvas, IntPtr /* float */ new_line_dashes, UIntPtr new_line_dash_count);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFCanvasSetLineDashOffset(IntPtr /* CanvasRenderingContext2D */ canvas, float new_offset);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFCanvasSetFontByPostScriptName(IntPtr /* CanvasRenderingContext2D */ canvas, IntPtr /* c_char */ postscript_name, UIntPtr postscript_name_len);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFCanvasSetFontSize(IntPtr /* CanvasRenderingContext2D */ canvas, float new_font_size);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFCanvasSetFillStyle(IntPtr /* CanvasRenderingContext2D */ canvas, IntPtr /* FillStyle */ fill_style);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFCanvasSetStrokeStyle(IntPtr /* CanvasRenderingContext2D */ canvas, IntPtr /* FillStyle */ stroke_style);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFCanvasFillPath(IntPtr /* CanvasRenderingContext2D */ canvas, IntPtr /* Path2D */ path);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFCanvasStrokePath(IntPtr /* CanvasRenderingContext2D */ canvas, IntPtr /* Path2D */ path);

    [DllImport("GfxPluginPathfinder")]
    internal static extern IntPtr /* Path2D */ PFPathCreate();

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFPathDestroy(IntPtr /* Path2D */ path);

    [DllImport("GfxPluginPathfinder")]
    internal static extern IntPtr /* Path2D */ PFPathClone(IntPtr /* Path2D */ path);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFPathMoveTo(IntPtr /* Path2D */ path, ref PFVector2F to);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFPathLineTo(IntPtr /* Path2D */ path, ref PFVector2F to);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFPathQuadraticCurveTo(IntPtr /* Path2D */ path, ref PFVector2F ctrl, ref PFVector2F to);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFPathBezierCurveTo(IntPtr /* Path2D */ path, ref PFVector2F ctrl0, ref PFVector2F ctrl1, ref PFVector2F to);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFPathArc(IntPtr /* Path2D */ path, ref PFVector2F center, float radius, float start_angle, float end_angle, byte direction);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFPathArcTo(IntPtr /* Path2D */ path, ref PFVector2F ctrl, ref PFVector2F to, float radius);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFPathRect(IntPtr /* Path2D */ path, ref PFRectF rect);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFPathEllipse(IntPtr /* Path2D */ path, ref PFVector2F center, ref PFVector2F axes, float rotation, float start_angle, float end_angle);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFPathClosePath(IntPtr /* Path2D */ path);

    [DllImport("GfxPluginPathfinder")]
    internal static extern IntPtr /* FillStyle */ PFFillStyleCreateColor(ref PFColorU color);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFFillStyleDestroy(IntPtr /* FillStyle */ fill_style);

    [DllImport("GfxPluginPathfinder")]
    internal static extern IntPtr /* Box */ PFFilesystemResourceLoaderLocate();

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFResourceLoaderDestroy(IntPtr /* Box */ loader);

}
