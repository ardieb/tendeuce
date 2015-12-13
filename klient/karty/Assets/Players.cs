using UnityEngine;
using System.Collections;

public class Players : MonoBehaviour {

    public GameObject player0;
    public GameObject player1;
    public GameObject player2;
    public GameObject player3;
    public GameObject player4;
    public GameObject player5;
    public GameObject player6;
    public GameObject player7;
    public GameObject player8;
    public GameObject player9;

    // Use this for initialization
    void Start () {
	
	}

   
	
    public GameObject getPlayer (int number)
    {
        switch (number)
        {
            case 0:
                return player0;
                break;
            case 1:
                return player1;
                break;
            case 2:
                return player2;
                break;
            case 3:
                return player3;
                break;
            case 4:
                return player4;
                break;
            case 5:
                return player5;
                break;
            case 6:
                return player6;
                break;
            case 7:
                return player7;
                break;
            case 8:
                return player8;
                break;
            case 9:
                return player9;
                break;
            default:
                return null;
        }

    }

    // Update is called once per frame
    void Update () {
	
	}
}
