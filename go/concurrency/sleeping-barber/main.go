package main

import (
	"errors"
	"fmt"
	"math/rand"
	"sync"
	"time"
)

func main() {
	fmt.Println("잠자는 이발사 문제")
	fmt.Println("==================================================")

	shop := NewBarberShop(10, time.Duration(time.Second)*10)

	wg := sync.WaitGroup{}

	shop.OpenShop()

	NewBarber("철수", 1000*time.Millisecond, &wg).GoToWork(shop)
	NewBarber("영희", 2000*time.Millisecond, &wg).GoToWork(shop)
	NewBarber("영수", 3000*time.Millisecond, &wg).GoToWork(shop)
	NewBarber("민수", 2000*time.Millisecond, &wg).GoToWork(shop)
	NewBarber("민희", 3000*time.Millisecond, &wg).GoToWork(shop)
	NewBarber("국봉", 1000*time.Millisecond, &wg).GoToWork(shop)

	go func() {
		customerId := 1

		for {
			randMillisecond := rand.Int() % 200

			<-time.After(time.Duration(randMillisecond) * time.Millisecond)

			c := NewCustomer(fmt.Sprintf("고객%d", customerId))

			fmt.Printf("%s(이)가 바버샵에 들어갑니다.\n", c)

			err := c.EnterBarberShop(shop)
			if err != nil {
				if errors.Is(err, ErrBarberShopClosed) {
					fmt.Printf("%s(이)가 바버샵에 들어가지 못했습니다. 바버샵이 문을 닫았습니다.\n", c)
					return
				} else if errors.Is(err, ErrorCustomerFull) {
					fmt.Printf("바버샵이 꽉 찼습니다. %s(은)는 집으로 돌아갑니다.\n", c)
				} else {
					fmt.Printf("알 수 없는 오류가 발생했습니다. %s(은)는 집으로 돌아갑니다.\n", c)
				}
			}

			fmt.Printf("%s(이)가 머리를 자르고 집으로 돌아갑니다.\n", c)

			customerId++
		}
	}()

	wg.Wait()

	fmt.Println("바버샵이 문을 닫았습니다. 모든 이발사가 퇴근했습니다. 다음에 또 오세요.")
	fmt.Println("==================================================")
}
