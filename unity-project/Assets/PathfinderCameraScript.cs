using UnityEngine;
using System;
using System.Runtime.InteropServices;

public class PathfinderCameraScript : MonoBehaviour
{
    private PFCanvasFontContext fontContext;

    // Start is called before the first frame update
    void Start()
    {
        fontContext = new PFCanvasFontContext();
    }

    public void OnPostRender() {
        // Make a canvas. We're going to draw a house.
        var canvas = new PFCanvas(fontContext, new Vector2(Screen.width, Screen.height));

        canvas.SetStrokeStyle(PFFillStyle.CreateColor(Color.blue));
        canvas.SetFillStyle(PFFillStyle.CreateColor(Color.green));
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

        canvas.SetFillStyle(PFFillStyle.CreateColor(Color.black));
        canvas.SetFontSize(24.0f);
        canvas.FillText("Hello world\u2026", new Vector2(10.0f, 40.0f));

        canvas.QueueForRendering();

        // This is temporary code just to make sure calls don't crash.
        path.Clone();
    }

    // Update is called once per frame
    void Update()
    {
        if (Input.GetKey("escape")) {
            Application.Quit();
        }
    }
}
