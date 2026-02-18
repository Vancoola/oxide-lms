# Changelog
All notable changes to Oxide-LMS will be documented in this file.

## [0.1.1] - 2026-02-18

### Added
- **Config**: All settings are made in App.toml and uploaded to AppConfig. all secrets are stored in .env and uploaded to AppConfig.
- **Error**: The ApiError was removed from the AppError and linked to the Http Status code
- **Branding**: Added a project logo
- **Share model**: The first Share structure is between the Back and Front.
- **Admin**: Creating an Admin account from .env
- **Auth/JWT**: A fully working version of the login via JWT
- **AuthProvider**: The AuthProvider has been redesigned and authorization bugs on the client have been fixed