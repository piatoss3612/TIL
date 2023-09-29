package main

import (
	"os"
	"sync"
	"testing"
	"time"
)

func TestSolution1(t *testing.T) {
	cnt := 10
	timeAcc := 0

	oldOut := os.Stdout

	r, w, _ := os.Pipe()

	os.Stdout = w

	wg := sync.WaitGroup{}
	mu := sync.Mutex{}

	for i := 0; i < cnt; i++ {
		wg.Add(1)

		go func(i int) {
			defer wg.Done()
			start := time.Now()
			solution1()
			mu.Lock()
			timeAcc += int(time.Since(start).Milliseconds())
			mu.Unlock()

			t.Logf("solution1 %d done", i)
		}(i)
	}

	wg.Wait()

	_ = w.Close()
	_ = r.Close()

	os.Stdout = oldOut

	avg := timeAcc / cnt

	avgTime := time.Duration(avg) * time.Millisecond

	t.Logf("avg: %v", avgTime)
}

func TestSolution2(t *testing.T) {
	cnt := 10
	timeAcc := 0

	oldOut := os.Stdout

	r, w, _ := os.Pipe()

	os.Stdout = w

	wg := sync.WaitGroup{}
	mu := sync.Mutex{}

	for i := 0; i < cnt; i++ {
		wg.Add(1)

		go func(i int) {
			defer wg.Done()
			start := time.Now()
			solution2()
			mu.Lock()
			timeAcc += int(time.Since(start).Seconds())
			mu.Unlock()

			t.Logf("solution2 %d done", i)
		}(i)
	}

	wg.Wait()

	_ = w.Close()
	_ = r.Close()

	os.Stdout = oldOut

	avg := timeAcc / cnt

	avgTime := time.Duration(avg) * time.Second

	t.Logf("avg: %v", avgTime)
}
