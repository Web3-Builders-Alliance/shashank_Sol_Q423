using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Random=UnityEngine.Random;
public class SpawnObstacles : MonoBehaviour
{
    public GameObject obstacle;
    public GameObject gift;
    public float maxX;
    public float minX;
    public float maxY;
    public float minY;
    public float timeBetweenSpawn;
    private float spawnTime;

    public float timeBetweenSpawnGift;
    private float spawnTimeGift;


    // Update is called once per frame
    void Update()
    {
        if(Time.time > spawnTime)
        {
            Spawn();
            spawnTime = Time.time +timeBetweenSpawn;
        }
        if(Time.time > spawnTimeGift)
        {
            SpawnGift();
            spawnTimeGift = Time.time +timeBetweenSpawnGift;
        }
        
    }

    void Spawn()
    {
        float randomX= Random.Range(minX,maxX);
        float randomY= Random.Range(minY,maxY);
        Instantiate(obstacle,transform.position+new Vector3(randomX,randomY,0),transform.rotation);

    }

    void SpawnGift()
    {
        float randomX= Random.Range(minX,maxX);
        float randomY= Random.Range(minY,maxY);
        Instantiate(gift,transform.position+new Vector3(randomX,randomY,0),transform.rotation);
    }
}
