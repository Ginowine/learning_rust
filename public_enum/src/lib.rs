mod back_of_house {
    pub enum Appetiser {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let _order1 = back_of_house::Appetiser::Soup;
    let _order2 = back_of_house::Appetiser::Salad;
}