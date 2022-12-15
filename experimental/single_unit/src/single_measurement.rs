//! Represents a single measurement units such as meter, millimeter or meter per second.
//! 


struct SingleMeasurementUnit {

}

impl SingleMeasurementUnit {
    fn try_new_for_identifier<p>(data_provider: &P, identifier: &str) -> Self
    where
        P: DataProvider<UnitsIdsData>,
    {
        unimplemented!();
    }
}