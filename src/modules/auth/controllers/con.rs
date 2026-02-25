use crate::{
    common::{ApiResult, Ctx, utils::cookies::set_token_cookie},
    modules::auth::{
        dto::req::{LoginReq, RegisterReq},
        usecases::usecases,
    },
};

pub async fn login(ctx: Ctx) -> ApiResult {
    let req: LoginReq = ctx.body()?;

    let token = usecases::login(ctx.db(), req).await?;
    set_token_cookie(ctx.cookies(), token, "token".to_string(), 7);

    ctx.json("")
}

pub async fn register(ctx: Ctx) -> ApiResult {
    let req: RegisterReq = ctx.body()?;

    let token = usecases::register(ctx.db(), req).await?;

    set_token_cookie(ctx.cookies(), token.clone(), "token".to_string(), 7);
    ctx.json(token)
}
