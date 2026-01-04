#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::prelude::*;
    use leptos::config::get_configuration;
    use leptos_meta::MetaTags;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use dashboard_app_rs::app::*;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();
        // Generate the routes inside the factory
        let routes = generate_route_list(App);

        App::new()
            // 1. Handle Server Functions
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            // 2. Serve WASM and JS
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // 3. Serve Assets
            .service(Files::new("/assets", &site_root))
            .service(favicon)
            // 4. Corrected Leptos Routes signature
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || {
                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta charset="utf-8"/>
                                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                <HydrationScripts options=leptos_options.clone()/>
                                <MetaTags/>
                            </head>
                            <body>
                                <App/>
                            </body>
                        </html>
                    }
                }
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!("{site_root}/favicon.ico"))?)
}