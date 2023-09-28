package main

import (
	"errors"
	"sync"
	"time"

	"github.com/fatih/color"
)

var (
	ErrBarberShopClosed = errors.New("바버샵이 문을 닫았습니다")
	ErrorCustomerFull   = errors.New("바버샵이 꽉 찼습니다")
)

type BarberShop struct {
	Capacity        int
	OpenDuration    time.Duration
	barbersDoneChan []chan<- bool
	customerChan    chan *Customer
	Open            bool

	mu sync.Mutex
}

func NewBarberShop(capacity int, openDuration time.Duration) *BarberShop {
	return &BarberShop{
		Capacity:        capacity,
		OpenDuration:    openDuration,
		barbersDoneChan: []chan<- bool{},
		customerChan:    make(chan *Customer, capacity),
		Open:            false,
		mu:              sync.Mutex{},
	}
}

func (b *BarberShop) OpenShop() {
	b.mu.Lock()
	defer b.mu.Unlock()
	b.Open = true

	color.Blue("공지: 바버샵이 문을 열었습니다. 영업 시간은 %s입니다.\n", b.OpenDuration)

	go func() {
		timer := time.NewTimer(b.OpenDuration)

		<-timer.C

		b.CloseShop()
	}()
}

func (b *BarberShop) CloseShop() {
	b.mu.Lock()
	defer b.mu.Unlock()
	b.Open = false

	color.Blue("공지: 영업 시간이 종료되었습니다. 대기 중인 고객들을 모두 돌려보냅니다.\n")

	for len(b.customerChan) > 0 {
		<-b.customerChan
	}

	color.Blue("공지: 모든 고객들을 돌려보냈습니다. 이발사들을 퇴근시킵니다.\n")

	for _, doneChan := range b.barbersDoneChan {
		doneChan <- true
	}

	close(b.customerChan)
}

func (b *BarberShop) AddBarber(barber *Barber) error {
	b.mu.Lock()
	defer b.mu.Unlock()

	if !b.Open {
		return ErrBarberShopClosed
	}

	b.barbersDoneChan = append(b.barbersDoneChan, barber.doneChan)
	barber.customerChan = b.customerChan

	return nil
}

func (b *BarberShop) AddCustomer(customer *Customer) error {
	b.mu.Lock()
	defer b.mu.Unlock()

	if !b.Open {
		return ErrBarberShopClosed
	}

	select {
	case b.customerChan <- customer:
		return nil
	default:
		return ErrorCustomerFull
	}
}
