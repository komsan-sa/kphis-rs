use axum::{
    Json,
    body::Bytes,
    extract::{Path, State},
    http::{Method, Response, header},
};
use http_body_util::Full;
use time::OffsetDateTime;
use typst_layout::PagedDocument;
use typst_library::foundations::{Datetime, Smart};
use typst_pdf::{PdfOptions, PdfStandards, Timestamp, merge, pdf};

use kphis_api_core::{
    open_api::{DocOne, DocPdf},
    pdf::{
        loader::load_file,
        runtime::{ReadFn, SystemWorld},
        signer::write_sig_content,
    },
    state::{ApiState, RequestState, UserState},
};
use kphis_api_query::{
    ipd::document,
    report::{select_report_template, select_report_template_content},
};
use kphis_model::report::{ReportTemplateParams, SystemReport, TypstRaw, TypstReport};
use kphis_util::error::{AppError, Source};

use crate::bundle_data::pdf_data;

// return typ file
/// /customs/config.typ
pub async fn get_config_template(State(app): State<ApiState>) -> Result<Response<Full<Bytes>>, AppError> {
    let typ = get_config_template_inner(&app);
    let body = Full::new(Bytes::from(typ));

    Response::builder()
        .header(header::CONTENT_TYPE, "text/vnd.typst")
        .header(header::CONTENT_DISPOSITION, ["attachment; filename=\"config.typ\""].concat())
        .body(body)
        .map_err(|e| Source::App.to_error(500, e, "Get ConfigTemplate"))
}

/// generate config.typ from app config
pub fn get_config_template_inner(app: &ApiState) -> String {
    let config = &app.app_config;
    let score_ews = config.score_ews.iter().map(|s| s.label()).collect::<Vec<&'static str>>().join("\",\"");
    let score_qsofa = config.score_qsofa.iter().map(|s| s.label()).collect::<Vec<&'static str>>().join("\",\"");
    let score_sirs = config.score_sirs.iter().map(|s| s.label()).collect::<Vec<&'static str>>().join("\",\"");
    [
        "#let hospcode = \"",
        &config.hospcode,
        "\"
#let code-name = \"",
        &config.code_name,
        "\"
#let hospital-name = \"",
        &config.hospital_name,
        "\"
#let hospital-info = \"",
        &config.hospital_info,
        "\"
#let hospital-address = \"",
        &config.hospital_address,
        "\"
#let an-len = ",
        &config.hosxp_an_length.to_string(),
        "
#let score-ews = (\"",
        &score_ews,
        "\")
#let score-qsofa = (\"",
        &score_qsofa,
        "\")
#let score-sirs = (\"",
        &score_sirs,
        "\")
#let drug_alert(c) = {
  ((c == ",
        &config.hosxp_had_displaycolor.unwrap_or(255).to_string(),
        ",\"HAD\"),
  (c == ",
        &config.hosxp_lasa_displaycolor.unwrap_or(16711935).to_string(),
        ",\"LASA\"),
  (true, none)).find(t => t.at(0)).at(1)
}
#let is_inj(c) = if c == none {false} else {(\"",
        &config.hosxp_injection_dosageforms.clone().join("\",\""),
        "\").contains(c)}",
    ]
    .concat()
}

// return typ file
/// /customs/{template.typ}
pub async fn get_custom_template(Path(template_with_ext): Path<String>, State(app): State<ApiState>) -> Result<Response<Full<Bytes>>, AppError> {
    let template = template_with_ext.trim_end_matches(".typ");
    let typ = select_report_template_content(template, &app.db_pool, &app.kphis_extra())
        .await?
        .ok_or_else(|| Source::App.to_error(404, "Template Not Found", "Get CustomTemplate"))?;
    let body = Full::new(Bytes::from(typ));

    Response::builder()
        .header(header::CONTENT_TYPE, "text/vnd.typst")
        .header(header::CONTENT_DISPOSITION, ["attachment; filename=\"", template, ".typ\""].concat())
        .body(body)
        .map_err(|e| Source::App.to_error(500, e, "Get CustomTemplate"))
}

// return pdf file
/// /api/report/template-type-id/{template}/{type}/{id}
///
/// Get PDF file by Template Name and ID, return PDF file
#[utoipa::path(
    get,
    path = "/report/template-type-id/{template}/{type}/{id}",
    responses(DocPdf),
    params(
        ("template" = String, Path, description = "Typst template file name", example = "ipd-consult"),
        ("type" = String, Path, description = "Type of Typst template (system, coercion, custom)", example = "system"),
        ("id" = String, Path, description = "IDs needed by Typst template, concat with pipe (|)", example = "660001234"),
    ),
)]
pub async fn get_single_pdf(Path((template, report_type, ids)): Path<(String, String, String)>, ctx: RequestState) -> Result<Response<Full<Bytes>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    if report_type.as_str() == "system" {
        if ["full-general", "full-labour", "full-psychia"].contains(&template.as_str()) {
            get_full_report_pdf(template, ids, ctx.api_state, ctx.user_state).await
        } else {
            get_single_report_pdf(template, report_type, ids, ctx.api_state, ctx.user_state).await
        }
    } else {
        get_single_report_pdf(template, report_type, ids, ctx.api_state, ctx.user_state).await
    }
}

