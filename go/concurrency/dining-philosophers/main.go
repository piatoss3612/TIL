package main

import (
	"fmt"
	"sync"
	"time"
)

const hunger = 3

var wg sync.WaitGroup

type Fork struct{ sync.Mutex }

type Philosopher struct {
	name                string
	leftFork, rightFork *Fork
}

func (p *Philosopher) eat() {
	defer wg.Done()

	fmt.Printf("%s가 자리에 앉음\n", p.name)
	time.Sleep(time.Second)

	for i := hunger; i > 0; i-- {
		p.leftFork.Lock()
		fmt.Printf("%s(이)가 왼쪽 포크를 들었음\n", p.name)
		p.rightFork.Lock()
		fmt.Printf("%s(이)가 오른쪽 포크를 들었음\n", p.name)

		fmt.Printf("%s(이)가 식사 중\n", p.name)
		time.Sleep(time.Second)

		fmt.Printf("%s(이)가 생각 중\n", p.name)
		time.Sleep(time.Second)

		p.rightFork.Unlock()
		fmt.Printf("%s(이)가 오른쪽 포크를 내려놓음\n", p.name)
		p.leftFork.Unlock()
		fmt.Printf("%s(이)가 왼쪽 포크를 내려놓음\n", p.name)

		time.Sleep(time.Second)
	}

	fmt.Println(p.name, "식사 완료!")
	time.Sleep(time.Second)

	fmt.Printf("%s(이)가 자리에서 일어남\n", p.name)
}

func main() {
	start := time.Now()
	fmt.Println("식사하는 철학자들 문제")
	fmt.Println("==================================================")

	names := []string{"플라톤", "아리스토텔레스", "칸트", "헤겔", "라이프니츠"}

	count := len(names)

	forks := make([]*Fork, count)

	for i := 0; i < count; i++ {
		forks[i] = new(Fork)
	}

	wg.Add(len(names))

	// 실행 시간: 33.014927846s
	// for i, name := range names {
	// 	philosopher := Philosopher{name, forks[i], forks[(i+1)%count]}

	// 	go philosopher.eat()
	// }

	// 실행 시간: 23.011723874s
	for i := 0; i < count-1; i++ {
		philosopher := Philosopher{names[i], forks[i], forks[i+1]}

		go philosopher.eat()
	}

	philosopher := Philosopher{names[count-1], forks[0], forks[count-1]} // 마지막 철학자는 첫번째 포크를 먼저 집어듬
	go philosopher.eat()

	wg.Wait()

	fmt.Println("테이블 위의 모든 철학자들이 식사를 마쳤습니다.")
	fmt.Println("==================================================")
	fmt.Println("실행 시간:", time.Since(start))
}
