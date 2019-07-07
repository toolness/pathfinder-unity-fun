using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class InputScript : MonoBehaviour
{
    public static readonly string pathfinderToggleKey = "p";
    private GlobalStateScript gState;

    void Start() {
        gState = GetComponent<GlobalStateScript>();
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
