// This file has been auto-generated, please do not edit it.

using System;
using System.Runtime.InteropServices;

[Serializable]
[StructLayout(LayoutKind.Sequential)]
public struct PFTextMetrics {
    public float width;

    public PFTextMetrics(float width) {
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
internal struct PFMatrix2x2F {
    internal float m00;
    internal float m01;
    internal float m10;
    internal float m11;

    internal PFMatrix2x2F(float m00, float m01, float m10, float m11) {
        this.m00 = m00;
        this.m01 = m01;
        this.m10 = m10;
        this.m11 = m11;
    }
}

[Serializable]
[StructLayout(LayoutKind.Sequential)]
internal struct PFTransform2DF {
    internal PFMatrix2x2F matrix;
    internal PFVector2F vector;

    internal PFTransform2DF(PFMatrix2x2F matrix, PFVector2F vector) {
        this.matrix = matrix;
        this.vector = vector;
    }
}

[Serializable]
[StructLayout(LayoutKind.Sequential)]
internal struct PFTransform3DF {
    internal float m00;
    internal float m01;
    internal float m02;
    internal float m03;
    internal float m10;
    internal float m11;
    internal float m12;
    internal float m13;
    internal float m20;
    internal float m21;
    internal float m22;
    internal float m23;
    internal float m30;
    internal float m31;
    internal float m32;
    internal float m33;

    internal PFTransform3DF(float m00, float m01, float m02, float m03, float m10, float m11, float m12, float m13, float m20, float m21, float m22, float m23, float m30, float m31, float m32, float m33) {
        this.m00 = m00;
        this.m01 = m01;
        this.m02 = m02;
        this.m03 = m03;
        this.m10 = m10;
        this.m11 = m11;
        this.m12 = m12;
        this.m13 = m13;
        this.m20 = m20;
        this.m21 = m21;
        this.m22 = m22;
        this.m23 = m23;
        this.m30 = m30;
        this.m31 = m31;
        this.m32 = m32;
        this.m33 = m33;
    }
}

[Serializable]
[StructLayout(LayoutKind.Sequential)]
internal struct PFPerspective {
    internal PFTransform3DF transform;
    internal PFVector2I window_size;

    internal PFPerspective(PFTransform3DF transform, PFVector2I window_size) {
        this.transform = transform;
        this.window_size = window_size;
    }
}

internal class PF {
    internal const byte PF_LINE_CAP_BUTT = 0;

    internal const byte PF_LINE_CAP_SQUARE = 1;

    internal const byte PF_LINE_CAP_ROUND = 2;

    internal const byte PF_LINE_JOIN_MITER = 0;

    internal const byte PF_LINE_JOIN_BEVEL = 1;

    internal const byte PF_LINE_JOIN_ROUND = 2;

    internal const byte PF_TEXT_ALIGN_LEFT = 0;

    internal const byte PF_TEXT_ALIGN_CENTER = 1;

    internal const byte PF_TEXT_ALIGN_RIGHT = 2;

    internal const byte PF_ARC_DIRECTION_CW = 0;

    internal const byte PF_ARC_DIRECTION_CCW = 1;

    internal const byte PF_GL_VERSION_GL3 = 0;

    internal const byte PF_GL_VERSION_GLES3 = 1;

    internal const byte PF_RENDERER_OPTIONS_FLAGS_HAS_BACKGROUND_COLOR = 1;

    [DllImport("GfxPluginPathfinder")]
    internal static extern IntPtr /* CanvasRenderingContext2D */ PFCanvasCreate(IntPtr /* CanvasFontContext */ font_context, ref PFVector2F size);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFCanvasDestroy(IntPtr /* CanvasRenderingContext2D */ canvas);

    [DllImport("GfxPluginPathfinder")]
    internal static extern IntPtr /* CanvasFontContext */ PFCanvasFontContextCreateWithSystemSource();

    [DllImport("GfxPluginPathfinder")]
    internal static extern IntPtr /* CanvasFontContext */ PFCanvasFontContextAddRef(IntPtr /* CanvasFontContext */ font_context);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFCanvasFontContextRelease(IntPtr /* CanvasFontContext */ font_context);

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
    internal static extern void PFCanvasSetTextAlign(IntPtr /* CanvasRenderingContext2D */ canvas, byte new_text_align);

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
    internal static extern IntPtr /* RenderTransform */ PFRenderTransformCreate2D(ref PFTransform2DF transform);

    [DllImport("GfxPluginPathfinder")]
    internal static extern IntPtr /* RenderTransform */ PFRenderTransformCreatePerspective(ref PFPerspective perspective);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFRenderTransformDestroy(IntPtr /* RenderTransform */ transform);

    [DllImport("GfxPluginPathfinder")]
    internal static extern IntPtr /* BuildOptions */ PFBuildOptionsCreate();

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFBuildOptionsDestroy(IntPtr /* BuildOptions */ options);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFBuildOptionsSetTransform(IntPtr /* BuildOptions */ options, IntPtr /* RenderTransform */ transform);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFBuildOptionsSetDilation(IntPtr /* BuildOptions */ options, ref PFVector2F dilation);

    [DllImport("GfxPluginPathfinder")]
    internal static extern void PFBuildOptionsSetSubpixelAAEnabled(IntPtr /* BuildOptions */ options, bool subpixel_aa_enabled);

}
