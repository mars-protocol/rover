use osmosis_std::types::osmosis::twap::v1beta1::{
    ArithmeticTwapToNowRequest, ArithmeticTwapToNowResponse,
};

use osmosis_testing::{fn_execute, fn_query};
use osmosis_testing::{Module, Runner};

pub struct Twap<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Twap<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Twap<'a, R>
where
    R: Runner<'a>,
{
    fn_query! {
        pub query_twap_price ["/osmosis.twap.v1beta1.Query/ArithmeticTwapToNow"]: ArithmeticTwapToNowRequest => ArithmeticTwapToNowResponse
    }
}
