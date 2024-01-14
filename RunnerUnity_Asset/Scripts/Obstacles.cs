using System.Collections;
using System.Collections.Generic;
using Solana.Unity.Soar.Accounts;
using TMPro;
using UnityEngine;
using UnityEngine.UI;
public class Obstacles : MonoBehaviour
{
    public TextMeshProUGUI scoreText;
    private float score;
    // Start is called before the first frame update
    void Start()
    {
        
    }

    private void OnTriggerEnter2D(Collider2D collision)
    {
        if(collision.tag=="Player"){
            Destroy(this.gameObject);
            score +=1;
            scoreText.text = ((int)score).ToString()+"";
        }

    }
}
