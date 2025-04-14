# Secure Edge Project
- A starter main.rs with app setup + TLS
- The LDAP login route and JWT auth setup
- The /video/live simulation (MJPEG or refresh)
- HTML template(s) for video + status

## A realistic simulation of a secure, TLS-enabled video edge device with:

- LDAP-based login
- Secure REST API (JSON over HTTPS)
- PostgreSQL integration via SeaORM
- HTML frontend to view video and status
- Video stream simulation (/video/live)
- Authenticated user session (JWT)

🚧 Project Features (Complete Scope)
- Feature	Stack/Tooling
- Web backend	Rust, Axum, Tower
- TLS	Rustls
- Auth	LDAP + JWT
- DB	PostgreSQL + SeaORM
- API	JSON (Axum)
- Video stream	MJPEG or auto-refreshing image
- HTML Frontend	Static served with Axum
- Project structure	Modular: routes, models, templates, config