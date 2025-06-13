use actix_web::{get, post, web};
use sqlx::sqlite::SqlitePool;

use crate::enums::fixed_data::FixedDataConfig;
use crate::entity::fixed_data::{FixedDataResponse, FixedDataConfigResponse, FixedDataConfigUpdateRequest};
use crate::utils::response::ApiResponse;

#[get("/fixed-data/{config}")]
pub async fn get_fixed_data(
    config: web::Path<FixedDataConfig>
) -> ApiResponse<FixedDataResponse> {
    let response = FixedDataResponse {
        config: *config,
        name: config.get_name().to_string(),
    };
    ApiResponse::success(response)
}

#[get("/fixed-data/config/{type}")]
pub async fn get_fixed_data_config(
    pool: web::Data<SqlitePool>,
    r#type: web::Path<FixedDataConfig>
) -> ApiResponse<Vec<FixedDataConfigResponse>> {
    let type_str = format!("{:?}", *r#type);

    let result = sqlx::query!(
        r#"
        SELECT key, value, "type"
        FROM T_FIXED_DATA_CONFIG
        WHERE "type" = ?
        "#,
        type_str
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(records) => {
            let configs = records.into_iter()
                .map(|record| FixedDataConfigResponse {
                    key: record.key,
                    value: record.value,
                })
                .collect();
            ApiResponse::success(configs)
        },
        Err(e) => ApiResponse::error(500, format!("Failed to get fixed data config: {}", e)),
    }
}

#[post("/manage/fixed-data/config/update")]
pub async fn update_fixed_data_config(
    pool: web::Data<SqlitePool>,
    req: web::Json<FixedDataConfigUpdateRequest>
) -> ApiResponse<()> {
    let type_str = format!("{:?}", req.r#type);

    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return ApiResponse::error(500, format!("Failed to begin transaction: {}", e)),
    };

    for item in &req.list {
        let result = sqlx::query!(
            r#"
            INSERT INTO T_FIXED_DATA_CONFIG (key, value, "type")
            VALUES (?, ?, ?)
            ON CONFLICT(key) DO UPDATE SET
                value = excluded.value,
                "type" = excluded."type"
            "#,
            item.key,
            item.value,
            type_str
        )
        .execute(&mut *tx)
        .await;

        if let Err(e) = result {
            let _ = tx.rollback().await;
            return ApiResponse::error(500, format!("Failed to update config: {}", e));
        }
    }

    match tx.commit().await {
        Ok(_) => ApiResponse::success(()),
        Err(e) => ApiResponse::error(500, format!("Failed to commit transaction: {}", e)),
    }
}

// 这个就是路由注册函数，必须放在模块里，供 main.rs 调用
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_fixed_data);
    cfg.service(get_fixed_data_config);
    cfg.service(update_fixed_data_config);
}
