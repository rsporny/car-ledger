use actix_web::{web, HttpResponse, Responder};
use log::info;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Expense {
    id: Uuid,
    amount: f64,
    description: String,
    date: String,
    mileage: i32,
    price: f64,
}

pub async fn create_expense(
    pool: web::Data<PgPool>,
    expense: web::Json<Expense>,
) -> impl Responder {
    let expense = expense.into_inner();
    info!("Creating expense: {:?}", expense);
    let result = sqlx::query!(
        "INSERT INTO expenses (id, amount, description, date, mileage, price) VALUES ($1, $2, $3, $4, $5, $6)",
        expense.id,
        expense.amount,
        expense.description,
        expense.date,
        expense.mileage,
        expense.price
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(expense),
        Err(err) => {
            info!("Failed to create expense: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn get_expense(pool: web::Data<PgPool>, expense_id: web::Path<Uuid>) -> impl Responder {
    let result = sqlx::query_as!(
        Expense,
        "SELECT id, amount, description, date, mileage, price FROM expenses WHERE id = $1",
        expense_id.into_inner()
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(expense) => HttpResponse::Ok().json(expense),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn list_expenses(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as!(
        Expense,
        "SELECT id, amount, description, date, mileage, price FROM expenses"
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(expenses) => HttpResponse::Ok().json(expenses),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn update_expense(
    pool: web::Data<PgPool>,
    expense_id: web::Path<Uuid>,
    expense: web::Json<Expense>,
) -> impl Responder {
    let expense = expense.into_inner();
    let result = sqlx::query!(
        "UPDATE expenses SET amount = $1, description = $2, date = $3, mileage = $4, price = $5 WHERE id = $6",
        expense.amount,
        expense.description,
        expense.date,
        expense.mileage,
        expense.price,
        expense_id.into_inner()
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(expense),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn delete_expense(
    pool: web::Data<PgPool>,
    expense_id: web::Path<Uuid>,
) -> impl Responder {
    let expense_id = expense_id.into_inner();
    let result = sqlx::query!("DELETE FROM expenses WHERE id = $1", expense_id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body(format!("Deleted expense with id: {}", expense_id)),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/expenses")
            .route("/", web::post().to(create_expense))
            .route("/", web::get().to(list_expenses))
            .route("/{id}", web::get().to(get_expense))
            .route("/{id}", web::put().to(update_expense))
            .route("/{id}", web::delete().to(delete_expense)),
    );
}
