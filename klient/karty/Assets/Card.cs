using UnityEngine;
using UnityEngine.UI;
using System.Collections;

public class Card : MonoBehaviour {

	// Use this for initialization
	void Start () {

	}
	
	// Update is called once per frame
	void Update () {
	
	}

    public void SetCard(string name){
        GetComponent<Image>().sprite = Resources.Load("cards/"+name) as Sprite;

    }
}
