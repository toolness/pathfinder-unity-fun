using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class DeletePlayerPrefsScript : MonoBehaviour
{
    // Start is called before the first frame update
    void Start()
    {
        // We don't save any player preferences, so there's not really much
        // to lose by deleting them AFAIK, and doing this makes it easier
        // for some changes in the Unity player settings to take effect,
        // such as the player's window size.
        //
        // Note, though, that this won't really do much the first time
        // it's run, as the initial window size will have already been
        // loaded from the player prefs. It will take effect once the
        // player is restarted, though.
        PlayerPrefs.DeleteAll();                
    }
}
