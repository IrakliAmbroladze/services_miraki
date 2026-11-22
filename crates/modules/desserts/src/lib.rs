use api_contract::{DessertDto, DessertsResponse};
use axum::Json;

pub async fn list_desserts() -> Json<DessertsResponse> {
    Json(DessertsResponse { items: desserts() })
}

fn desserts() -> Vec<DessertDto> {
    vec![
        DessertDto {
            id: uuid::Uuid::parse_str("9a0c587a-6498-4bf9-a976-95182a99fd83").unwrap(),
            title: String::from("ტორტები"),
            description: String::from("კლასიკური და საფირმო"),
            image_key: String::from("dessert_1"),
        },
        DessertDto {
            id: uuid::Uuid::parse_str("3b7e1c42-8d5a-4f21-b609-1e4c7a92d8f0").unwrap(),
            title: String::from("ტკბილეული"),
            description: String::from("მაფინები და მინი დესერტები"),
            image_key: String::from("dessert_2"),
        },
        DessertDto {
            id: uuid::Uuid::parse_str("7f2a9d18-3c6b-4e85-9a14-6d8b3f51c2e7").unwrap(),
            title: String::from("ცომეული"),
            description: String::from("კრუასანი და ფუნთუშები"),
            image_key: String::from("dessert_3"),
        },
        DessertDto {
            id: uuid::Uuid::parse_str("c49e6b21-5f7a-41d8-8e32-2b9f4c67a1d5").unwrap(),
            title: String::from("დესერტი"),
            description: String::from("ნამცხვრები და ტკბილეული"),
            image_key: String::from("dessert_4"),
        },
        DessertDto {
            id: uuid::Uuid::parse_str("1d8f5a73-9e2c-4b64-a751-4f3e8b19d6c0").unwrap(),
            title: String::from("მაკარუნები"),
            description: String::from("ფერადი ფრანგული ტკბილეული"),
            image_key: String::from("dessert_5"),
        },
        DessertDto {
            id: uuid::Uuid::parse_str("6e3c8d94-1b5f-4a72-9c83-7a2d5e64b9f1").unwrap(),
            title: String::from("ეკლერები"),
            description: String::from("მოხარშული კრემის დესერტი"),
            image_key: String::from("dessert_6"),
        },
        DessertDto {
            id: uuid::Uuid::parse_str("5a1b7e39-4d8c-4f16-b248-8c6e1a93f7d2").unwrap(),
            title: String::from("ფახლავა"),
            description: String::from("ტრადიციული თხილეულით"),
            image_key: String::from("dessert_7"),
        },
        DessertDto {
            id: uuid::Uuid::parse_str("8c4d2f65-7a9e-43b1-a592-3e7b6d18c4f9").unwrap(),
            title: String::from("ჩურჩხელა"),
            description: String::from("ქართული ტრადიციული ტკბილეული"),
            image_key: String::from("dessert_8"),
        },
        DessertDto {
            id: uuid::Uuid::parse_str("2f9a4e17-6c3d-48b9-8d76-5b1f9c42e8a3").unwrap(),
            title: String::from("ფენოვანი"),
            description: String::from("ხრაშუნა კარაქის ფენებით"),
            image_key: String::from("dessert_9"),
        },
        DessertDto {
            id: uuid::Uuid::parse_str("4b6e8c23-2f1a-49d7-9e65-1d5a7f83b2c4").unwrap(),
            title: String::from("ხილის ტარტები"),
            description: String::from("სეზონური და ახალი ხილით"),
            image_key: String::from("dessert_10"),
        },
        DessertDto {
            id: uuid::Uuid::parse_str("9e1c7b54-8a3f-42d6-b419-6f2c8e51a7d8").unwrap(),
            title: String::from("შოკოლადები"),
            description: String::from("ხელნაკეთი შოკოლადის ნაკრები"),
            image_key: String::from("dessert_11"),
        },
        DessertDto {
            id: uuid::Uuid::parse_str("3d5a9f86-1e7c-46b2-a834-9c4b2e76f1a5").unwrap(),
            title: String::from("ჯელატო / ნაყინი"),
            description: String::from("ცივი და გამაგრილებებელი"),
            image_key: String::from("dessert_12"),
        },
    ]
}