// return template.typ + data.json
/// /api/report/raw-template-type-id/{template}/{type}/{id}
///
/// Get Typst template with data by Template Name and ID, return Typst template with data
/// - Template name of coercion type is System template name (NOT custom name)
#[utoipa::path(
    get,
    path = "/report/raw-template-type-id/{template}/{type}/{id}",
    responses(DocOne<TypstRaw>),
    params(
        ("template" = String, Path, description = "Typst template file name", example = "ipd-consult"),
        ("type" = String, Path, description = "Type of Typst template (system, coercion, custom)", example = "system"),
        ("id" = String, Path, description = "IDs needed by Typst template, concat with pipe (|)", example = "660001234"),
    ),
)]
pub async fn get_raw_single_template(Path((template, report_type, ids)): Path<(String, String, String)>, ctx: RequestState) -> Result<Json<TypstRaw>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    if let Some(typst_report) = new_typst_report(template, report_type, &ctx.api_state).await? {
        let (typ, data_json) = prepare_template_data(&typst_report, &ids, &ctx.api_state, &ctx.user_state).await?;

        Ok(Json(TypstRaw { typ, data_json }))
    } else {
        Err(Source::App.to_error(404, "Template Not Found", "Get Template"))
    }
}

async fn get_single_report_pdf(template: String, report_type: String, ids: String, api_state: ApiState, user_state: UserState) -> Result<Response<Full<Bytes>>, AppError> {
    if let Some(typst_report) = new_typst_report(template.to_owned(), report_type.to_owned(), &api_state).await? {
        let (typ_file, data_json) = prepare_template_data(&typst_report, &ids, &api_state, &user_state).await?;
        let file_name = typst_report.download_file_name(&ids);

        let mut world = SystemWorld::new();
        // we need spawn_blocking() here to prevent deadlock from JsonActor.rt.block_on() in current async context
        let pdf_buf = tokio::task::spawn_blocking(move || create_pdf(&typ_file, &data_json, &typst_report.title_with_ids(&ids), &api_state, &user_state, &mut world, load_file))
            .await
            .map_err(|e| Source::App.to_error(500, e, "Create SinglePDF"))??;
        let body = Full::new(Bytes::from(pdf_buf));

        Response::builder()
            .header(header::CONTENT_TYPE, "application/pdf")
            .header(header::CONTENT_DISPOSITION, ["attachment; filename=\"", &file_name, ".pdf\""].concat())
            .body(body)
            .map_err(|e| Source::App.to_error(500, e, "Create SinglePDF"))
    } else {
        Err(Source::App.to_error(404, "Template Not Found", "Create SinglePDF"))
    }
}

