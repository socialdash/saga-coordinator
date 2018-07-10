use stq_router::RouteParser;

#[derive(Clone, Debug, PartialEq)]
pub enum Route {
    CreateAccount,
    CreateStore,
    CreateOrder,
    SetOrdersPaid,
}

pub fn create_route_parser() -> RouteParser<Route> {
    let mut router = RouteParser::default();

    router.add_route(r"^/create_account$", || Route::CreateAccount);

    router.add_route(r"^/create_store$", || Route::CreateStore);

    router.add_route(r"^/create_order$", || Route::CreateOrder);

    router.add_route(r"^/orders/set_paid$", || Route::SetOrdersPaid);

    router
}
