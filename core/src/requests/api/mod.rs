use std::time::Instant;

use axum::{
    headers::ContentType, http::StatusCode, middleware, routing::post, Extension, Json, Router,
    TypedHeader,
};
use monitor_types::requests::api::*;
use resolver_api::{derive::Resolver, Resolve, ResolveToString, Resolver};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use uuid::Uuid;

use crate::{
    auth::{auth_request, RequestUser, RequestUserExtension},
    state::{State, StateExtension},
};

mod build;
mod builder;
mod deployment;
mod permissions;
mod repo;
mod secret;
mod server;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Resolver)]
#[resolver_target(State)]
#[resolver_args(RequestUser)]
#[serde(tag = "type", content = "params")]
#[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
pub enum ApiRequest {
    // ==== SECRET ====
    CreateLoginSecret(CreateLoginSecret),
    DeleteLoginSecret(DeleteLoginSecret),

    // ==== PERMISSIONS ====
    UpdateUserPerimissions(UpdateUserPermissions),
    UpdateUserPermissionsOnTarget(UpdateUserPermissionsOnTarget),

    // ==== SERVER ====
    GetPeripheryVersion(GetPeripheryVersion),
    GetSystemInformation(GetSystemInformation),
    GetDockerContainers(GetDockerContainers),
    GetDockerImages(GetDockerImages),
    GetDockerNetworks(GetDockerNetworks),
    GetServer(GetServer),
    ListServers(ListServers),
    // CRUD
    CreateServer(CreateServer),
    DeleteServer(DeleteServer),
    UpdateServer(UpdateServer),
    RenameServer(RenameServer),
    // STATS
    #[to_string_resolver]
    GetAllSystemStats(GetAllSystemStats),
    #[to_string_resolver]
    GetBasicSystemStats(GetBasicSystemStats),
    #[to_string_resolver]
    GetCpuUsage(GetCpuUsage),
    #[to_string_resolver]
    GetDiskUsage(GetDiskUsage),
    #[to_string_resolver]
    GetNetworkUsage(GetNetworkUsage),
    #[to_string_resolver]
    GetSystemProcesses(GetSystemProcesses),
    #[to_string_resolver]
    GetSystemComponents(GetSystemComponents),
    // ACTIONS
    PruneContainers(PruneDockerContainers),
    PruneImages(PruneDockerImages),
    PruneNetworks(PruneDockerNetworks),

    // ==== DEPLOYMENT ====
    GetDeployment(GetDeployment),
    ListDeployments(ListDeployments),
    // CRUD
    CreateDeployment(CreateDeployment),
    DeleteDeployment(DeleteDeployment),
    UpdateDeployment(UpdateDeployment),
    RenameDeployment(RenameDeployment),
    // ACTIONS
    Deploy(Deploy),
    StartContainer(StartContainer),
    StopContainer(StopContainer),
    RemoveContainer(RemoveContainer),

    // ==== BUILD ====
    GetBuild(GetBuild),
    ListBuilds(ListBuilds),
    // CRUD
    CreateBuild(CreateBuild),
    DeleteBuild(DeleteBuild),
    UpdateBuild(UpdateBuild),
    // ACTIONS
    RunBuild(RunBuild),

    // ==== BUILDER ====
    GetBuilder(GetBuilder),
    ListBuilders(ListBuilders),
    // CRUD
    CreateBuilder(CreateBuilder),
    DeleteBuilder(DeleteBuilder),
    UpdateBuilder(UpdateBuilder),

    // ==== REPO ====
    GetRepo(GetRepo),
    ListRepos(ListRepos),
    // CRUD
    CreateRepo(CreateRepo),
    UpdateRepo(UpdateRepo),
    DeleteRepo(DeleteRepo),
    // ACTIONS
    CloneRepo(CloneRepo),
    PullRepo(PullRepo),
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            post(
                |state: StateExtension,
                 Extension(user): RequestUserExtension,
                 Json(request): Json<ApiRequest>| async move {
                    let timer = Instant::now();
                    let req_id = Uuid::new_v4();
                    info!("/api request {req_id} | {request:?}");
                    let res = tokio::spawn(async move {
                        state
                            .resolve_request(request, user)
                            .await
                            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{e:?}")))
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{e:?}")));
                    if let Err(e) = &res {
                        info!("/api request {req_id} SPAWN ERROR: {e:?}");
                    }
                    let res = res?;
                    if let Err(e) = &res {
                        info!("/api request {req_id} ERROR: {e:?}");
                    }
                    let res = res?;
                    let elapsed = timer.elapsed();
                    info!("/api request {req_id} | resolve time: {elapsed:?}");
                    debug!("/api request {req_id} RESPONSE: {res}");
                    Result::<_, (StatusCode, String)>::Ok((TypedHeader(ContentType::json()), res))
                },
            ),
        )
        .layer(middleware::from_fn(auth_request))
}
