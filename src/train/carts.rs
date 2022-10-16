
/*
    enum Carts representing different types of carts in a trains
*/

pub enum Cart {
    Locomotive  {name: String, next_cart: Option<*mut Cart>},
    Passenger   {name: String, num_of_passenger: usize, next_cart: Option<*mut Cart>},
    Freight     {name: String, num_of_cases: usize, next_cart: Option<*mut Cart>},
    Staff       {name: String, num_of_staff: usize, next_cart: Option<*mut Cart>}
}

impl Cart {

    fn get_cart_name(cart: &Cart) -> String {
        match cart {
            Locomotive  => String::from("Locomotive"),
            Passenger   => String::from("Passenger"),
            Freight     => String::from("Frieght"),
            Staff       => String::from("Staff")
        }
    }

    fn get_base_cart_weight(cart: &Cart) -> usize {
        match cart {
            Locomotive  => 8000,
            Passenger   => 2000,
            Freight     => 4000,
            Staff       => 1500
        }
    }

    fn get_total_cart_weight(cart: &Cart) -> usize {
        let total = Cart::get_base_cart_weight(&cart);
        match cart {
            Self::Locomotive { .. }  => total,
            Self::Passenger { num_of_passenger, .. }   => total + (num_of_passenger * 80),
            Self::Freight { num_of_cases, ..}     => total + (num_of_cases * 10),
            Self::Staff { num_of_staff, .. }       => total + (num_of_staff * 80)
        }
    }

    fn get_total_cart_weight_v2(cart: &Cart) -> usize {
        let total = Cart::get_base_cart_weight(cart);
        if let Cart::Locomotive { name:_, next_cart:_ } = cart {
            return total;
        }
        else if let Cart::Passenger { num_of_passenger, ..} = cart {
            return total + (num_of_passenger*80);
        }
        else if let Cart::Freight { num_of_cases, .. } = cart {
            return total + (num_of_cases * 10);
        }
        else if let Cart::Staff { num_of_staff, .. } = cart {
            return total + (num_of_staff * 80);
        }
        total
    }
}