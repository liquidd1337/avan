use crate::client_http2;
use serde::{Deserialize, Serialize};

type FloatStr = f64;
type Float = f64;
type DataStr = String;

use serde_aux::prelude::*;

#[derive(Deserialize, Serialize, PartialEq, Default, Debug)]
#[serde(default)]
pub struct Root {
    count: u32,
    page_count: u32,
    limit: u32,
    prev_page: Option<u32>,
    page: u32,
    next_page: Option<u32>,
    data: Vec<Item>,
    #[serde(rename = "rateId")]
    rate_id: u32,
}

#[derive(Deserialize, Serialize, PartialEq, Default, Debug)]
#[serde(default)]
pub struct Item {
    full_name: String,
    variants: Vec<Variant>,
    sell_items: Vec<SellItems>,
}

#[derive(Deserialize, Serialize, PartialEq, Default, Debug)]
#[serde(default)]
pub struct Variant {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    sell_price: FloatStr,
}
#[derive(Deserialize, Serialize, PartialEq, Default, Debug)]
#[serde(default)]
pub struct SellItems {
    sell_price: Float,
}

pub async fn parse_avan() {
    let url: &str = "https://avan.market/v1/api/users/catalog?app_id=252490&currency=2&page=30";
    let body = client_http2::run_client_http2(url, client_http2::Mode::Uncheck)
        .await
        .unwrap();

    let root: Root = match serde_json::from_str(&body) {
        Ok(c) => c,
        Err(e) => {
            println!("Ошибка чтения JSON : {} ", e);
            let root_0 = Root::default();

            /*
            let mut o_from_file = serde_json::from_str(&body).unwrap();
            let o_def = serde_json::to_value(&root_0).unwrap();

            add_default(&mut o_from_file, &o_def);

            root_0 = serde_json::from_value(o_from_file).unwrap();*/
            root_0
        }
    };

    println!("get items - {}", root.data.len());
    assert_eq!(root.count as usize, root.data.len());
    assert_eq!(root.page_count, root.page);

    for item in root.data {

        let item_url = item.full_name.replace(" ", "%20");

        let url = format!("https://steamcommunity.com/market/listings/{app_id}/{item_url}");
        let body = get_http_body(&url, ModeUTF8Check::Uncheck)
            .await
            .unwrap();
        dbg!(url);
        //	Market_LoadOrderSpread( 176250984 );
        let substr1 = "Market_LoadOrderSpread(";
        let substr2 = ")";
        let Some(beg_pos) = body.find(substr1) else {continue};
        let beg_pos = beg_pos + substr1.len();
        let next_str = &body[beg_pos..];
        let Some(end_pos) = next_str.find(substr2) else {continue};
        let item_id = &next_str[..end_pos];

        let item_id = String::from(item_id);
        let item_id = item_id.trim();
        dbg!(&item_id);

        let url = format!("https://steamcommunity.com/market/itemordershistogram?country=UA&language=russian&currency=1&item_nameid={item_id}");
        let body = get_http_body(&url, ModeUTF8Check::Uncheck)
            .await
            .unwrap();

        dbg!(url);
        let v: Value = serde_json::from_str(&body).unwrap();
        let body = serde_json::to_string_pretty(&v).unwrap();
        let file_name = item.full_name.replace(" ", "_")+".json";
        let mut f = fs::File::create(&file_name).expect(&format!("создаем файл {file_name}"));
        f.write_all(body.as_bytes()).expect(&format!("пишем body в файл {file_name}"));
        println!("записали в файл {file_name}... спим 2 минуты...");

        sleep(Duration::from_millis(2*60*10)).await;
    }



}

/*fn add_default(a: &mut Value, def: &Value) {
    if let (&mut Value::Object(ref mut a), &Value::Object(ref def)) = (a, def) {
        for (k, v) in def {
            add_default(a.entry(k.as_str()).or_insert(v.clone()), v);
        }
    }
}*/