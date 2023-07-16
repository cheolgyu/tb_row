# tb_row

Change struct serialization to tuple. Specifies the order of tuples.

```


#[cfg(test)]
mod tests {
    use serde::ser::SerializeTupleStruct;
    use serde::Serializer;

    use tb_row::TbRow;
    #[derive(TbRow, Default, Debug)]
    struct FrenchToast {
        #[order(2)]
        pub id: usize,
        #[order(0)]
        pub short_name: String,
        #[order(1)]
        pub price: i64,
    }

    #[test]
    fn set_value_to_tuple() {
        use serde_json::json;

        let mut i = FrenchToast::default();

        i.price = 123;
        i.short_name = "f_toast".to_string();
        i.id = 321;

        let j = json!(i);
        println!("result ={:#?}", j);

        // result =Array [
        //     String("f_toast"),
        //     Number(123),
        //     Number(321),
        // ]

        let result = 4;
        assert_eq!(result, 4);
    }

}

```
