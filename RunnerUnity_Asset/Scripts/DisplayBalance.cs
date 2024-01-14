using System.Collections;
using System.Collections.Generic;
using System.Globalization;
using TMPro;
using UnityEngine;

public class DisplayBalance : MonoBehaviour
{
    private TextMeshProUGUI _txtBalance;

    // Start is called before the first frame update
    void Start()
    {
        _txtBalance = GetComponent<TextMeshProUGUI>();
    }


    private void OnBalanceChange(double amount)
    {
        _txtBalance.text = amount;

    }
}
