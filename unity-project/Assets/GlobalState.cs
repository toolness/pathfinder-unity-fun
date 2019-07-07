using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class GlobalState : MonoBehaviour
{
    public static readonly string pathfinderToggleKey = "a";
    private PFCanvasFontContext fontContext;

    // Start is called before the first frame update
    void Start()
    {
        fontContext = new PFCanvasFontContext();
    }

    public bool IsPathfinderEnabled() {
        return fontContext != null;
    }

    public PFCanvasFontContext GetFontContext() {
        return fontContext;
    }

    // Update is called once per frame
    void Update()
    {
        if (Input.GetKeyUp(pathfinderToggleKey)) {
            if (fontContext != null) {
                fontContext = null;
                PFPlugin.ReleaseResources();
            } else {
                fontContext = new PFCanvasFontContext();
            }
        }
        if (Input.GetKey("escape")) {
            Application.Quit();
        }
    }
}
