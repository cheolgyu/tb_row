pub use tb_row_derive::*;

pub trait TbRow {
    fn th_name() -> Vec<String>;
}




#[cfg(test)]
mod tests {
    use serde::ser::SerializeTupleStruct;
    use serde::Serializer;

    use tb_row::TbRow;
    #[derive(TbRow, Default, Debug)]
    struct FrenchToast {
        #[order(5)]
        pub code_id: usize,
        #[order(1)]
        pub short_name: String,
        #[order(2)]
        pub name: String,
        #[order(0)]
    }

    #[test]
    #[cfg(feature = "site")]
    fn set_value_to_tuple() {
        use serde_json::json;

        let i = FrenchToast::default();
        
        // let res = i.to_tuple_json();
        // println!("result ={:#?}", res);
        // let j = serde_json::to_string(&i).unwrap();
        // println!("result ={:#?}", j);

        let j = json!(i);
        println!("result ={:#?}", j);

        let result = 4;
        assert_eq!(result, 4);
    }


}
