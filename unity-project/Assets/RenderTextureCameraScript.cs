using UnityEngine;

public class RenderTextureCameraScript : MonoBehaviour
{
    private PFCanvasFontContext fontContext;
    public float fontSize;
    private float fontVelocity;
    private float maxFontSize;
    private float minFontSize;

    // Start is called before the first frame update
    void Start()
    {
        fontContext = new PFCanvasFontContext();
        fontSize = 10.0f;
        fontVelocity = 1.0f;
        maxFontSize = 160.0f;
        minFontSize = 10.0f;
    }

    public void OnPostRender() {
        var canvas = new PFCanvas(fontContext, new Vector2(256, 256));

        canvas.SetFontSize(fontSize);
        canvas.FillText("Yo!", new Vector2(10.0f, fontSize));

        canvas.QueueForRendering(2);
    }

    public void Update() {
        fontSize += fontVelocity;
        if (fontSize > maxFontSize) {
            fontSize = maxFontSize;
            fontVelocity = -Mathf.Abs(fontVelocity);
        } else if (fontSize < minFontSize) {
            fontSize = minFontSize;
            fontVelocity = Mathf.Abs(fontVelocity);
        }
    }
}
