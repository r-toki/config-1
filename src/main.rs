use app_config::ENV;

mod app_config;

fn main() {
    assert_eq!(ENV.frontend_origin, "http://127.0.0.1:3000");
    assert_eq!(ENV.host, "127.0.0.1");
    assert_eq!(ENV.port, "8080");
    assert_eq!(ENV.database_url, "postgresql://127.0.0.1:5432/config_1");
}
