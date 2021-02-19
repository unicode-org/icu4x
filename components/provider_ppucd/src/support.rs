// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::parse_ppucd;
use icu_provider::prelude::*;
use icu_uniset::provider::*;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::collections::HashMap;
use tinystr::TinyStr16;
use tinystr::tinystr16;

#[derive(Debug, PartialEq, Clone)]
pub struct UnicodeProperties<'s> {
    pub props: Vec<UnicodeProperty<'s>>,
}

#[derive(Debug)]
pub struct PpucdDataProvider<'s> {
    pub ppucd_props: UnicodeProperties<'s>,
}

impl<'s> PpucdDataProvider<'s> {
    pub fn new(prop_str: &'s str) -> Self {
        let data: UnicodeProperties = parse_ppucd::parse(prop_str);
        PpucdDataProvider { ppucd_props: data }
    }

    pub fn from_prop(ppucd_prop: UnicodeProperty<'s>) -> Self {
        PpucdDataProvider {
            ppucd_props: UnicodeProperties {
                props: vec![ppucd_prop],
            },
        }
    }
}

impl<'d, 's> DataProvider<'d, UnicodeProperty<'s>> for PpucdDataProvider<'s> {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, UnicodeProperty<'s>>, DataError> {
        let resc_key: &ResourceKey = &req.resource_path.key;
        let resc_key_str: &str = resc_key.sub_category.as_str();
        let props_data: &UnicodeProperties = &self.ppucd_props;
        let matching_prop: Option<&UnicodeProperty> =
            props_data.props.iter().find(|p| p.name == resc_key_str);
        let owned_matching_prop: Option<UnicodeProperty> = matching_prop.cloned();
        let prop = match owned_matching_prop {
            Some(p) => p,
            None => return Err(req.clone().into()),
        };
        Ok(DataResponse {
            metadata: DataResponseMetadata { data_langid: None },
            payload: Some(Cow::Owned(prop)),
        })
    }
}

//
// Enumerated properties - mapping subcategory string to `UnicodeProperty.name` field
//

