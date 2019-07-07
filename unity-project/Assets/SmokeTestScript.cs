using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class SmokeTestScript : MonoBehaviour
{
    private bool ranTests;

    void Start() {
        ranTests = false;
    }

    void Update()
    {
        if (ranTests) return;

        // Just some smoke tests to make sure we don't segfault.
        var gState = GetComponent<GlobalStateScript>();

        if (!gState.IsPathfinderEnabled()) return;

        var path = new PFPath();
        // We should be able to clone paths.
        path.Clone();

        var canvas = new PFCanvas(gState.GetFontContext(), new Vector2(10, 10));
        // We should be able to pass empty strings.
        canvas.FillText("", new Vector2(0, 0));

        ranTests = true;
        print("Successfully ran smoke tests.");
    }
}
