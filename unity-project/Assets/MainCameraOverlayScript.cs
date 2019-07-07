using UnityEngine;

/// <summary>
/// This is a poor substitute for HTML Canvas' transformation API,
/// which we don't currently support. We should eventually support
/// it and then get rid of this stopgap.
/// </summary>
class GeomMaker {
    public float scale;
    public Vector2 translate;

    public GeomMaker(float scale, Vector2 translate) {
        this.scale = scale;
        this.translate = translate;
    }

    public Rect MakeRect(float x, float y, float width, float height) {
        return new Rect(
            new Vector2(x, y) * scale + translate,
            new Vector2(width, height) * scale
        );
    }

    public Vector2 MakeVector2(float x, float y) {
        return new Vector2(x, y) * scale + translate;
    }
}

public class MainCameraOverlayScript : MonoBehaviour
{
    public GameObject globalState;
    private GlobalStateScript gState;
    private PFString instructions;

    // Start is called before the first frame update
    void Start()
    {
        gState = globalState.GetComponent<GlobalStateScript>();
        instructions = new PFString("Press “" + InputScript.pathfinderToggleKey + "” to toggle Pathfinder.");
    }

    private void DrawHouse(PFCanvas canvas, GeomMaker geom) {
        canvas.SetStrokeStyle(Color.black);
        canvas.SetFillStyle(Color.black);
        canvas.SetLineJoin(PFLineJoin.Round);
        canvas.SetLineWidth(10.0f * geom.scale);

        // Draw walls.
        canvas.StrokeRect(geom.MakeRect(75.0f, 140.0f, 150.0f, 110.0f));

        // Draw door.
        canvas.FillRect(geom.MakeRect(130.0f, 190.0f, 40.0f, 60.0f));

        // Draw roof.
        var path = new PFPath();
        path.MoveTo(geom.MakeVector2(50.0f, 140.0f));
        path.LineTo(geom.MakeVector2(150.0f, 60.0f));
        path.LineTo(geom.MakeVector2(250.0f, 140.0f));
        path.ClosePath();
        canvas.StrokePath(path);
    }

    private void DrawInstructions(PFCanvas canvas) {
        canvas.SetFillStyle(Color.black);
        canvas.SetFontSize(24.0f);
        var textWidth = canvas.MeasureText(instructions).width;
        canvas.FillText(
            instructions,
            new Vector2(Screen.width / 2 - textWidth / 2, 40.0f)
        );
    }

    public void OnPostRender() {
        if (!gState.IsPathfinderEnabled()) return;

        var canvas = new PFCanvas(gState.GetFontContext(), new Vector2(Screen.width, Screen.height));

        DrawHouse(canvas, new GeomMaker(0.5f, new Vector2(0, Screen.height - 160)));
        DrawInstructions(canvas);

        canvas.QueueForRendering();
    }
}
