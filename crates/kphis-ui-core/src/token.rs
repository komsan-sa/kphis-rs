use base64::{Engine, engine::general_purpose};
use discard::Discard;
use dominator::append_dom;
use futures_signals::signal::SignalExt;
use pasetors::{
    Public,
    keys::AsymmetricPublicKey,
    token::UntrustedToken,
    version4::{PublicToken, V4},
};
use std::rc::Rc;

use kphis_model::{
    SERVER_ENTITY,
    app::AppState,
    claim::Claims,
    user::his::{LoginResponse, UserClient},
};
use kphis_util::{
    datetime::get_timestamp_wasm,
    error::{AppError, ErrorTitle, Source},
    util::add_u64_with_i64,
};

use crate::popups::{PopupAuth, prompt_password::PromptPasswordPopup};

// tracing by change `// log::debug!` to `log::debug!`
/// check access token and refresh token, update user and token to the most usable app.token()
pub async fn update_token(app: Rc<AppState>) -> bool {
    match app.user.get_cloned() {
        Some(user) => {
            if user.authorized() {
                // | X | access expired |   | last xx minutes |   | refresh expired |   |
                // using old access token if old access token not expired
                // log::debug!("Access Token Valid");
                true
            } else {
                // log::debug!("Access Token Invalid");
                match app.token_rexp_with_earlier() {
                    Some((rexp, earlier)) => {
                        // |   | access expired | ? | last xx minutes | ? | refresh expired | ? |
                        // access token expired, renew access token
                        let now = get_timestamp_wasm();
                        let now_adj = add_u64_with_i64(now, earlier).saturating_add(5); // make client-now == server-now
                        // log::debug!("Token TS: now {} rexp {}", now, rexp);
                        if (now_adj + (app.reauthen_before_expired_minutes() * 60)) < rexp {
                            // |   | access expired | X | last xx minutes |   | refresh expired |   |
                            // refresh token will not expored in xx mins (with 5 secs transport time), renew access token
                            // log::debug!("Refresh Token Valid");
                            renew_access_token(app.clone()).await
                        } else if rexp < now_adj {
                            // |   | access expired |   | last xx minutes |   | refresh expired | X |
                            // refresh token expired (with 5 seconds transfer time)
                            // log::debug!("Refresh Token Invalid");
                            false
                        } else {
                            // |   | access expired |   | last xx minutes | X | refresh expired | X |
                            // refresh token will expired in xx mins, renew refresh token
                            let popup = PromptPasswordPopup::new(user.user.totp_done.get().unwrap_or_default());
                            // bootstrap modal will lock focus only within .modal-content
                            // so we need to append to '.modal.show .modal-body' if exist
                            match app.query_selector(".modal.show .modal-body").or(app.get_id("popup")) {
                                Some(parent) => {
                                    let handle = append_dom(&parent, PromptPasswordPopup::render(popup.clone(), app.clone()));
                                    popup.focus(app.clone());
                                    match popup.finished().wait_for(true).await {
                                        Some(is_fin) => {
                                            if is_fin {
                                                let result = popup.result.get_cloned();
                                                handle.discard();
                                                match result {
                                                    PopupAuth::Ok(password, token_2fa) => {
                                                        // log::debug!("Renew Refresh Token");
                                                        renew_refresh_token(&password, &token_2fa, app).await
                                                    }
                                                    PopupAuth::Cancel => {
                                                        // log::debug!("Cancel Renew Refresh Token");
                                                        false
                                                    }
                                                }
                                            } else {
                                                // log::debug!("wait_for return Some(false)");
                                                false
                                            }
                                        }
                                        None => {
                                            // log::debug!("wait_for return None");
                                            false
                                        }
                                    }
                                }
                                None => {
                                    // log::debug!("no modal or popup element");
                                    false
                                }
                            }
                        }
                    }
                    None => {
                        // log::debug!("Access Token Invalid and No exp in Refresh Token");
                        false
                    }
                }
            }
        }
        None => {
            // log::debug!("No user data, try using refresh token");
            renew_access_token(app.clone()).await
        }
    }
}

