mod carts;
use carts::Cart;

pub struct Train {
    first_cart: Cart,
    num_of_carts: usize,
    train_weight: usize,
}
/*
impl Train {

    pub fn new() -> Self {
        Train {
            first_cart: Cart::Locomotive = {"Locomotive", Option::None},
            num_of_carts: 1,
            train_weight: get_train_weight()
        }
    }

    fn get_train_weight(self) -> usize {
        let res = 0;
        let curr_cart: *mut Cart = &first_cart;
        
        loop{
            res += Cart::get_total_cart_weight(curr_cart);
            match curr_cart.next_cart {
                Optional::None => break,
                Optional::Some => curr_cart = curr_cart.next_cart.unwrap()
            }
        }

        res
    }
}
*/