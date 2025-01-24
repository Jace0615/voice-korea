use by_axum::{
    axum::{
        extract::{Path, Query, State},
        middleware,
        routing::post,
        Extension, Json, Router,
    },
    log::root,
};
use slog::o;
use crate::{
    common::CommonQueryResponse, middleware::auth::authorization_middleware
};
use models::prelude::*;

#[derive(Clone, Debug)]
pub struct PanelControllerV1 {
    log: slog::Logger,
}

impl PanelControllerV1 {
    pub fn router() -> Router {
        let log = root().new(o!("api-controller" => "PanelControllerV1"));
        let ctrl = PanelControllerV1 { log };

        Router::new()
            .route("/", post(Self::act_panel).get(Self::list_panels))
            .route(
                "/:panel_id",
                post(Self::act_panel_by_id).get(Self::get_panel),
            )
            .with_state(ctrl)
            .layer(middleware::from_fn(authorization_middleware))
    }

    pub async fn act_panel(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<PanelControllerV1>,
        Json(body): Json<PanelActionRequest>,
    ) -> Result<(), ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "act_panel"));
        slog::debug!(log, "act_panel: {:?} {:?}", organization_id, body);

        match body {
            PanelActionRequest::Create(req) => {
                ctrl.create_panel(&organization_id, req).await?;
            }
        }

        Ok(())
    }

    pub async fn act_panel_by_id(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<PanelControllerV1>,
        Path(panel_id): Path<String>,
        Json(body): Json<PanelByIdActionRequest>,
    ) -> Result<(), ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "act_panel_by_id"));
        slog::debug!(log, "act_panel_by_id: {:?} {:?}", organization_id, panel_id);

        match body {
            PanelByIdActionRequest::Delete => {
                ctrl.remove_panel(&organization_id, &panel_id).await?;
            }
            PanelByIdActionRequest::Update(req) => {
                ctrl.update_panel(&organization_id, &panel_id, req).await?;
            }
        }

        Ok(())
    }

    pub async fn get_panel(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<PanelControllerV1>,
        Path(panel_id): Path<String>,
    ) -> Result<Json<()>, ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "get_panel"));
        slog::debug!(log, "get_panel: {:?} {:?}", organization_id, panel_id);

        Ok(Json(()))
    }

    pub async fn list_panels(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<PanelControllerV1>,
        Query(pagination): Query<Pagination>,
    ) -> Result<(), ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "list_panels"));
        slog::debug!(log, "list_panels {:?} {:?}", organization_id, pagination);

        // Ok(Json(CommonQueryResponse {
        //     items: vec![],
        //     bookmark: None,
        // }))
        Ok(())
    }
}

impl PanelControllerV1 {
    pub async fn create_panel(
        &self,
        organization_id: &str,
        body: CreatePanelRequest,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "create_panel"));
        slog::debug!(log, "create_panel {:?} {:?}", organization_id, body);
        let cli = easy_dynamodb::get_client(&log);

        let panel = PanelInfo::new(
            organization_id.to_string(), 
            body.name, 
            body.count
        );

        let _ = cli
            .upsert(&panel)
            .await
            .map_err(|e| ApiError::DynamoCreateException(e.to_string()))?;

        for item in body.attribute {
            let attribute = PanelAttributeItem::new(
                panel.id.clone(),
                item.attribute_id
            );

            let _ = cli
                .upsert(&attribute)
                .await
                .map_err(|e| ApiError::DynamoCreateException(e.to_string()))?;
        }

        Ok(())
    }
}

impl PanelControllerV1 {
    pub async fn remove_panel(
        &self,
        organization_id: &str,
        panel_id: &str,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "remove_panel"));
        slog::debug!(log, "remove_panel {:?} {:?}", organization_id, panel_id);
        Ok(())
    }

    pub async fn update_panel(
        &self,
        organization_id: &str,
        panel_id: &str,
        body: UpdatePanelRequest,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "update_panel"));
        slog::debug!(
            log,
            "update_panel {:?} {:?} {:?}",
            organization_id,
            panel_id,
            body
        );
        Ok(())
    }
}
