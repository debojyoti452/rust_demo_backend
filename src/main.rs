/**
 * * GNU General Public License v3.0
 * *******************************************************************************************
 *  * Created By Debojyoti Singha
 *  * Copyright (c) 29 May, 2023.
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  *
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *  * Contact Email: support@swingtechnologies.in
 * ******************************************************************************************
 */
use rocket::{get, http::Status, serde::json::Json};
use serde::Serialize;

#[macro_use]
extern crate rocket;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "API Status ok.", body = [GenericResponse]),
        (status = 500, description = "Internal server error.", body = [GenericResponse]),
        (status = 503, description = "Service unavailable.", body = [GenericResponse]),
    )
)]
#[get("/status")]
pub async fn health_checker_handler() -> Result<Json<GenericResponse>, Status> {
    const MESSAGE: &str = "API is up and running.";

    let response_json = GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(Json(response_json))
}

#[launch]
fn rocket() -> _ {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            health_checker_handler,
        ),
        tags(
            (name = "Rust Demo APIs", description = "Rust Demo Showcase APIs.")
        ),
    )]
    struct ApiDoc;

    rocket::build()
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .mount("/api", routes![health_checker_handler])
}
