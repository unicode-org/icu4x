
///
/// Default internal structure for person name.
///
pub struct DefaultPersonName {
    person_data: BTreeMap<NameField, String>,
    locale: Option<Locale>,
    preferred_order: Option<PreferredOrder>,
}

impl PersonName for DefaultPersonName {
    fn name_locale(&self) -> Option<&Locale> {
        self.locale.as_ref()
    }

    fn preferred_order(&self) -> Option<&PreferredOrder> {
        self.preferred_order.as_ref()
    }

    fn get(&self, field: &NameField) -> Option<&str> {
        self.person_data.get(field).map(String::as_ref)
    }

    fn all_provided_fields(&self) -> Vec<&NameField> {
        self.person_data.keys().collect()
    }
}

/// Default person name functions.
impl DefaultPersonName {
    ///
    /// Returns a new person name structure.
    ///
    pub fn try_new_unstable(
        person_data: BTreeMap<NameField, String>,
        locale: Option<Locale>,
        preferred_order: Option<PreferredOrder>,
    ) -> Result<DefaultPersonName, String> {
        let result = DefaultPersonName {
            person_data,
            locale,
            preferred_order,
        };

        if !(&result as &dyn PersonName).is_valid() {
            return Err(String::from(
                "Trying to build an invalid DefaultPersonName !",
            ));
        }
        Ok(result)
    }
}
