use crate::data_provider::DataProvider;
use crate::data_provider::Request;
use crate::data_provider::Response;
use crate::error::Error;
use crate::structs::get_type_id;

/// A data provider that validates the type IDs returned by another data provider.
pub struct DataProviderValidator<'b, 'd> {
    pub data_provider: &'b dyn DataProvider<'d>,
}

impl<'b, 'd> DataProvider<'d> for DataProviderValidator<'b, 'd> {
    fn load(&self, req: &Request) -> Result<Response<'d>, Error> {
        match self.data_provider.load(req) {
            Ok(res) => {
                let actual_type_id = res.get_payload_type_id();
                match get_type_id(&req.data_key) {
                    Some(data_key_type_id) => {
                        if data_key_type_id == actual_type_id {
                            Ok(res)
                        } else {
                            Err(Error::MismatchedType {
                                actual: actual_type_id,
                                data_key: Some(data_key_type_id),
                                generic: None,
                            })
                        }
                    }
                    None => Err(Error::MismatchedType {
                        actual: actual_type_id,
                        data_key: None,
                        generic: None,
                    }),
                }
            }
            Err(err) => Err(err),
        }
    }
}
