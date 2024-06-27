use std::cell::RefCell;




thread_local!{
    static PRODUCTS_TITLE: RefCell<Vec<String>> = RefCell::default();
    static PRODUCTS_PRICE: RefCell<Vec<String>> = RefCell::default();
    static PRODUCTS_URL: RefCell<Vec<String>> = RefCell::default();

    
}

#[ic_cdk::update]
fn dodaj_product(title: String, price: String, img_path: String) {
    PRODUCTS_PRICE.with(|products| {
        products.borrow_mut().push(price);
    });
    PRODUCTS_TITLE.with(|products| {
        products.borrow_mut().push(title);
    });
    PRODUCTS_URL.with(|products| {
        products.borrow_mut().push(img_path);
    });
}

#[ic_cdk::query]
fn read_title() -> Vec<String>{
    PRODUCTS_TITLE.with(|wpisy| {
        wpisy.borrow().clone()
    })
}
#[ic_cdk::query]
fn read_price() -> Vec<String>{
    PRODUCTS_PRICE.with(|wpisy| {
        wpisy.borrow().clone()
    })
}
