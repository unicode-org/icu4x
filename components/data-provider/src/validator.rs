use crate::data_provider::DataProvider;
use crate::data_provider::Request;
use crate::data_provider::Response;
use crate::error::ResponseError;

/// A data provider that validates the type IDs returned by another data provider.
pub struct DataProviderValidator<'a, 'b, 'd> {
    pub data_provider: &'b dyn DataProvider<'a, 'd>,
}

impl<'a, 'b, 'd> DataProvider<'a, 'd> for DataProviderValidator<'a, 'b, 'd> {
    fn load(&'a self, req: &Request) -> Result<Response<'d>, ResponseError> {
        match self.data_provider.load(req) {
            Ok(res) => {
                let actual_type_id = res.get_payload_type_id();
                match req.data_key.get_type_id() {
                    Some(expected_type_id) => {
                        if expected_type_id == actual_type_id {
                            Ok(res)
                        } else {
                            Err(ResponseError::InvalidTypeError {
                                actual: actual_type_id,
                                expected: Some(expected_type_id),
                            })
                        }
                    }
                    None => Err(ResponseError::InvalidTypeError {
                        actual: actual_type_id,
                        expected: None,
                    }),
                }
            }
            Err(err) => Err(err),
        }
    }
}
