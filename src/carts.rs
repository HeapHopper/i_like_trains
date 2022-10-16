
/*
    enum Carts representing different types of carts in a trains
*/

enum Carts {
    Locomotive  {name: String, next_cart: Optional<Carts>},
    Passenger   {name: String, num_of_passenger: usize, next_cart: Optional<Carts>},
    Freight     {name: String, num_of_cases: usize, next_cart: Optional<Carts>},
    Staff       {name: String, num_of_stuff: usize, next_cart: Optional<Carts>}
}

impl Carts {

    fn get_base_weight(cart: Carts) -> usize {
        match cart {
            Locomotive => 8000,
            Passenger => 2000,
            Freight => 4000,
            Staff => 1500
        }
    }

    fn get_total_weight(cart: Carts) -> usize {
        
        let total = get_base_weight(cart);

        match cart {
            Locomotive => total,
            Passenger => total + (num_of_passenger * 80),
            Freight => total + (num_of_cases * 10),
        }
    }
}