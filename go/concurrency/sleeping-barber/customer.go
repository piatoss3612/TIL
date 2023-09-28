package main

type Customer string

func NewCustomer(name string) *Customer {
	customer := Customer(name)
	return &customer
}

func (c *Customer) String() string {
	return string(*c)
}

func (c *Customer) EnterBarberShop(shop *BarberShop) error {
	return shop.AddCustomer(c)
}
