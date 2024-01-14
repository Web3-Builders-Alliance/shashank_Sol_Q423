using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

public class Send : MonoBehaviour
{

    public Button btnClick;
    public InputField input;

    private void Start()
    {
        btnClick.onClick.AddListener(getInputOnClick);
    }

    public void getInputOnClick()
    {
        Debug.Log(input.text);
        
    }
}
