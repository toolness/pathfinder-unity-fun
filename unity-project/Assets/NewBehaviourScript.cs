using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System;
using System.Runtime.InteropServices;

public class NewBehaviourScript : MonoBehaviour
{
    [DllImport("GfxPluginPathfinder")]
    private static extern int boop_stdcall(int val);

    [DllImport("GfxPluginPathfinder")]
    private static extern IntPtr get_render_event_func();

    // Start is called before the first frame update
    void Start()
    {
        print("Result of pathfinder plugin FFI call is: " + boop_stdcall(50));

        // We don't save any player preferences, so there's not really much
        // to lose by deleting them AFAIK, and doing this makes it easier
        // for some changes in the Unity player settings to take effect,
        // such as the player's window size.
        PlayerPrefs.DeleteAll();
    }

    public void OnPostRender() {
        GL.IssuePluginEvent(get_render_event_func(), 1);
    }

    // Update is called once per frame
    void Update()
    {
        if (Input.GetKey("escape")) {
            Application.Quit();
        }
    }
}
