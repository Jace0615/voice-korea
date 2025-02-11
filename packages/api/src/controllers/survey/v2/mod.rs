use std::time::Duration;

use aws_config::BehaviorVersion;
use aws_sdk_s3::{presigning::PresigningConfig, primitives::ByteStream};
use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
};
use excel::SurveyResponseExcel;
use models::response::*;
use models::*;
use rust_xlsxwriter::Workbook;
use sqlx::postgres::PgRow;

use crate::utils::nonce_lab::NonceLabClient;

#[derive(Clone, Debug)]
pub struct SurveyControllerV2 {
    panel_survey_repo: PanelSurveysRepository,
    repo: SurveyV2Repository,
    pool: sqlx::Pool<sqlx::Postgres>,
    nonce_lab: NonceLabClient,
}

impl SurveyControllerV2 {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = SurveyV2::get_repository(pool.clone());
        let panel_survey_repo = PanelSurveys::get_repository(pool.clone());

        Self {
            repo,
            panel_survey_repo,
            pool,
            nonce_lab: NonceLabClient::new(),
        }
    }

    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let ctrl = Self::new(pool);

        Ok(by_axum::axum::Router::new()
            .route("/:id/responses", get(Self::download_excel))
            .with_state(ctrl.clone())
            .route("/:id", post(Self::act_by_id).get(Self::get_survey_v2))
            .with_state(ctrl.clone())
            .route("/", post(Self::act_survey_v2).get(Self::list_survey_v2))
            .with_state(ctrl.clone()))
    }

    pub async fn download_excel(
        State(ctrl): State<SurveyControllerV2>,
        Extension(auth): Extension<Option<Authorization>>,
        Path((org_id, survey_id)): Path<(i64, i64)>,
    ) -> Result<Json<SurveyResponseExcel>> {
        tracing::debug!("act_by_id: {:?} {:?}", org_id, survey_id);
        auth.ok_or(ApiError::Unauthorized)?;

        let survey: SurveyV2 = SurveyV2::query_builder()
            .id_equals(survey_id)
            .query()
            .map(|r: sqlx::postgres::PgRow| r.into())
            .fetch_one(&ctrl.pool)
            .await?;

        if survey.org_id != org_id {
            return Err(ApiError::Unauthorized);
        }

        let len = survey.questions.len();

        // NOTE: fetch all data
        let responses: Vec<SurveyResponse> = SurveyResponse::query_builder()
            .survey_id_equals(survey_id)
            .query()
            .map(|r: sqlx::postgres::PgRow| r.into())
            .fetch_all(&ctrl.pool)
            .await?;

        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();

        for i in 0..responses.len() {
            let panel = survey
                .panels
                .iter()
                .find(|p| p.id == responses[i].panel_id)
                .ok_or(ApiError::SurveyResponseNoMatchedPanelId)?;

            worksheet
                .write_string(0, i as u16 + 1, &panel.name)
                .unwrap();
        }

        for i in 0..len {
            worksheet
                .write_string(i as u32 + 1, 0, &survey.questions[i].title())
                .unwrap();
            for (j, response) in responses.iter().enumerate() {
                worksheet
                    .write_string(
                        i as u32 + 1,
                        j as u16 + 1,
                        response.answers[i].to_answer_string(),
                    )
                    .unwrap();
            }
        }

        let bytes = workbook.save_to_buffer().map_err(|e| {
            tracing::error!("error: {:?}", e);
            ApiError::SurveyResponseExcelWritingError
        })?;

        let conf = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let cli = aws_sdk_s3::Client::new(&conf);
        let c = crate::config::get();
        let bucket_name = c.bucket_name;
        let path = format!("surveys/{}.xlsx", survey_id,);

        cli.put_object()
            .bucket(bucket_name)
            .key(&path)
            .body(ByteStream::from(bytes))
            .content_type("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
            .send()
            .await
            .map_err(|e| {
                tracing::error!("error: {:?}", e);
                ApiError::SurveyResponseExcelUploadError
            })?;

        let presigning_config = PresigningConfig::expires_in(Duration::from_secs(
            c.presigned_url_expiration,
        ))
        .map_err(|e| {
            tracing::error!("error: {:?}", e);
            ApiError::SurveyResponseExcelPresigningError
        })?;
        let url = cli
            .get_object()
            .bucket(bucket_name)
            .key(&path)
            .presigned(presigning_config)
            .await
            .map_err(|e| {
                tracing::error!("error: {:?}", e);
                ApiError::SurveyResponseExcelPresigningError
            })?
            .uri()
            .to_string();
        tracing::debug!("excel url: {:?}", url);

        Ok(Json(SurveyResponseExcel { url }))
    }

    pub async fn act_by_id(
        State(ctrl): State<SurveyControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path((org_id, id)): Path<(i64, i64)>,
        Json(body): Json<SurveyV2ByIdAction>,
    ) -> Result<Json<SurveyV2>> {
        tracing::debug!("act_by_id: {:?} {:?}", id, body);

        match body {
            SurveyV2ByIdAction::Update(params) => ctrl.update(org_id, id, params).await,
            SurveyV2ByIdAction::StartSurvey(_) => ctrl.start_survey(id).await,
        }
    }

    pub async fn act_survey_v2(
        State(ctrl): State<SurveyControllerV2>,
        Path(org_id): Path<i64>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<SurveyV2Action>,
    ) -> Result<Json<SurveyV2>> {
        tracing::debug!("act_survey_v2 {:?}", body);
        if auth.is_none() {
            return Err(ApiError::Unauthorized);
        }

        match body {
            SurveyV2Action::Create(body) => ctrl.create(org_id, body).await,
            SurveyV2Action::Delete(body) => ctrl.delete(body.id).await,
        }
    }

    pub async fn get_survey_v2(
        State(ctrl): State<SurveyControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path((org_id, id)): Path<(i64, i64)>,
    ) -> Result<Json<SurveyV2>> {
        tracing::debug!("get_survey_v2 {:?}", id);
        let survey = ctrl
            .repo
            .find_one(&SurveyV2ReadAction::new().find_by_id(id))
            .await?;

        if survey.org_id != org_id {
            return Err(ApiError::Unauthorized);
        }

        Ok(Json(survey))
    }

    pub async fn list_survey_v2(
        State(ctrl): State<SurveyControllerV2>,
        Path(org_id): Path<i64>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(q): Query<SurveyV2Param>,
    ) -> Result<Json<SurveyV2GetResponse>> {
        tracing::debug!("list_survey_v2 {:?}", q);

        match q {
            SurveyV2Param::Query(q) => ctrl.find(org_id, q).await,
            _ => Err(ApiError::InvalidAction),
        }
    }
}

