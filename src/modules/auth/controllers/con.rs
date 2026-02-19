use crate::{
    common::{ApiResult, Ctx},
    modules::auth::dto::req::LoginReq,
};

pub async fn login(ctx: Ctx) -> ApiResult {
    let req: LoginReq = ctx.body()?;

    println!("{:?}", req);

    ctx.json("")
}
