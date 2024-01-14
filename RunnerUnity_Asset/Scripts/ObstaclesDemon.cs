using System.Collections;
using System.Collections.Generic;
using Solana.Unity.Soar.Accounts;
using UnityEngine;

public class ObstaclesDemon : MonoBehaviour
{
    private GameObject player;
    // Start is called before the first frame update
    void Start()
    {
        player = GameObject.FindGameObjectWithTag("Player");
    }

    private void OnTriggerEnter2D(Collider2D collision)
    {
        if(collision.tag=="Player"){
            Destroy(player.gameObject);
        }
        if(collision.tag=="Border"){
            Destroy(this.gameObject);
        }

    }
}
