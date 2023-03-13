use crate::response::{AppError, ServiceResult};
use path_slash::PathBufExt as _;
use rocket::fs::{NamedFile, TempFile};
use rocket::http::MediaType;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub struct Storage {
    pub path: PathBuf,
    pub user_avatars_storage: SubStorage,
}

pub struct SubStorage {
    pub name: String,
    pub path: PathBuf,
    pub media_types: Vec<MediaType>,
}

impl Storage {
    pub async fn add_user_avatar(&self, avatar_file: TempFile<'_>) -> ServiceResult<PathBuf> {
        self.user_avatars_storage
            .add_file(self.path.as_path(), avatar_file)
            .await
    }
    pub async fn get_user_avatar(&self, avatar_path: PathBuf) -> ServiceResult<NamedFile> {
        self.user_avatars_storage.get_file(avatar_path).await
    }
    pub fn new() -> Storage {
        let storage_path = env::current_dir().unwrap().join("storage");
        if !storage_path.exists() {
            fs::create_dir(&storage_path).unwrap();
        }
        let user_avatars_storage = SubStorage::new(
            String::from("user_avatars"),
            storage_path.as_path(),
            vec![MediaType::JPEG, MediaType::PNG],
        );
        Storage {
            path: storage_path,
            user_avatars_storage,
        }
    }
}

impl SubStorage {
    pub async fn add_file(
        &self,
        storage_path: &Path,
        mut file: TempFile<'_>,
    ) -> ServiceResult<PathBuf> {
        let name = match file.name() {
            Some(name) => name,
            None => return Err(AppError::InternalServerError),
        };
        let media_type = match file.content_type() {
            Some(ct) => ct.media_type(),
            None => {
                let expected = self.get_expected_extension();
                return Err(AppError::ValidationError(Box::new(move |locale| {
                    t!(
                        "invalid_file_type_expected_error",
                        locale = locale,
                        expected = &expected
                    )
                })));
            }
        };
        let extension = match media_type.extension() {
            Some(e) => e.as_str(),
            None => {
                let expected = self.get_expected_extension();
                return Err(AppError::ValidationError(Box::new(move |locale| {
                    t!(
                        "invalid_file_type_expected_error",
                        locale = locale,
                        expected = &expected
                    )
                })));
            }
        };
        if !self.media_types.contains(media_type) {
            let expected = self.get_expected_extension();
            let got = extension.to_owned();
            return Err(AppError::ValidationError(Box::new(move |locale| {
                t!(
                    "invalid_file_type_expected_error",
                    locale = locale,
                    expected = &expected,
                    got = &got
                )
            })));
        }
        let full_path = self
            .path
            .join(format!("{}_{}.{}", name, Uuid::new_v4(), extension));
        if let Err(_) = file.persist_to(&full_path).await {
            return Err(AppError::InternalServerError);
        }
        Ok(PathBuf::from(format!(
            "/{}",
            pathdiff::diff_paths(full_path, storage_path.parent().unwrap())
                .unwrap()
                .to_slash()
                .unwrap(),
        )))
    }
    pub async fn get_file(&self, path: PathBuf) -> ServiceResult<NamedFile> {
        match NamedFile::open(self.path.join(path)).await {
            Ok(named_file) => Ok(named_file),
            Err(_) => Err(AppError::NotFoundError(String::from(
                "file_not_found_error",
            ))),
        }
    }
    fn new(name: String, storage_path: &Path, media_types: Vec<MediaType>) -> SubStorage {
        let path = storage_path.join(&name);
        if !path.exists() {
            fs::create_dir(&path).unwrap();
        }
        SubStorage {
            name,
            path,
            media_types,
        }
    }
    fn get_expected_extension(&self) -> String {
        self.media_types
            .iter()
            .map(|m| m.extension().unwrap().as_str())
            .collect::<Vec<&str>>()
            .join(", ")
    }
}