impl SurveyControllerV2 {
    pub async fn find(
        &self,
        org_id: i64,
        SurveyV2Query { size, bookmark, .. }: SurveyV2Query,
    ) -> Result<Json<SurveyV2GetResponse>> {
        let mut total_count: i64 = 0;

        // let query = SurveyV2Summary::base_sql_with("where org_id = $1 limit $2 offset $3");

        // FIXME: fix to this line bug
        // query.push_str(" order by id desc");
        // tracing::debug!("find query: {}", query);

        let items: Vec<SurveyV2Summary> = sqlx::query(
            &"SELECT 
        COUNT(*) OVER() AS total_count, 
        p.id, 
        p.created_at, 
        p.updated_at, 
        p.name, 
        p.project_type, 
        p.project_area, 
        p.status, 
        p.started_at, 
        p.ended_at, 
        p.quotes, 
        p.org_id, 
        p.panel_counts, 
        p.noncelab_id,
        COALESCE(
            jsonb_agg(to_jsonb(panels)) FILTER (WHERE panels.id IS NOT NULL), 
            '[]'::jsonb
        ) AS panels
    FROM surveys p 
    LEFT JOIN panel_surveys ps ON p.id = ps.survey_id
    LEFT JOIN panels panels ON ps.panel_id = panels.id
    WHERE p.org_id = $1 
    GROUP BY p.id, p.created_at, p.updated_at, p.name, p.project_type, 
             p.project_area, p.status, p.started_at, p.ended_at, p.quotes, 
             p.org_id, p.panel_counts, p.noncelab_id
    LIMIT $2 OFFSET $3;",
        )
        .bind(org_id)
        .bind(size as i64)
        .bind(size as i64 * (bookmark.unwrap_or("1".to_string()).parse::<i64>().unwrap() - 1))
        .map(|r: PgRow| {
            use sqlx::Row;

            total_count = r.get("total_count");
            r.into()
        })
        .fetch_all(&self.pool)
        .await?;

        Ok(Json(SurveyV2GetResponse::Query(QueryResponse {
            items,
            total_count,
        })))
    }

