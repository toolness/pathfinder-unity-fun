using UnityEngine;


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

    private void DrawHouse(PFCanvas canvas) {
        var scale = 0.5f;

        canvas.Save();

        canvas.SetCurrentTransform(
            Matrix4x4.Translate(new Vector2(0, Screen.height - 160)) *
            Matrix4x4.Scale(new Vector2(scale, scale))
        );
        canvas.SetLineWidth(10.0f * scale);
        canvas.SetStrokeStyle(Color.black);
        canvas.SetFillStyle(Color.black);
        canvas.SetLineJoin(PFLineJoin.Round);

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

        canvas.Restore();
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

        DrawHouse(canvas);
        DrawInstructions(canvas);

        canvas.QueueForRendering();
    }
}
