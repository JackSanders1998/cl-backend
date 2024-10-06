From Source:
- Run project locally in watch mode (exclude bindings changes): `cargo watch -qcx 'shuttle run' --ignore '**/cl-bindings/**'`
- Deploy project to shuttle: `cargo shuttle deploy`
- Prepare sqlx: `cargo sqlx prepare`
- Add sqlx migration: `sqlx migrate add <migration-name>`

From cl-bindings (`cl-bindings`)
- Generate openapi bindings: `npm run generate`
- Publish openapi bindings patch: `npm version patch && npm publish --access=public`
