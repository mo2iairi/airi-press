use sqlx::PgPool;

use crate::error::{AppError, AppResult};
use crate::models::category::{Category, CategoryWithChildren, CreateCategoryRequest, UpdateCategoryRequest};

pub async fn create_category(pool: &PgPool, req: CreateCategoryRequest) -> AppResult<Category> {
    let category = sqlx::query_as::<_, Category>(
        r#"
        INSERT INTO categories (name, parent_id)
        VALUES ($1, $2)
        RETURNING *
        "#
    )
    .bind(&req.name)
    .bind(req.parent_id)
    .fetch_one(pool)
    .await?;

    Ok(category)
}

pub async fn get_category_by_id(pool: &PgPool, id: i32) -> AppResult<Category> {
    sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Category not found".to_string()))
}

pub async fn get_all_categories(pool: &PgPool) -> AppResult<Vec<Category>> {
    let categories = sqlx::query_as::<_, Category>(
        "SELECT * FROM categories ORDER BY name"
    )
    .fetch_all(pool)
    .await?;

    Ok(categories)
}

pub async fn get_categories_tree(pool: &PgPool) -> AppResult<Vec<CategoryWithChildren>> {
    let categories = get_all_categories(pool).await?;
    Ok(build_category_tree(&categories, None))
}

fn build_category_tree(categories: &[Category], parent_id: Option<i32>) -> Vec<CategoryWithChildren> {
    categories
        .iter()
        .filter(|c| c.parent_id == parent_id)
        .map(|c| CategoryWithChildren {
            id: c.id,
            name: c.name.clone(),
            parent_id: c.parent_id,
            children: build_category_tree(categories, Some(c.id)),
        })
        .collect()
}

pub async fn update_category(pool: &PgPool, id: i32, req: UpdateCategoryRequest) -> AppResult<Category> {
    let existing = get_category_by_id(pool, id).await?;

    let name = req.name.unwrap_or(existing.name);
    let parent_id = req.parent_id.or(existing.parent_id);

    let category = sqlx::query_as::<_, Category>(
        r#"
        UPDATE categories
        SET name = $1, parent_id = $2
        WHERE id = $3
        RETURNING *
        "#
    )
    .bind(&name)
    .bind(parent_id)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(category)
}

pub async fn delete_category(pool: &PgPool, id: i32) -> AppResult<()> {
    let result = sqlx::query("DELETE FROM categories WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Category not found".to_string()));
    }

    Ok(())
}