// ids MUST be vn|an
async fn get_full_report_pdf(template: String, ids: String, api_state: ApiState, user_state: UserState) -> Result<Response<Full<Bytes>>, AppError> {
    let vn_an = ids.split("|").collect::<Vec<&str>>();
    if vn_an.len() != 2 {
        return Err(AppError::app_400("Create FullReportPDF"));
    }
    let exists = document::get_ipd_document_list(vn_an[0], vn_an[1], &api_state.db_pool, &api_state.hosxp(), &api_state.kphis(), &api_state.kphis_extra()).await?;
    let reports = TypstReport::full_report(&template, exists, vn_an[1], &api_state.app_config.report_coercions);

    let mut typ_raws = Vec::with_capacity(41);
    for (typst_report, ids) in reports {
        let typ_raw = prepare_template_data(&typst_report, &ids, &api_state, &user_state).await?;
        typ_raws.push(typ_raw);
    }

    // we need spawn_blocking() here to prevent deadlock from JsonActor.rt.block_on() in current async context
    let pdf_buf = tokio::task::spawn_blocking(move || merge_pdf(&typ_raws, &api_state, &user_state))
        .await
        .map_err(|e| Source::App.to_error(500, e, "Create FullReportPDF"))??;
    let body = Full::new(Bytes::from(pdf_buf));

    let file_name = [vn_an[1], "-", &template.to_uppercase()].concat();

    Response::builder()
        .header(header::CONTENT_TYPE, "application/pdf")
        .header(header::CONTENT_DISPOSITION, ["attachment; filename=\"", &file_name, ".pdf\""].concat())
        .body(body)
        .map_err(|e| Source::App.to_error(500, e, "Create SinglePDF"))
}

/// new with complete coercion data
pub async fn new_typst_report(template: String, report_type: String, app: &ApiState) -> Result<Option<TypstReport>, AppError> {
    let result = match report_type.as_str() {
        "system" => SystemReport::new(&template).map(|r| TypstReport::System(r)),
        "coercion" => {
            match TypstReport::new_system_with_coercion(&template, app.app_config.report_coercions.clone()) {
                Some(typst_report) => {
                    match typst_report {
                        // fill custom data of coersion
                        TypstReport::Coercion((name, system, _)) => {
                            let custom = select_report_template(
                                &ReportTemplateParams {
                                    template_name: Some(template.clone()),
                                    ..Default::default()
                                },
                                &app.db_pool,
                                &app.hosxp(),
                                &app.kphis_extra(),
                            )
                            .await?
                            .first()
                            .cloned();
                            Some(TypstReport::Coercion((name, system, custom)))
                        }
                        _ => Some(typst_report),
                    }
                }
                // not system or coercion
                None => select_report_template(
                    &ReportTemplateParams {
                        template_name: Some(template.clone()),
                        ..Default::default()
                    },
                    &app.db_pool,
                    &app.hosxp(),
                    &app.kphis_extra(),
                )
                .await?
                .first()
                .map(|cr| TypstReport::Custom(cr.clone())),
            }
        }
        // custom
        _ => select_report_template(
            &ReportTemplateParams {
                template_name: Some(template),
                ..Default::default()
            },
            &app.db_pool,
            &app.hosxp(),
            &app.kphis_extra(),
        )
        .await?
        .first()
        .map(|cr| TypstReport::Custom(cr.to_owned())),
    };
    Ok(result)
}

pub async fn prepare_template_data(typst_report: &TypstReport, ids: &str, app: &ApiState, user: &UserState) -> Result<(String, String), AppError> {
    let typ = get_typ(typst_report, app).await?;
    let data_json = pdf_data(typst_report, ids, app, user).await?;

    Ok((typ, data_json))
}

