using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Runtime.InteropServices;

public class NewBehaviourScript : MonoBehaviour
{
    [DllImport("pathfinder_c_api_fun")]

    private static extern int boop_stdcall(int val);

    // Start is called before the first frame update
    void Start()
    {
        print("Result of pathfinder plugin FFI call is: " + boop_stdcall(50));
    }

    // Update is called once per frame
    void Update()
    {
        if (Input.GetKey("escape")) {
            Application.Quit();
        }
    }
}
