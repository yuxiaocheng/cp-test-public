package main

import (
	"time"
	"fmt"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

func main(){
	t := time.Tick(time.Second)
	fmt.Printf("%s", primitive.ObjectID{})
	select {
	case <- t:
		fmt.Println("1 second elapsed")
	}
}


