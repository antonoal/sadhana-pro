use actix_web::{web, HttpRequest, HttpResponse};
use common::error::AppError;
use diesel::prelude::*;
use diesel::{sql_query, sql_types::Uuid as DieselUuid, sql_types::*, PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::middleware::auth;
use crate::middleware::state::AppState;

#[derive(Debug, QueryableByName, Serialize)]
pub struct UserPractice {
    #[diesel(sql_type = Text)]
    pub practice: String,
    #[diesel(sql_type = Bool)]
    pub is_active: bool,
}

impl UserPractice {
    pub fn all_user_practices(
        conn: &mut PgConnection,
        user_id: &Uuid,
    ) -> Result<Vec<Self>, AppError> {
        let res = sql_query(
            r#"
        select up.practice, up.is_active
        from user_practices up
        where up.user_id = $1
        order by up.practice
        "#,
        )
        .bind::<DieselUuid, _>(user_id)
        .load(conn)?;

        Ok(res)
    }

    pub fn update_is_active(
        conn: &mut PgConnection,
        user_id: &Uuid,
        practice: &Self,
    ) -> Result<(), AppError> {
        sql_query(
            r#"
        update user_practices
        set is_active = $1
        where practice = $2
        and user_id = $3
        "#,
        )
        .bind::<Bool, _>(&practice.is_active)
        .bind::<Text, _>(&practice.practice)
        .bind::<DieselUuid, _>(user_id)
        .execute(conn)?;

        Ok(())
    }
}

/// Retrieves all user practices
pub async fn get_user_practices(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let mut conn = state.get_conn()?;
    let user_id = auth::get_current_user(&req)?.id;

    let res = UserPractice::all_user_practices(&mut conn, &user_id)?;
    Ok(HttpResponse::Ok().json(AllUserPracticesResponse::from(res)))
}

#[derive(Serialize, Debug)]
pub struct AllUserPracticesResponse {
    pub user_practices: Vec<UserPractice>,
}

impl From<Vec<UserPractice>> for AllUserPracticesResponse {
    fn from(user_practices: Vec<UserPractice>) -> Self {
        Self { user_practices }
    }
}

type PracticeSlug = String;

#[derive(Deserialize, Debug)]
pub struct IsActiveParams {
    is_active: bool,
}

/// Updates is active state on user practice
pub async fn set_is_active(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<PracticeSlug>,
    params: web::Query<IsActiveParams>,
) -> Result<HttpResponse, AppError> {
    let mut conn = state.get_conn()?;
    let practice = UserPractice {
        practice: path.into_inner(),
        is_active: params.is_active,
    };
    let user_id = auth::get_current_user(&req)?.id;

    log::debug!("Updating practice activity to {:?}", practice);

    UserPractice::update_is_active(&mut conn, &user_id, &practice)?;

    Ok(HttpResponse::Ok().json(()))
}