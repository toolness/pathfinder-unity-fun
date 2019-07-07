using UnityEngine;

public class RenderTextureCameraScript : MonoBehaviour
{
    public GameObject globalState;
    private GlobalStateScript gState;
    private Camera textureCamera;
    private ColorGradient colors;

    private const float VELOCITY = 0.02f;
    private const float OUTER_RADIUS = 64.0f;
    private const float INNER_RADIUS = 48.0f;
    private const int CIRCLE_COUNT = 2;
    private const float CIRCLE_SPACING = 48.0f;
    private const float CIRCLE_THICKNESS = 16.0f;
    private const float COLOR_CYCLE_SPEED = 0.0025f;

    // Start is called before the first frame update
    void Start()
    {
        gState = globalState.GetComponent<GlobalStateScript>();
        textureCamera = GetComponent<Camera>();
        colors = new ColorGradient(new [] {
            // Extracted from https://stock.adobe.com/69426938/
            new Color32(0x02, 0x48, 0x73, 0xff),
            new Color32(0x03, 0x65, 0x8c, 0xff),
            new Color32(0x03, 0x88, 0xa6, 0xff),
            new Color32(0xf2, 0x8e, 0x6b, 0xff),
            new Color32(0xd9, 0x5a, 0x4e, 0xff)
        });
    }

    public void OnPostRender() {
        var tex = textureCamera.targetTexture;
        var drawableSize = new Vector2(tex.width, tex.height);
        float time = Time.frameCount;
        var sinTime = Mathf.Sin(time * VELOCITY);
        var cosTime = Mathf.Cos(time * VELOCITY);
        var colorTime = time * COLOR_CYCLE_SPEED;
        var bgColor = colors.Sample(colorTime);
        var fgColor = colors.Sample(colorTime + 0.5f);
        fgColor.a = 0.75f;

        var windowCenter = drawableSize * 0.5f;
        var outerCenter = windowCenter + OUTER_RADIUS * new Vector2(sinTime, cosTime);
        var innerCenter = windowCenter + cosTime * INNER_RADIUS * new Vector2(1.0f, sinTime);

        textureCamera.backgroundColor = bgColor;

        if (gState.IsPathfinderEnabled()) {
            var canvas = new PFCanvas(gState.GetFontContext(), drawableSize);
            canvas.SetLineWidth(CIRCLE_THICKNESS);
            canvas.SetStrokeStyle(fgColor);

            drawCircles(canvas, outerCenter);
            drawCircles(canvas, innerCenter);

            canvas.QueueForRendering();
        }
    }

    private void drawCircles(PFCanvas canvas, Vector2 center) {
        for (int index = 0; index < CIRCLE_COUNT; index++) {
            var radius = (index + 1) * CIRCLE_SPACING;
            var path = new PFPath();
            path.Ellipse(center, new Vector2(radius, radius), 0, 0, Mathf.PI * 2.0f);
            canvas.StrokePath(path);
        }
    }
}

class ColorGradient {
    private Color32[] colors;

    public ColorGradient(Color32[] colors) {
        this.colors = colors;
    }

    public Color Sample(float t) {
        var count = colors.Length;
        t *= count;
        var lo = Mathf.FloorToInt(t) % count;
        var hi = Mathf.CeilToInt(t) % count;
        var fract = t - Mathf.Floor(t);
        return Color.Lerp(colors[lo], colors[hi], fract);
    }
}
