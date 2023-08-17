use chrono::NaiveDate;
use poeledger_economy_data::{Confidence, ItemLinks, League, PriceRecord};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NinjaCurrencyRecord {
    #[serde(alias = "League")]
    pub league: League,
    #[serde(alias = "Date")]
    pub date: NaiveDate,
    #[serde(alias = "Get")]
    pub get: String,
    #[serde(alias = "Pay")]
    pub pay: String,
    #[serde(alias = "Value")]
    pub value: f32,
    #[serde(alias = "Confidence")]
    pub confidence: Confidence,
}

impl NinjaCurrencyRecord {
    pub fn to_price_record(self) -> PriceRecord {
        PriceRecord {
            league: self.league,
            confidence: self.confidence,
            date: self.date,
            value: self.value,
            name: self.get,
            item_id: None,
            item_type: None,
            base_type: None,
            item_variant: None,
            item_links: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NinjaItemRecord {
    #[serde(alias = "League")]
    pub league: League,
    #[serde(alias = "Date")]
    pub date: NaiveDate,
    #[serde(alias = "Id")]
    pub id: i32,
    #[serde(alias = "Type")]
    pub r#type: String,
    #[serde(alias = "Name")]
    pub name: String,
    #[serde(alias = "BaseType")]
    pub base_type: Option<String>,
    #[serde(alias = "Variant")]
    pub variant: Option<String>,
    #[serde(alias = "Links")]
    pub links: Option<ItemLinks>,
    #[serde(alias = "Value")]
    pub value: f32,
    #[serde(alias = "Confidence")]
    pub confidence: Confidence,
}

impl NinjaItemRecord {
    pub fn to_price_record(self) -> PriceRecord {
        PriceRecord {
            league: self.league,
            confidence: self.confidence,
            date: self.date,
            value: self.value,
            name: self.name,
            item_id: Some(self.id),
            item_type: Some(self.r#type),
            base_type: self.base_type,
            item_variant: self.variant,
            item_links: self.links,
        }
    }
}

#[test]
fn deserialize_ninja_currency_record() -> anyhow::Result<()> {
    let data = "\
League;Date;Get;Pay;Value;Confidence
Sanctum;2022-12-09;Sacrifice at Dusk;Chaos Orb;1;High
Sanctum;2022-12-10;Sacrifice at Dusk;Chaos Orb;0.9787;High
Sanctum;2022-12-11;Sacrifice at Dusk;Chaos Orb;0.4618;High
Sanctum;2022-12-12;Sacrifice at Dusk;Chaos Orb;0.27212;High
Sanctum;2022-12-13;Sacrifice at Dusk;Chaos Orb;0.182;High
";

    let mut csv_reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(data.as_bytes());

    let mut raw_record = csv::StringRecord::new();
    let headers = csv_reader.headers()?.clone();

    let mut count = 0;
    while csv_reader.read_record(&mut raw_record)? {
        let _: NinjaCurrencyRecord = raw_record
            .deserialize(Some(&headers))
            .expect("should deserialize to CurrencyPriceRecord");

        count += 1;
    }

    let expected_deser_count = 5;
    assert_eq!(count, expected_deser_count);

    Ok(())
}

#[test]
fn deserialize_ninja_item_record() -> anyhow::Result<()> {
    let data = "\
League;Date;Id;Type;Name;BaseType;Variant;Links;Value;Confidence
Sanctum;2022-12-09;2673;SkillGem;Dark Pact;;1/20;;1;Low
Sanctum;2022-12-10;2673;SkillGem;Dark Pact;;1/20;;1.42;Medium
Sanctum;2023-03-19;60081;HelmetEnchant;16% increased Reap Area of Effect;;16;;9;High
Sanctum;2023-03-20;60081;HelmetEnchant;16% increased Reap Area of Effect;;16;;9;High
Sanctum;2023-01-26;44550;UniqueFlask;Replica Rumi's Concoction;Granite Flask;;;9.89;High
Sanctum;2023-01-29;44550;UniqueFlask;Replica Rumi's Concoction;Granite Flask;;;9.92;High
Sanctum;2023-03-13;63845;ClusterJewel;Exerted Attacks deal 20% increased Damage;Medium Cluster Jewel;6 passives;;10;High
Sanctum;2023-03-15;63845;ClusterJewel;Exerted Attacks deal 20% increased Damage;Medium Cluster Jewel;6 passives;;10;High
Sanctum;2022-12-09;1441;UniqueArmour;The Brass Dome;Gladiator Plate;;1-4 links;168.35;Low
Sanctum;2022-12-11;1441;UniqueArmour;The Brass Dome;Gladiator Plate;;1-4 links;76.24;High
";

    let mut csv_reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(data.as_bytes());

    let mut raw_record = csv::StringRecord::new();
    let headers = csv_reader.headers()?.clone();

    let mut count = 0;
    while csv_reader.read_record(&mut raw_record)? {
        let _: NinjaItemRecord = raw_record
            .deserialize(Some(&headers))
            .expect("should deserialize to NinjaItemRecord");

        count += 1;
    }

    let expected_deser_count = 10;
    assert_eq!(count, expected_deser_count);

    Ok(())
}
