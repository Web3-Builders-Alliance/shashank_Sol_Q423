using System.Collections;
using System.Collections.Generic;
using TMPro;
using UnityEngine;

public class DisplayPublicKey : MonoBehaviour
{
    private TextMeshProUGUI _txtPublicKey;
    // Start is called before the first frame update
    void Start()
    {
        _txtPublicKey = GetComponent<TextMeshProUGUI>();
    }


    private void OnLogin(Account account)
    {
        _txtPublicKey.text = account.PublicKey+"";
        Debug.Log(account.PublicKey+"");

    }
}
