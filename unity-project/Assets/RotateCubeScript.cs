using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class RotateCubeScript : MonoBehaviour
{
    private float angle = 0.0f;

    // Start is called before the first frame update
    void Start()
    {
        
    }

    // Update is called once per frame
    void Update()
    {
        var transform = GetComponent<Transform>();
        angle += 1.0f;
        transform.rotation = Quaternion.AngleAxis(angle, new Vector3(0, 1, 1));        
    }
}
