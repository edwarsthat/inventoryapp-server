# Inventario Server

API REST para sistema de inventario desarrollada en Rust con Axum y PostgreSQL.

## 🚀 Características

- ✅ API REST con Axum
- ✅ Autenticación JWT
- ✅ Base de datos PostgreSQL con SQLx
- ✅ Arquitectura en capas (Routes → Handlers → Services → Repositories)
- ✅ Manejo de errores personalizado
- ✅ Variables de entorno con dotenvy

## 📋 Requisitos

- Rust 1.70 o superior
- PostgreSQL 14 o superior
- Cargo

## 🛠️ Instalación

1. **Clonar el repositorio**
```bash
git clone https://github.com/edwarsthat/inventoryapp-server.git
cd inventario-server
```

2. **Configurar variables de entorno**

Crear archivo `.env` en la raíz del proyecto:

```env
DATABASE_URL=postgres://postgres:tu_contraseña@localhost:5432/inventarioapp
URL=localhost
PORT=8080
JWT_SECRET=tu_clave_secreta_super_segura
```

3. **Crear la base de datos**

```sql
CREATE DATABASE inventarioapp;

\c inventarioapp

CREATE TABLE usuarios (
    id_usuario          SERIAL PRIMARY KEY,
    nombre              VARCHAR(100) NOT NULL,
    apellido            VARCHAR(100) NOT NULL,
    correo              VARCHAR(150) UNIQUE NOT NULL,
    username            VARCHAR(100) UNIQUE NOT NULL,
    contrasena_hash     VARCHAR(255) NOT NULL,
    rol                 VARCHAR(50) NOT NULL,
    telefono            VARCHAR(30),
    fecha_creacion      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ultimo_acceso       TIMESTAMP,
    activo              BOOLEAN NOT NULL DEFAULT TRUE,
    debe_cambiar_contrasena BOOLEAN NOT NULL DEFAULT FALSE,
    fecha_ultimo_cambio TIMESTAMP
);
    fecha_ultimo_cambio TIMESTAMP
);

CREATE TABLE lista_productos (
    id UUID PRIMARY KEY,
    barcode VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL
);
```

4. **Compilar y ejecutar**

```bash
cargo build
cargo run
```

El servidor estará corriendo en `http://localhost:8080`

## 📁 Estructura del Proyecto

```
src/
├── config/           # Configuración (env, database)
├── controllers/      # Controladores HTTP
├── models/           # Modelos y DTOs
│   ├── errors/       # Errores personalizados
│   └── users.rs      # Modelo de usuario
├── repositories/     # Acceso a datos
├── routes/           # Definición de rutas
├── server/           # Configuración del servidor
├── services/         # Lógica de negocio
└── main.rs           # Punto de entrada
```

## 🔌 Endpoints

### Autenticación

#### Login
```http
POST /api/auth/login
Content-Type: application/json

{
  "username": "usuario",
  "password": "contraseña"
}
```

**Respuesta exitosa:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "message": "Login exitoso"
}
```

### Productos

#### Crear Producto
```http
POST /api/lista_productos
Content-Type: application/json
Authorization: Bearer <token>

{
  "barcode": "123456789",
  "name": "Producto Ejemplo"
}
```

**Respuesta exitosa:**
```json
{
  "status": "success",
  "message": "Producto creado exitosamente",
  "data": ""
}
```

### Health Check

```http
GET /health
```

**Respuesta:**
```
OK
```

## 🧪 Probar la API

### Con PowerShell

```powershell
Invoke-WebRequest -Uri "http://localhost:8080/api/auth/login" `
  -Method POST `
  -ContentType "application/json" `
  -Body '{"username":"test","password":"test123"}' | 
  Select-Object -ExpandProperty Content
```

### Con curl

```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"test","password":"test123"}'
```

## 🏗️ Arquitectura

El proyecto sigue una arquitectura en capas:

1. **Routes** - Define los endpoints HTTP
2. **Controllers/Handlers** - Maneja requests/responses
3. **Services** - Lógica de negocio
4. **Repositories** - Acceso a base de datos

**Flujo de datos:**
```
Request → Routes → Controllers → Services → Repositories → Database
                                      ↓
Response ← ← ← ← ← ← ← ← ← ← ← ← ← ← ←
```

## 🔒 Seguridad

- Contraseñas hasheadas con bcrypt
- Autenticación JWT
- Validación de datos en servicios
- Manejo de errores sin exponer información sensible

## 🛡️ Manejo de Errores

La API utiliza códigos HTTP estándar:

- `200 OK` - Solicitud exitosa
- `400 Bad Request` - Datos inválidos
- `401 Unauthorized` - Credenciales inválidas
- `404 Not Found` - Recurso no encontrado
- `500 Internal Server Error` - Error del servidor

## 📦 Dependencias Principales

- `axum` - Framework web
- `sqlx` - Driver de PostgreSQL async
- `tokio` - Runtime async
- `serde` - Serialización JSON
- `jsonwebtoken` - Manejo de JWT
- `bcrypt` - Hashing de contraseñas
- `dotenvy` - Variables de entorno

## 🚧 En Desarrollo

- [ ] Registro de usuarios
- [x] Gestión de inventario (Creación de productos)
- [ ] Roles y permisos
- [ ] Logs y auditoría
- [ ] Tests unitarios e integración

## 👤 Autor

**edwarsthat**
- GitHub: [@edwarsthat](https://github.com/edwarsthat)

## 📄 Licencia

Este proyecto es privado.

---

⚡ Desarrollado con Rust y Axum
