using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class InputScript : MonoBehaviour
{
    public static readonly string pathfinderToggleKey = "p";
    private GlobalState gState;

    void Start() {
        gState = GetComponent<GlobalState>();
    }

    // Update is called once per frame
    void Update()
    {
        if (Input.GetKeyUp(pathfinderToggleKey)) {
            gState.TogglePathfinderEnabled();
        }
        if (Input.GetKey("escape")) {
            Application.Quit();
        }
    }
}
