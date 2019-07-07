using UnityEngine;

public class RenderTextureCameraScript : MonoBehaviour
{
    public GameObject globalState;
    private GlobalState gState;
    private Camera ourCamera;
    public float fontSize;
    private float fontVelocity;
    private float maxFontSize;
    private float minFontSize;

    // Start is called before the first frame update
    void Start()
    {
        gState = globalState.GetComponent<GlobalState>();
        ourCamera = GetComponent<Camera>();
        fontSize = 10.0f;
        fontVelocity = 1.0f;
        maxFontSize = 160.0f;
        minFontSize = 10.0f;
    }

    public void OnPostRender() {
        if (!gState.IsPathfinderEnabled()) {
            return;
        }

        var tex = ourCamera.targetTexture;
        var size = new Vector2(tex.width, tex.height);
        var canvas = new PFCanvas(gState.GetFontContext(), size);

        canvas.SetFontSize(fontSize);
        canvas.FillText("Yo!", new Vector2(10.0f, fontSize));

        canvas.QueueForRendering();
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
