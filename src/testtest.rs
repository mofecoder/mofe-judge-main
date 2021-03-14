mod config;
#[tokio::test(threaded_scheduler)]
async fn my_test() {
    let db_conn = Arc::new(db::new_pool(config::load_config().unwrap()).unwrap());
    let task = JudgeTask {
        db_conn,
        docker_conn: Arc::new(bollard::Docker::connect_with_http_defaults().unwrap()),
        json_map: (),
    };
    assert!(true);
}
