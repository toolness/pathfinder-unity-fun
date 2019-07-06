using UnityEngine;

public class RenderTextureCameraScript : MonoBehaviour
{
    private PFCanvasFontContext fontContext;

    // Start is called before the first frame update
    void Start()
    {
        fontContext = new PFCanvasFontContext();
    }

    public void OnPostRender() {
        var canvas = new PFCanvas(fontContext, new Vector2(256, 256));

        canvas.SetStrokeStyle(Color.red);
        canvas.SetFontSize(128.0f);
        canvas.FillText("Yo!", new Vector2(10.0f, 128.0f));

        canvas.QueueForRendering(2);
    }
}
