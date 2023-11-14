use std::path::Path;

pub struct StartupChecker {
    pub config: Option<super::Config>,
}

impl StartupChecker {
    pub fn new() -> Self {
        Self { config: None }
    }

    pub fn check(&mut self) {
        let result = self.check_config();
        if !result {
            self.create_config();
        }
        self.load_config();
    }

    fn check_config(&self) -> bool {
        let config_path = dirs::home_dir()
            .unwrap()
            .join(".config")
            .join("kctrlstimer")
            .join("config.yaml");
        if !Path::is_file(&config_path) {
            false
        } else {
            true
        }
    }

    fn create_config(&self) {
        let config_path = dirs::home_dir()
            .unwrap()
            .join(".config")
            .join("kctrlstimer")
            .join("config.yaml");
        let config_dir = config_path.parent().unwrap();
        if !Path::is_dir(&config_dir) {
            std::fs::create_dir_all(&config_dir).unwrap();
        }
        let config = super::Config {
            dark_mode: false,
            width: 400,
            height: 200,
            alert_config: vec![
                super::AlertConfig {
                    time_sec: 60,
                    color: "#be4bdb".to_string(),
                },
                super::AlertConfig {
                    time_sec: 120,
                    color: "#fd7e14".to_string(),
                },
                super::AlertConfig {
                    time_sec: 180,
                    color: "#fa5252".to_string(),
                },
            ],
        };
        let config_str = serde_yaml::to_string(&config).unwrap();
        std::fs::write(&config_path, config_str).unwrap();
    }

    fn load_config(&mut self) {
        let config_path = dirs::home_dir()
            .unwrap()
            .join(".config")
            .join("kctrlstimer")
            .join("config.yaml");
        let config_str = std::fs::read_to_string(&config_path).unwrap();
        let config: super::Config = serde_yaml::from_str(&config_str).unwrap();
        self.config = Some(config);
    }

    pub fn save_config(&self) {
        let config_path = dirs::home_dir()
            .unwrap()
            .join(".config")
            .join("kctrlstimer")
            .join("config.yaml");
        println!("{:?}", self.config.as_ref().unwrap().dark_mode);
        let config_str = serde_yaml::to_string(&self.config).unwrap();
        std::fs::write(&config_path, config_str).unwrap();
    }
}
