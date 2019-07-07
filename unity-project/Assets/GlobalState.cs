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
        if (fontContext == null) {
            // This will only release the plugin's resources for the current
            // GL context at the time that it's called, so we'll intentionally
            // call it frequently in hopes that it will eventually release
            // the resources from all GL contexts that Unity uses.
            PFPlugin.ReleaseResources();
        }
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
            } else {
                fontContext = new PFCanvasFontContext();
            }
        }
        if (Input.GetKey("escape")) {
            Application.Quit();
        }
    }
}