fn get_enum_prop_name_mapping() -> HashMap<&'static str, TinyStr16> {
    let mut result: HashMap<&'static str, TinyStr16> = HashMap::new();
    result.insert("InPC=Bottom_And_Left", tinystr16!("InPC=BottomLeft"));
    result.insert("InPC=Bottom_And_Right", tinystr16!("InPC=BottomRight"));
    result.insert("InPC=Top_And_Bottom", tinystr16!("InPC=TopBottom"));
    result.insert("InPC=Top_And_Bottom_And_Left", tinystr16!("InPC=TopBotLeft"));
    result.insert("InPC=Top_And_Bottom_And_Right", tinystr16!("InPC=TopBotRight"));
    result.insert("InPC=Top_And_Left", tinystr16!("InPC=TopLeft"));
    result.insert("InPC=Top_And_Left_And_Right", tinystr16!("InPC=TopLR"));
    result.insert("InPC=Top_And_Right", tinystr16!("InPC=TopRight"));
    result.insert("InPC=Visual_Order_Left", tinystr16!("InPC=VisOrdLeft"));
    result.insert("InSC=Brahmi_Joining_Number", tinystr16!("InSC=BJoinNum"));
    result.insert("InSC=Cantillation_Mark", tinystr16!("InSC=CantillMark"));
    result.insert("InSC=Consonant_Dead", tinystr16!("InSC=ConsDead"));
    result.insert("InSC=Consonant_Final", tinystr16!("InSC=ConsFinal"));
    result.insert("InSC=Consonant_Head_Letter", tinystr16!("InSC=ConsHeadLet"));
    result.insert("InSC=Consonant_Initial_Postfixed", tinystr16!("InSC=CInitPost"));
    result.insert("InSC=Consonant_Killer", tinystr16!("InSC=ConsKiller"));
    result.insert("InSC=Consonant_Medial", tinystr16!("InSC=ConsMedial"));
    result.insert("InSC=Consonant_Placeholder", tinystr16!("InSC=ConsPholder"));
    result.insert("InSC=Consonant_Preceding_Repha", tinystr16!("InSC=ConsPreReph"));
    result.insert("InSC=Consonant_Prefixed", tinystr16!("InSC=ConsPre"));
    result.insert("InSC=Consonant_Subjoined", tinystr16!("InSC=ConsSubjoin"));
    result.insert("InSC=Consonant_Succeeding_Repha", tinystr16!("InSC=CSuccRepha"));
    result.insert("InSC=Consonant_With_Stacker", tinystr16!("InSC=ConsStacker"));
    result.insert("InSC=Gemination_Mark", tinystr16!("InSC=GeminMark"));
    result.insert("InSC=Invisible_Stacker", tinystr16!("InSC=InvisStack"));
    result.insert("InSC=Modifying_Letter", tinystr16!("InSC=ModLetter"));
    result.insert("InSC=Number_Joiner", tinystr16!("InSC=NumJoiner"));
    result.insert("InSC=Register_Shifter", tinystr16!("InSC=RegistShift"));
    result.insert("InSC=Syllable_Modifier", tinystr16!("InSC=SyllMod"));
    result.insert("InSC=Vowel_Dependent", tinystr16!("InSC=VowelDep"));
    result.insert("InSC=Vowel_Independent", tinystr16!("InSC=VowelIndep"));
    result.insert("jg=Burushaski_Yeh_Barree", tinystr16!("jg=BurushYehBarr"));
    result.insert("jg=Hanifi_Rohingya_Kinna_Ya", tinystr16!("jg=HRohingKinYa"));
    result.insert("jg=Hanifi_Rohingya_Pa", tinystr16!("jg=HRohingPa"));
    result.insert("jg=Malayalam_Nnna", tinystr16!("jg=Ma_Nnna"));
    result.insert("jg=Malayalam_Llla", tinystr16!("jg=Ma_Llla"));
    result.insert("jg=Manichaean_Aleph", tinystr16!("jg=ManichAleph"));
    result.insert("jg=Manichaean_Ayin", tinystr16!("jg=ManichAyin"));
    result.insert("jg=Manichaean_Beth", tinystr16!("jg=ManichBeth"));
    result.insert("jg=Manichaean_Daleth", tinystr16!("jg=ManichDaleth"));
    result.insert("jg=Manichaean_Dhamedh", tinystr16!("jg=ManichDhamedh"));
    result.insert("jg=Manichaean_Five", tinystr16!("jg=Manich_Five"));
    result.insert("jg=Manichaean_Gimel", tinystr16!("jg=Manich_Gimel"));
    result.insert("jg=Manichaean_Heth", tinystr16!("jg=Manich_Heth"));
    result.insert("jg=Manichaean_Hundred", tinystr16!("jg=ManichHundred"));
    result.insert("jg=Manichaean_Kaph", tinystr16!("jg=Manich_Kaph"));
    result.insert("jg=Manichaean_Lamedh", tinystr16!("jg=Manich_Lamedh"));
    result.insert("jg=Manichaean_Mem", tinystr16!("jg=Manich_Mem"));
    result.insert("jg=Manichaean_Nun", tinystr16!("jg=Manich_Nun"));
    result.insert("jg=Manichaean_One", tinystr16!("jg=Manich_One"));
    result.insert("jg=Manichaean_Pe", tinystr16!("jg=Manich_Pe"));
    result.insert("jg=Manichaean_Qoph", tinystr16!("jg=Manich_Qoph"));
    result.insert("jg=Manichaean_Resh", tinystr16!("jg=Manich_Resh"));
    result.insert("jg=Manichaean_Sadhe", tinystr16!("jg=Manich_Sadhe"));
    result.insert("jg=Manichaean_Samekh", tinystr16!("jg=Manich_Samekh"));
    result.insert("jg=Manichaean_Taw", tinystr16!("jg=Manich_Taw"));
    result.insert("jg=Manichaean_Ten", tinystr16!("jg=Manich_Ten"));
    result.insert("jg=Manichaean_Teth", tinystr16!("jg=Manich_Teth"));
    result.insert("jg=Manichaean_Thamedh", tinystr16!("jg=ManichThamedh"));
    result.insert("jg=Manichaean_Twenty", tinystr16!("jg=Manich_Twenty"));
    result.insert("jg=Manichaean_Waw", tinystr16!("jg=Manich_Waw"));
    result.insert("jg=Manichaean_Yodh", tinystr16!("jg=Manich_Yodh"));
    result.insert("jg=Manichaean_Zayin", tinystr16!("jg=Manich_Zayin"));
    result.insert("jg=No_Joining_Group", tinystr16!("jg=NoJoinGroup"));
    result.insert("jg=Teh_Marbuta_Goal", tinystr16!("jg=TehMarbGoal"));
    result
}

static ENUM_PROP_NAME_MAPPING: HashMap<&'static str, TinyStr16> = get_enum_prop_name_mapping();

impl<'s> TryFrom<&'s str> for PpucdDataProvider<'s> {
    type Error = DataError;
    fn try_from(s: &'s str) -> Result<Self, Self::Error> {
        let props_data: UnicodeProperties = parse_ppucd::parse(s);
        Ok(PpucdDataProvider {
            ppucd_props: props_data,
        })
    }
}

#[test]
fn test_ppucd_provider_parse() {
    let ppucd_property_files_root_path = "tests/testdata/ppucd-wspace-test.txt";
    let ppucd_property_file_str = std::fs::read_to_string(ppucd_property_files_root_path).unwrap();
    let ppucd_provider: PpucdDataProvider = PpucdDataProvider::new(&ppucd_property_file_str);
    let data_req = DataRequest {
        resource_path: ResourcePath {
            key: key::WHITE_SPACE_V1,
            options: ResourceOptions {
                variant: None,
                langid: None,
            },
        },
    };
    let mut resp: DataResponse<UnicodeProperty> = ppucd_provider.load_payload(&data_req).unwrap();

    let ppucd_property_cow: Cow<UnicodeProperty> = resp.take_payload().unwrap();
    let exp_prop_uniset: UnicodeProperty = UnicodeProperty {
        name: Cow::Borrowed("WSpace"),
        inv_list: vec![
            9, 14, 32, 33, 133, 134, 160, 161, 5760, 5761, 8192, 8203, 8232, 8234, 8239, 8240,
            8287, 8288, 12288, 12289,
        ],
    };
    assert_eq!(exp_prop_uniset, ppucd_property_cow.into_owned());
}
