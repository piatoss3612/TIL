package main

import (
	"errors"
	"fmt"
	"sync"
	"time"
)

type BarberState int8

const (
	Checking BarberState = iota
	Cutting
	Sleeping
)

type Barber struct {
	Name            string
	CuttingDuration time.Duration
	State           BarberState
	doneChan        chan bool
	customerChan    <-chan *Customer

	wg *sync.WaitGroup
	mu sync.Mutex
}

func NewBarber(name string, cuttingDuration time.Duration, wg *sync.WaitGroup) *Barber {
	return &Barber{
		Name:            name,
		CuttingDuration: cuttingDuration,
		State:           Checking,
		doneChan:        make(chan bool, 1),
		customerChan:    nil,
		wg:              wg,
		mu:              sync.Mutex{},
	}
}

func (b *Barber) GoToWork(shop *BarberShop) {
	b.mu.Lock()
	defer b.mu.Unlock()

	fmt.Printf("%s(은)는 출근합니다.\n", b.Name)

	if err := shop.AddBarber(b); err != nil {
		if errors.Is(err, ErrBarberShopClosed) {
			fmt.Printf("%s(은)는 출근하지 못했습니다. 바버샵이 문을 닫았습니다.\n", b.Name)
		}
		return
	}

	b.wg.Add(1)

	go func() {
		defer func() {
			close(b.doneChan)
		}()

		for {
			select {
			case <-b.doneChan:
				b.doneForToday()
				return
			case customer, ok := <-b.customerChan:
				if !ok || customer == nil {
					b.doneForToday()
					return
				}

				b.mu.Lock()
				if b.State == Sleeping {
					fmt.Printf("%s(은)는 %s(을)를 깨웁니다.\n", customer, b.Name)
					b.State = Checking
				}
				b.mu.Unlock()

				b.CutHair(customer)
			default:
				b.mu.Lock()
				if b.State == Checking {
					fmt.Printf("%s(은)는 할 일이 없어 잠을 잡니다.\n", b.Name)
					b.State = Sleeping
				}
				b.mu.Unlock()
			}
		}
	}()
}

func (b *Barber) CutHair(customer *Customer) {
	fmt.Printf("%s(은)는 %s의 머리를 깍습니다.\n", b.Name, customer)

	b.mu.Lock()
	b.State = Cutting
	b.mu.Unlock()

	time.Sleep(b.CuttingDuration)

	fmt.Printf("%s(은)는 %s의 머리를 다 깍았습니다.\n", b.Name, customer)

	b.mu.Lock()
	b.State = Checking
	b.mu.Unlock()
}

func (b *Barber) doneForToday() {
	defer b.wg.Done()
	fmt.Printf("%s(은)는 오늘 하루 일을 마치고 집으로 돌아갑니다.\n", b.Name)
}
