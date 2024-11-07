#### In my opinion, this structure is very good for a small project, but it can be improved for a larger project.jk
## This Projectstructure is based on the following structure:
```
./
├── Cargo.lock
├── Cargo.toml
├── readme.md
└── src
    ├── config
    │   ├── config.rs
    │   └── mod.rs
    ├── controllers
    │   ├── auth_controller
    │   │   ├── login.rs
    │   │   ├── mod.rs
    │   │   └── register.rs
    │   ├── dto
    │   │   ├── auth.rs
    │   │   ├── mod.rs
    │   │   └── user.rs
    │   ├── health_check.rs
    │   ├── mod.rs
    │   └── user_controller
    │       ├── get_user.rs
    │       ├── mod.rs
    │       └── update_user.rs
    ├── domain
    │   ├── app_state.rs
    │   ├── error.rs
    │   ├── mod.rs
    │   ├── models
    │   │   ├── auth.rs
    │   │   ├── jwt.rs
    │   │   ├── mod.rs
    │   │   └── user.rs
    │   ├── repositories
    │   │   ├── mod.rs
    │   │   └── user.rs
    │   └── services
    │       ├── auth.rs
    │       ├── mod.rs
    │       └── user.rs
    ├── extractor
    │   ├── mod.rs
    │   └── user.rs
    ├── infrastructure
    │   ├── mod.rs
    │   ├── models
    │   │   ├── mod.rs
    │   │   └── user.rs
    │   └── repositories
    │       ├── mod.rs
    │       └── user.rs
    ├── lib.rs
    ├── main.rs
    ├── middleware
    │   ├── jwt.rs
    │   └── mod.rs
    ├── services
    │   ├── auth.rs
    │   ├── mod.rs
    │   └── user.rs
    └── utils
        ├── hash_password.rs
        ├── jwt.rs
        └── mod.rs
```