pub fn set_user(token_response: Option<LoginResponse>, app: Rc<AppState>) -> Result<(), AppError> {
    if let Some(LoginResponse {
        token,
        timestamp,
        public,
        app_status,
        user,
        roles,
        permissions,
    }) = token_response
    {
        let now = get_timestamp_wasm();
        let earlier_second = if timestamp < now {
            -(now.saturating_sub(timestamp) as i64)
        } else {
            timestamp.saturating_sub(now) as i64
        };
        // log::debug!("Client time is {} seconds earlier than server", earlier_second);
        let claim = get_claim_and_verify(&token, &public, earlier_second)?;
        match app.user.get_cloned() {
            // update current user
            // - update 2FA (MenuCpn::submit_2fa)
            // - update access token (`update_token()` when `render()` and `async_load()`)
            // - update refresh token (`update_token()` when `render()` and `async_load()`)
            Some(user_client) => {
                user_client.user.name.set_neq(user.name);
                user_client.user.doctorcode.set_neq(user.doctorcode);
                user_client.user.groupname.set_neq(user.groupname);
                user_client.user.accessright.set_neq(user.accessright);
                user_client.user.entryposition.set_neq(user.entryposition);
                user_client.user.licenseno.set_neq(user.licenseno);
                user_client.user.theme.set_neq(user.theme);
                user_client.user.wide_screen.set_neq(user.wide_screen);
                user_client.user.totp_done.set_neq(user.totp_done);
                {
                    let mut roles_lock = user_client.roles.lock_mut();
                    roles_lock.clear();
                    roles_lock.extend(roles.into_iter().map(|role| Rc::new(role.into())));
                }
                user_client.permissions.set(permissions);
                user_client.token.set_neq(token);
                user_client.sub.set_neq(claim.sub);
                user_client.iat.set_neq(claim.iat);
                user_client.exp.set_neq(claim.exp);
                user_client.rexp.set_neq(claim.rexp);
                user_client.earlier_second.set_neq(earlier_second);
            }
            None => {
                // add new user
                // - IndexPage::submit
                // - IndexPage::submit_2fa
                let client = UserClient {
                    user,
                    roles,
                    permissions,
                    token,
                    sub: claim.sub,
                    iat: claim.iat,
                    exp: claim.exp,
                    rexp: claim.rexp,
                    earlier_second,
                };
                // clear SSE messages and InMemory stuff
                app.clear_in_memory_except_user();
                app.user.set(Some(Rc::new(client.into())));
            }
        }
        app.app_status.set(Some(Rc::new(app_status)));
        app.to_local_storage();
        // log::debug!("Set User Done");

        Ok(())
    } else {
        Ok(())
    }
}

/// GET `EndPoint::User`
pub async fn renew_access_token(app: Rc<AppState>) -> bool {
    match LoginResponse::call_api_get_access_renew(app.clone()).await {
        Ok(token_response) => {
            if let Err(e) = set_user(Some(token_response.clone()), app.clone()) {
                log::warn!("Token error: {}", e.message);
                false
            } else {
                true
            }
        }
        Err(e) => {
            log::warn!("Server return: {}", e.message);
            false
        }
    }
}

/// PUT `EndPoint::User`
pub async fn renew_refresh_token(password: &str, token_2fa: &str, app: Rc<AppState>) -> bool {
    match LoginResponse::call_api_put_refresh_renew(password, token_2fa, app.clone()).await {
        Ok(token_response) => {
            if let Err(e) = set_user(Some(token_response.clone()), app.clone()) {
                log::warn!("Token error: {}", e.message);
                false
            } else {
                true
            }
        }
        Err(e) => {
            log::warn!("Server return: {}", e.message);
            false
        }
    }
}

fn get_claim_and_verify(token: &str, key: &str, earlier_second: i64) -> Result<Claims, AppError> {
    let claims = get_claim_encoded_key_public(token, key)?;
    let now = get_timestamp_wasm();
    let now_adj = add_u64_with_i64(now, earlier_second);

    if now_adj > claims.exp {
        return Err(Source::App.to_teapot_error("Expired", "Verify Token"));
    }
    // we allow more 30 seconds earier than server
    if claims.iat > now_adj + 30 {
        return Err(Source::App.to_teapot_error(&["Client time is ", &(claims.iat - now).to_string(), " seconds earlier than server time"].concat(), "Verify Token"));
    }

    Ok(claims)
}

// CANNOT share in kphis-model due to getrandom version
// integration test at tests/src/token.rs
pub fn get_claim_encoded_key_public(token: &str, key: &str) -> Result<Claims, AppError> {
    let key_vec = general_purpose::URL_SAFE_NO_PAD.decode(key).map_err(|e| Source::Base64.to_error(401, e, "Verify Token"))?;
    let public_key = AsymmetricPublicKey::<V4>::from(&key_vec).map_err(|_e| AppError::app_401("Verify Token"))?;

    get_claim_public(token, &public_key)
}

fn get_claim_public(token: &str, public_key: &AsymmetricPublicKey<V4>) -> Result<Claims, AppError> {
    let untrusted_token = UntrustedToken::<Public, V4>::try_from(token).map_err(|_e| AppError::app_401("Verify Token"))?;
    let trusted_token = PublicToken::verify(public_key, &untrusted_token, None, Some(SERVER_ENTITY.as_bytes())).map_err(|_e| AppError::app_401("Verify Token"))?;

    let claims_string = trusted_token.payload();
    let claims = serde_json::from_str::<Claims>(claims_string).map_err(|_e| AppError::app_401("Verify Token").with_title(ErrorTitle::Security))?;

    Ok(claims)
}
