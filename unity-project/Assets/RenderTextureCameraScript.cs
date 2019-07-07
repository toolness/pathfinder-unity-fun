using UnityEngine;

public class RenderTextureCameraScript : MonoBehaviour
{
    public GameObject globalState;
    private GlobalStateScript gState;
    private RenderTexture renderTexture;

    private const float VELOCITY = 0.02f;
    private const float OUTER_RADIUS = 64.0f;
    private const float INNER_RADIUS = 48.0f;
    private const int CIRCLE_COUNT = 12;
    private const float CIRCLE_SPACING = 48.0f;
    private const float CIRCLE_THICKNESS = 16.0f;
    private const float COLOR_CYCLE_SPEED = 0.0025f;

    // Start is called before the first frame update
    void Start()
    {
        gState = globalState.GetComponent<GlobalStateScript>();
        renderTexture = GetComponent<Camera>().targetTexture;
    }

    public void OnPostRender() {
        if (!gState.IsPathfinderEnabled()) {
            return;
        }

        var drawableSize = new Vector2(renderTexture.width, renderTexture.height);
        float time = Time.frameCount;
        var sinTime = Mathf.Sin(time * VELOCITY);
        var cosTime = Mathf.Cos(time * VELOCITY);
        var colorTime = time * COLOR_CYCLE_SPEED;
        // TODO: Calculate background color from gradient and set it.
        // TODO: Calculate foreground color from gradient.
        var fgColor = Color.black;
        fgColor.a = 0.75f;

        var windowCenter = drawableSize * 0.5f;
        var outerCenter = windowCenter + OUTER_RADIUS * new Vector2(sinTime, cosTime);
        var innerCenter = windowCenter + cosTime * INNER_RADIUS * new Vector2(1.0f, sinTime);

        var canvas = new PFCanvas(gState.GetFontContext(), drawableSize);
        canvas.SetLineWidth(CIRCLE_THICKNESS);
        canvas.SetStrokeStyle(fgColor);

        drawCircles(canvas, outerCenter);
        drawCircles(canvas, innerCenter);

        canvas.QueueForRendering();
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
