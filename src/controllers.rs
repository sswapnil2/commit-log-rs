use actix_web::{Responder, HttpResponse, web, get, post, Result};
use crate::AppState;
use crate::dto::{ProduceRequest, ConsumeRequest, ProduceResponse, ConsumeResponse};
use crate::log::Record;
use crate::dto;
use bytes::{Bytes};

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("running")
}


#[post("/append")]
pub async fn handle_produce(r: web::Json<ProduceRequest>, state: web::Data<AppState>) -> Result<HttpResponse> {


    let record = Record::new(Bytes::from(r.record.value.clone()));

    let log = state.log.lock().unwrap();
    println!("Length of log: {}", log.len());

    match log.append(record) {
        Some(offset) => Ok(HttpResponse::Ok().json(ProduceResponse {
            offset
        })),
        None => Ok(HttpResponse::Ok().body("Error in saving log"))
    }


}

#[post("/get")]
pub async fn handle_consume(c: web::Json<ConsumeRequest>, state: web::Data<AppState>) -> Result<HttpResponse> {

    let log = state.log.lock().unwrap();
    println!("Length of log: {}", log.len());

    match log.read(c.offset) {
        Ok(record) => {
            let value = std::str::from_utf8(record.value.as_ref()).unwrap();
            let value = String::from(value);

            Ok(HttpResponse::Ok().json(ConsumeResponse {
                record: dto::Record {
                    value,
                    offset: record.offset
                }
            }))
        }
        Err(e) => Ok(HttpResponse::Ok().body(e))
    }
}