    pub async fn delete(&self, id: i64) -> Result<Json<SurveyV2>> {
        tracing::debug!("delete survey: {:?}", id);

        let _ = self.repo.delete(id).await?;

        Ok(Json(SurveyV2::default()))
    }

    pub async fn start_survey(&self, id: i64) -> Result<Json<SurveyV2>> {
        let mut survey = self
            .repo
            .find_one(&SurveyV2ReadAction::new().find_by_id(id))
            .await?;

        survey.status = ProjectStatus::InProgress;

        let survey_dto = survey.clone().into();

        tracing::info!("id: {} survey dto: {:?}", id, survey_dto);

        let noncelab_id = self.nonce_lab.create_survey(survey_dto).await?;

        let survey = self
            .repo
            .update(
                survey.clone().id,
                SurveyV2RepositoryUpdateRequest {
                    name: None,
                    project_type: None,
                    project_area: None,
                    status: Some(ProjectStatus::InProgress),
                    started_at: None,
                    ended_at: None,
                    description: None,
                    quotes: None,
                    org_id: None,
                    questions: None,
                    panel_counts: None,
                    noncelab_id: Some(noncelab_id as i64),
                },
            )
            .await?;

        Ok(Json(survey))
    }

    pub async fn update(
        &self,
        org_id: i64,
        id: i64,
        body: SurveyV2UpdateRequest,
    ) -> Result<Json<SurveyV2>> {
        //FIXME: receive panel params and update panel data
        let survey = self
            .repo
            .update(
                id,
                SurveyV2RepositoryUpdateRequest {
                    name: Some(body.name),
                    project_type: Some(body.project_type),
                    project_area: Some(body.project_area),
                    status: Some(body.status),
                    started_at: Some(body.started_at),
                    ended_at: Some(body.ended_at),
                    description: Some(body.description),
                    quotes: Some(body.quotes),
                    org_id: Some(org_id),
                    questions: Some(body.questions),
                    panel_counts: Some(body.panel_counts),
                    noncelab_id: None,
                },
            )
            .await?;
        Ok(Json(survey))
    }

    pub async fn create(
        &self,
        org_id: i64,
        SurveyV2CreateRequest {
            name,
            project_area,
            started_at,
            ended_at,
            description,
            quotes,
            questions,
            panels,
            panel_counts,
        }: SurveyV2CreateRequest,
    ) -> Result<Json<SurveyV2>> {
        tracing::debug!("create {:?}", org_id,);
        let mut tx = self.pool.begin().await?;

        let survey = match self
            .repo
            .insert_with_tx(
                &mut *tx,
                name,
                ProjectType::Survey,
                project_area,
                ProjectStatus::Ready,
                started_at,
                ended_at,
                description,
                quotes,
                org_id.clone(),
                questions,
                panel_counts,
                None,
            )
            .await?
        {
            Some(v) => v,
            None => return Err(ApiError::SurveyAlreadyExists),
        };

        for panel in panels.clone() {
            let _ = self
                .panel_survey_repo
                .insert_with_tx(&mut *tx, panel.id, survey.id)
                .await?;
        }

        tx.commit().await?;

        Ok(Json(survey))
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::tests::*;

    #[tokio::test]
    async fn test_survey_create() {
        let TestContext {
            user,
            now,
            endpoint,
            ..
        } = setup().await.unwrap();

        let cli = SurveyV2::get_client("http://localhost:3000");
        let org_id = user.orgs[0].id;

        let res = cli
            .create(
                org_id,
                "test".to_string(),
                ProjectArea::City,
                now,
                now + 3600,
                "test description".to_string(),
                100,
                vec![],
                vec![],
                vec![],
            )
            .await;

        assert!(res.is_ok(), "{:?}", res);
    }
}