async fn get_typ(typst_report: &TypstReport, app: &ApiState) -> Result<String, AppError> {
    let typ = match &typst_report {
        TypstReport::System(system_report) => tokio::fs::read_to_string(&system_report.typ_path_server())
            .await
            .map_err(|e| Source::App.to_error(500, e, "Get Template"))?,
        TypstReport::Coercion((custom_template, system_report, _)) => match select_report_template_content(custom_template, &app.db_pool, &app.kphis_extra()).await? {
            Some(report) => report,
            None => {
                tracing::warn!("Custom template '{}' not found, use default template instead", &custom_template);
                tokio::fs::read_to_string(&system_report.typ_path_server())
                    .await
                    .map_err(|e| Source::App.to_error(500, e, "Get Template"))?
            }
        },
        TypstReport::Custom(custom_report) => select_report_template_content(&custom_report.template_name, &app.db_pool, &app.kphis_extra())
            .await?
            .ok_or_else(|| Source::App.to_error(404, "Template Not Found", "Get Template"))?,
    };

    Ok(typ)
}

pub fn create_pdf(typ_file: &str, data_json: &str, title: &str, app: &ApiState, user: &UserState, world: &mut SystemWorld, read_fn: ReadFn) -> Result<Vec<u8>, AppError> {
    let mut document = create_paged_document(typ_file, data_json, app, user, world, read_fn)?;
    generate_pdf_file(&mut document, title, app, user)
}

fn merge_pdf(typ_raws: &[(String, String)], app: &ApiState, user: &UserState) -> Result<Vec<u8>, AppError> {
    let mut world = SystemWorld::new();
    let (pdf_option, need_signing) = generate_pdf_option(app);

    let mut documents = Vec::with_capacity(38);
    for (typst_file, data_json) in typ_raws {
        let document = create_paged_document(&typst_file, &data_json, app, &user, &mut world, load_file)?;
        documents.push(document);
    }

    let pdf_unsign_buf = merge(&documents, &pdf_option).map_err(|errors| {
        let message = errors.iter().map(|e| e.message.to_string()).collect::<Vec<String>>().join(", ");
        Source::App.to_error(500, message, "Create FullReportPDF")
    })?;

    let pdf_buf = if need_signing { write_sig_content(pdf_unsign_buf, app)? } else { pdf_unsign_buf };

    Ok(pdf_buf)
}

pub fn create_paged_document(typ_file: &str, data_json: &str, app: &ApiState, user: &UserState, world: &mut SystemWorld, read_fn: ReadFn) -> Result<PagedDocument, AppError> {
    world
        .compile(typ_file, data_json, read_fn, Some(app.clone()), Some(user.clone()))
        .map_err(|e| Source::Typst.to_error(500, e, "Create PagedDocument"))
}

pub fn generate_pdf_file(document: &mut PagedDocument, title: &str, app: &ApiState, user: &UserState) -> Result<Vec<u8>, AppError> {
    let info = document.info_mut();
    info.title = Some(title.into());
    info.author = vec![app.app_config.hospital_name.clone().into(), user.user.name.clone().into()];

    let (pdf_option, need_signing) = generate_pdf_option(app);

    let pdf_unsign_buf = pdf(&document, &pdf_option).map_err(|errors| {
        let message = errors.iter().map(|e| e.message.to_string()).collect::<Vec<String>>().join(", ");
        Source::App.to_error(500, message, "Create PDF")
    })?;

    let pdf_buf = if need_signing { write_sig_content(pdf_unsign_buf, app)? } else { pdf_unsign_buf };

    Ok(pdf_buf)
}

pub fn generate_pdf_option(app: &ApiState) -> (PdfOptions, bool) {
    let now = std::time::SystemTime::now();
    let dt = OffsetDateTime::from(now);
    let timestamp = Datetime::from_ymd_hms(dt.year(), dt.month().into(), dt.day(), dt.hour(), dt.minute(), dt.second()).map(Timestamp::new_utc);

    let signer = app.pdf_signer.as_ref().map(|s| s.sig.clone());
    let need_signing = signer.is_some();

    let pdf_option = PdfOptions {
        ident: Smart::Auto,
        creator: Smart::Auto,
        timestamp,
        page_ranges: None,
        standards: PdfStandards::default(),
        tagged: false,
        pretty: false,
        signer,
    };

    (pdf_option, need_signing)
}
