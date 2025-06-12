use actix_web::{get, post, web};
use crate::enums::fixed_data::FixedDataConfig;
use crate::entity::fixed_data::{FixedDataResponse, FixedDataConfigResponse, FixedDataConfigUpdateRequest};
use crate::utils::response::ApiResponse;
use crate::db;

#[get("/fixed-data/{config}")]
pub async fn get_fixed_data(config: web::Path<FixedDataConfig>) -> ApiResponse<FixedDataResponse> {
    let response = FixedDataResponse {
        config: *config,
        name: config.get_name().to_string(),
    };
    ApiResponse::success(response)
}

#[get("/fixed-data/config/{type}")]
pub async fn get_fixed_data_config(r#type: web::Path<FixedDataConfig>) -> ApiResponse<Vec<FixedDataConfigResponse>> {
    let pool = db::get_pool();
    let type_str = format!("{:?}", *r#type);
    
    // 从数据库查询固定数据配置
    let result = sqlx::query!(
        r#"
        SELECT key, value, type as "type"
        FROM T_FIXED_DATA_CONFIG
        WHERE type = $1
        "#,
        type_str
    )
    .fetch_all(pool)
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

#[post("/fixed-data/config/update")]
pub async fn update_fixed_data_config(req: web::Json<FixedDataConfigUpdateRequest>) -> ApiResponse<()> {
    let pool = db::get_pool();
    let type_str = format!("{:?}", req.r#type);
    
    // 开始事务
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return ApiResponse::error(500, format!("Failed to begin transaction: {}", e)),
    };

    // 批量更新数据
    for item in &req.list {
        // 使用 UPSERT 语法（SQLite 3.24.0+ 支持）
        match sqlx::query!(
            r#"
            INSERT INTO T_FIXED_DATA_CONFIG (key, value, type)
            VALUES ($1, $2, $3)
            ON CONFLICT(key) DO UPDATE SET
                value = $2,
                type = $3
            "#,
            item.key,
            item.value,
            type_str
        )
        .execute(&mut *tx)
        .await
        {
            Ok(_) => continue,
            Err(e) => {
                // 回滚事务
                if let Err(rollback_err) = tx.rollback().await {
                    return ApiResponse::error(500, format!("Failed to rollback transaction: {}", rollback_err));
                }
                return ApiResponse::error(500, format!("Failed to update config: {}", e));
            }
        }
    }

    // 提交事务
    match tx.commit().await {
        Ok(_) => ApiResponse::success(()),
        Err(e) => ApiResponse::error(500, format!("Failed to commit transaction: {}", e)),
    }
} 