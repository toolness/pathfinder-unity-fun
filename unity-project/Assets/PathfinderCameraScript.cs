using UnityEngine;

public class PathfinderCameraScript : MonoBehaviour
{
    public GameObject globalState;
    private GlobalState gState;

    // Start is called before the first frame update
    void Start()
    {
        gState = globalState.GetComponent<GlobalState>();
    }

    public void OnPostRender() {
        if (!gState.IsPathfinderEnabled()) return;

        // Make a canvas. We're going to draw a house.
        var canvas = new PFCanvas(gState.GetFontContext(), new Vector2(Screen.width, Screen.height));

        canvas.SetStrokeStyle(Color.blue);
        canvas.SetFillStyle(Color.green);
        canvas.SetLineJoin(PFLineJoin.Round);
        canvas.SetLineWidth(10.0f);

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

        canvas.SetFillStyle(Color.black);
        canvas.SetFontSize(24.0f);
        canvas.FillText(
            "Press “" + InputScript.pathfinderToggleKey + "” to toggle Pathfinder.",
            new Vector2(10.0f, 40.0f)
        );

        canvas.QueueForRendering();

        // This is temporary code just to make sure calls don't crash.
        path.Clone();
    }
}
