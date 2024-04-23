package main

import (
	"fmt"
	"log"
	"os"
	"time"

	"github.com/gopxl/beep"
	"github.com/gopxl/beep/mp3"
	"github.com/gopxl/beep/speaker"
)

func main() {
	file1, err := os.Open("example/black_catcher.mp3")
	if err != nil {
		log.Fatal(err)
	}
	file2, err := os.Open("example/again.mp3")
	if err != nil {
		log.Fatal(err)
	}

	streamer1, format1, err := mp3.Decode(file1)
	if err != nil {
		log.Fatal(err)
	}
	defer streamer1.Close()

	streamer2, _, err := mp3.Decode(file2)
	if err != nil {
		log.Fatal(err)
	}
	defer streamer2.Close()

	speaker.Init(format1.SampleRate, format1.SampleRate.N(time.Second/10))
	// speaker.Init(format2.SampleRate, format2.SampleRate.N(time.Second/10))

	done := make(chan struct{})

	speaker.Play(beep.Seq(streamer1, streamer2, beep.Callback(func() {
		fmt.Println("done playing both")
		done <- struct{}{}
		close(done)
	})))

	<-done
}
