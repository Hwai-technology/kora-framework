//! Automatic OpenAPI 3.1.0 & Swagger UI Generator for Kora.
//! Inspects Kora AST (models, decorators, enums, controllers, interfaces, and routes)
//! and synthesizes standard OpenAPI specifications with zero manual configuration.

#![allow(clippy::collapsible_match, clippy::collapsible_if)]

use kora_ast::{ClassDecl, Expr, Item, ModelDecl, Module, Stmt, TypeExpr};
use serde_json::{Map, Value, json};

pub struct OpenApiDoc {
    pub spec: Value,
    pub endpoint_count: usize,
    pub schema_count: usize,
}

/// Generates an OpenAPI 3.1.0 JSON document by inspecting the entire Kora application AST.
pub fn generate_openapi(ast: &Module, app_name: &str) -> OpenApiDoc {
    let mut schemas = Map::new();
    let mut paths = Map::new();
    let mut schema_count = 0;
    let mut endpoint_count = 0;

    // 1. Auto-discover all models and entities
    for item in &ast.items {
        match item {
            Item::Model(m) => {
                let schema = model_to_schema(m);
                schemas.insert(m.name.name.clone(), schema);
                schema_count += 1;
            }
            Item::Struct(s) => {
                let mut props = Map::new();
                let mut req = Vec::new();
                for f in &s.fields {
                    props.insert(f.name.name.clone(), type_to_schema(&f.ty));
                    req.push(Value::String(f.name.name.clone()));
                }
                schemas.insert(
                    s.name.name.clone(),
                    json!({
                        "type": "object",
                        "properties": props,
                        "required": req
                    }),
                );
                schema_count += 1;
            }
            Item::Enum(en) => {
                let variants: Vec<Value> = en
                    .variants
                    .iter()
                    .map(|v| Value::String(v.name.name.clone()))
                    .collect();
                schemas.insert(
                    en.name.name.clone(),
                    json!({
                        "type": "string",
                        "enum": variants
                    }),
                );
                schema_count += 1;
            }
            _ => {}
        }
    }

    // Collect all controllers and their methods to correlate routes to controller return types
    let mut controllers: std::collections::HashMap<String, &ClassDecl> =
        std::collections::HashMap::new();
    for item in &ast.items {
        if let Item::Class(cls) = item {
            controllers.insert(cls.name.name.clone(), cls);
        }
    }

    // 2. Auto-discover all routes (get, post, put, delete, patch)
    for item in &ast.items {
        if let Item::Stmt(Stmt::Expr { expr, .. }) = item {
            if let Expr::Call { callee, args, .. } = expr {
                if let Expr::Ident(id) = callee.as_ref() {
                    let method = id.name.to_lowercase();
                    if matches!(method.as_str(), "get" | "post" | "put" | "delete" | "patch") {
                        if let Some(path_expr) = args.first() {
                            let raw_path = match path_expr {
                                Expr::StringLit { value, .. } => value.clone(),
                                _ => "/api".to_string(),
                            };

                            let (normalized_path, path_params) = normalize_route_path(&raw_path);
                            let tag = derive_route_tag(&normalized_path);

                            let closure_expr = args.get(1);
                            let mut op_obj = Map::new();
                            op_obj.insert("tags".into(), json!([tag]));
                            op_obj.insert(
                                "summary".into(),
                                Value::String(format!(
                                    "{} {}",
                                    method.to_uppercase(),
                                    normalized_path
                                )),
                            );

                            // Path parameters
                            if !path_params.is_empty() {
                                let mut params_arr = Vec::new();
                                for p in &path_params {
                                    params_arr.push(json!({
                                        "name": p,
                                        "in": "path",
                                        "required": true,
                                        "schema": {
                                            "type": if p.ends_with("id") || p.ends_with("Id") { "integer" } else { "string" },
                                            "format": if p.ends_with("id") || p.ends_with("Id") { "int64" } else { "" }
                                        }
                                    }));
                                }
                                op_obj.insert("parameters".into(), Value::Array(params_arr));
                            }

                            // Correlate handler and Request Body / Responses
                            let (req_body, resp_schema) =
                                correlate_handler(closure_expr, &controllers, &schemas, &method);

                            if let Some(rb) = req_body {
                                op_obj.insert("requestBody".into(), rb);
                            }

                            let success_code = if method == "post" { "201" } else { "200" };
                            let mut responses = Map::new();
                            responses.insert(
                                success_code.into(),
                                json!({
                                    "description": "Successful operation",
                                    "content": {
                                        "application/json": {
                                            "schema": resp_schema
                                        }
                                    }
                                }),
                            );
                            if !path_params.is_empty() {
                                responses.insert(
                                    "404".into(),
                                    json!({ "description": "Resource not found" }),
                                );
                            }
                            responses.insert(
                                "400".into(),
                                json!({ "description": "Invalid input / bad request" }),
                            );
                            responses.insert(
                                "500".into(),
                                json!({ "description": "Internal server error" }),
                            );

                            op_obj.insert("responses".into(), Value::Object(responses));

                            let path_entry = paths
                                .entry(normalized_path.clone())
                                .or_insert_with(|| Value::Object(Map::new()));
                            if let Value::Object(p_map) = path_entry {
                                p_map.insert(method, Value::Object(op_obj));
                                endpoint_count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    let spec = json!({
        "openapi": "3.1.0",
        "info": {
            "title": format!("{app_name} API"),
            "version": "1.0.0",
            "description": "Enterprise API Documentation automatically generated by the Kora Language Compiler & Architecture Engine."
        },
        "servers": [
            {
                "url": "http://127.0.0.1:7582",
                "description": "Local Development Server"
            }
        ],
        "paths": paths,
        "components": {
            "schemas": schemas
        }
    });

    OpenApiDoc {
        spec,
        endpoint_count,
        schema_count,
    }
}

fn model_to_schema(m: &ModelDecl) -> Value {
    let mut props = Map::new();
    let mut required = Vec::new();
    let mut is_entity = false;
    let mut table_name = None;

    for attr in &m.attributes {
        if attr.name.name == "entity" {
            is_entity = true;
        } else if attr.name.name == "table" {
            if let Some(Expr::StringLit { value, .. }) = attr.args.first() {
                table_name = Some(value.clone());
            }
        }
    }

    for f in &m.fields {
        let (field_schema, is_req) = field_to_schema(f);
        props.insert(f.name.name.clone(), field_schema);
        if is_req {
            required.push(Value::String(f.name.name.clone()));
        }
    }

    let mut schema = json!({
        "type": "object",
        "properties": props
    });

    if !required.is_empty() {
        schema
            .as_object_mut()
            .unwrap()
            .insert("required".into(), Value::Array(required));
    }
    if is_entity {
        schema
            .as_object_mut()
            .unwrap()
            .insert("x-entity".into(), Value::Bool(true));
    }
    if let Some(tbl) = table_name {
        schema
            .as_object_mut()
            .unwrap()
            .insert("x-table".into(), Value::String(tbl));
    }

    schema
}

fn field_to_schema(f: &kora_ast::FieldDecl) -> (Value, bool) {
    let mut schema = type_to_schema(&f.ty);
    let mut is_required = false;

    // Check if type is Option
    let is_optional = matches!(&f.ty, TypeExpr::Named { name, .. } if name.name == "Option");

    for attr in &f.attributes {
        match attr.name.name.as_str() {
            "required" | "primary" => is_required = true,
            "autoIncrement" => {
                schema
                    .as_object_mut()
                    .unwrap()
                    .insert("readOnly".into(), Value::Bool(true));
            }
            "email" => {
                schema
                    .as_object_mut()
                    .unwrap()
                    .insert("format".into(), Value::String("email".into()));
            }
            "unique" => {
                schema.as_object_mut().unwrap().insert(
                    "description".into(),
                    Value::String("Unique field constraint".into()),
                );
            }
            "min" => {
                if let Some(Expr::IntLit { value, .. }) = attr.args.first() {
                    schema
                        .as_object_mut()
                        .unwrap()
                        .insert("minimum".into(), json!(value));
                }
            }
            "max" => {
                if let Some(Expr::IntLit { value, .. }) = attr.args.first() {
                    schema
                        .as_object_mut()
                        .unwrap()
                        .insert("maximum".into(), json!(value));
                }
            }
            "default" => {
                if let Some(arg) = attr.args.first() {
                    match arg {
                        Expr::IntLit { value, .. } => {
                            schema
                                .as_object_mut()
                                .unwrap()
                                .insert("default".into(), json!(value));
                        }
                        Expr::StringLit { value, .. } => {
                            schema
                                .as_object_mut()
                                .unwrap()
                                .insert("default".into(), json!(value));
                        }
                        Expr::BoolLit { value, .. } => {
                            schema
                                .as_object_mut()
                                .unwrap()
                                .insert("default".into(), json!(value));
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    if !is_optional && !is_required {
        // By default non-optional fields in models are required
        is_required = true;
    }

    (schema, is_required)
}

fn type_to_schema(ty: &TypeExpr) -> Value {
    match ty {
        TypeExpr::Named { name, args, .. } => match name.name.as_str() {
            "Int" => json!({ "type": "integer", "format": "int64" }),
            "Float" => json!({ "type": "number", "format": "double" }),
            "String" => json!({ "type": "string" }),
            "Bool" => json!({ "type": "boolean" }),
            "Unit" => json!({ "type": "null" }),
            "List" => {
                let inner = args
                    .first()
                    .map(type_to_schema)
                    .unwrap_or_else(|| json!({ "type": "object" }));
                json!({ "type": "array", "items": inner })
            }
            "Option" => {
                let mut inner = args
                    .first()
                    .map(type_to_schema)
                    .unwrap_or_else(|| json!({ "type": "object" }));
                if let Some(obj) = inner.as_object_mut() {
                    obj.insert("nullable".into(), Value::Bool(true));
                }
                inner
            }
            custom => json!({ "$ref": format!("#/components/schemas/{custom}") }),
        },
        TypeExpr::Function { .. } => {
            json!({ "type": "string", "description": "Function callback" })
        }
    }
}

/// Converts path parameters like `:id` into `{id}`
fn normalize_route_path(path: &str) -> (String, Vec<String>) {
    let mut normalized = String::new();
    let mut params = Vec::new();

    for segment in path.split('/') {
        if segment.is_empty() {
            continue;
        }
        normalized.push('/');
        if let Some(stripped) = segment.strip_prefix(':') {
            normalized.push('{');
            normalized.push_str(stripped);
            normalized.push('}');
            params.push(stripped.to_string());
        } else if segment.starts_with('{') && segment.ends_with('}') {
            let p = &segment[1..segment.len() - 1];
            normalized.push_str(segment);
            params.push(p.to_string());
        } else {
            normalized.push_str(segment);
        }
    }

    if normalized.is_empty() {
        normalized = "/".to_string();
    }

    (normalized, params)
}

fn derive_route_tag(path: &str) -> String {
    let segments: Vec<&str> = path
        .split('/')
        .filter(|s| !s.is_empty() && *s != "api" && *s != "v1" && *s != "v2")
        .collect();

    if let Some(first) = segments.first() {
        let clean = first.trim_matches('{').trim_matches('}');
        let mut chars = clean.chars();
        match chars.next() {
            Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
            None => "General".to_string(),
        }
    } else {
        "General".to_string()
    }
}

fn correlate_handler(
    closure_expr: Option<&Expr>,
    controllers: &std::collections::HashMap<String, &ClassDecl>,
    schemas: &Map<String, Value>,
    method: &str,
) -> (Option<Value>, Value) {
    let mut request_body = None;
    let mut response_schema = json!({ "type": "object" });

    if let Some(Expr::Closure { params, body, .. }) = closure_expr {
        // 1. Inspect parameters for DTOs
        for p in params {
            if let Some(TypeExpr::Named { name, .. }) = &p.ty {
                if schemas.contains_key(&name.name)
                    && (name.name.contains("Dto")
                        || name.name.contains("Input")
                        || name.name.contains("Request"))
                {
                    request_body = Some(json!({
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": { "$ref": format!("#/components/schemas/{}", name.name) }
                            }
                        }
                    }));
                }
            }
        }

        // 2. Correlate with controller methods called in body
        let mut called_method_name = None;
        let mut target_controller = None;

        match body {
            kora_ast::ClosureBody::Expr(e) => {
                extract_call_info(e, &mut target_controller, &mut called_method_name);
            }
            kora_ast::ClosureBody::Block(b) => {
                for st in &b.stmts {
                    if let Stmt::Return { value: Some(e), .. } | Stmt::Expr { expr: e, .. } = st {
                        extract_call_info(e, &mut target_controller, &mut called_method_name);
                    }
                }
            }
        }

        // Find matching controller method signature
        if let Some(m_name) = called_method_name {
            for cls in controllers.values() {
                for m in &cls.methods {
                    if m.sig.name.name == m_name {
                        if let Some(ret_ty) = &m.sig.return_ty {
                            response_schema = type_to_schema(ret_ty);
                        }
                        if request_body.is_none() {
                            for p in &m.sig.params {
                                if let TypeExpr::Named { name, .. } = &p.ty {
                                    if schemas.contains_key(&name.name)
                                        && (name.name.contains("Dto")
                                            || name.name.contains("Input"))
                                    {
                                        request_body = Some(json!({
                                            "required": true,
                                            "content": {
                                                "application/json": {
                                                    "schema": { "$ref": format!("#/components/schemas/{}", name.name) }
                                                }
                                            }
                                        }));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Default DTO matching if still none found
    if request_body.is_none() && matches!(method, "post" | "put" | "patch") {
        for s_name in schemas.keys() {
            if s_name.starts_with("Create") || s_name.starts_with("Update") {
                request_body = Some(json!({
                    "required": true,
                    "content": {
                        "application/json": {
                            "schema": { "$ref": format!("#/components/schemas/{s_name}") }
                        }
                    }
                }));
                break;
            }
        }
    }

    if response_schema == json!({ "type": "object" }) {
        for s_name in schemas.keys() {
            if s_name.ends_with("ResponseDto") || s_name.ends_with("Response") {
                if method == "get" {
                    response_schema = json!({
                        "type": "array",
                        "items": { "$ref": format!("#/components/schemas/{s_name}") }
                    });
                } else {
                    response_schema = json!({ "$ref": format!("#/components/schemas/{s_name}") });
                }
                break;
            }
        }
    }

    (request_body, response_schema)
}

fn extract_call_info(expr: &Expr, _ctrl: &mut Option<String>, method_name: &mut Option<String>) {
    if let Expr::Call { callee, .. } = expr {
        if let Expr::FieldAccess { field, .. } = callee.as_ref() {
            *method_name = Some(field.name.clone());
        }
    }
}

/// Generates a standalone, beautiful HTML document embedding Swagger UI.
pub fn generate_swagger_ui_html(spec_json: &Value, title: &str) -> String {
    let spec_str = serde_json::to_string(spec_json).unwrap_or_else(|_| "{}".to_string());
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>{title} — Interactive Swagger UI & OpenAPI 3.1</title>
  <meta name="description" content="Explore and test the {title} API endpoints, data transfer objects, and CRUD operations with automated OpenAPI 3.1.0 interactive documentation." />
  <meta name="keywords" content="kora, api, openapi, swagger, rest, {title}, clean-architecture, microservices, backend" />
  <meta name="robots" content="index, follow" />
  <link rel="canonical" href="http://127.0.0.1:7582/docs" />

  <!-- OpenGraph Metadata -->
  <meta property="og:title" content="{title} — Interactive API Documentation" />
  <meta property="og:description" content="Explore and test the {title} RESTful endpoints with automated OpenAPI 3.1.0 specifications." />
  <meta property="og:type" content="website" />
  <meta property="og:url" content="http://127.0.0.1:7582/docs" />
  <meta property="og:site_name" content="Kora Framework" />

  <!-- Twitter Card Metadata -->
  <meta name="twitter:card" content="summary_large_image" />
  <meta name="twitter:title" content="{title} — Interactive API Documentation" />
  <meta name="twitter:description" content="Explore and test the {title} RESTful endpoints with automated OpenAPI 3.1.0 specifications." />

  <!-- Geographic Location Metadata -->
  <meta name="geo.region" content="US-CA" />
  <meta name="geo.placename" content="San Francisco, California" />
  <meta name="geo.position" content="37.7749;-122.4194" />
  <meta name="ICBM" content="37.7749, -122.4194" />

  <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5/swagger-ui.css" />
  <link rel="icon" type="image/png" href="https://unpkg.com/swagger-ui-dist@5/favicon-32x32.png" />
  <style>
    html {{ box-sizing: border-box; overflow: -moz-scrollbars-vertical; overflow-y: scroll; }}
    *, *:before, *:after {{ box-sizing: inherit; }}
    body {{ margin: 0; background: #fafafa; font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif; }}
    .topbar {{ display: none !important; }}
    .kora-header {{
        background: linear-gradient(135deg, #1e1e2e 0%, #11111b 100%);
        color: #cdd6f4;
        padding: 16px 24px;
        display: flex;
        align-items: center;
        justify-content: space-between;
        border-bottom: 2px solid #89b4fa;
        box-shadow: 0 4px 12px rgba(0,0,0,0.15);
    }}
    .kora-header h1 {{ margin: 0; font-size: 20px; font-weight: 700; color: #89b4fa; display: flex; align-items: center; gap: 8px; }}
    .kora-badge {{
        background: #313244;
        color: #a6adc8;
        font-size: 12px;
        font-weight: 600;
        padding: 4px 10px;
        border-radius: 999px;
        border: 1px solid #45475a;
    }}
  </style>
</head>
<body>
  <div class="kora-header">
    <h1>⚡ {title}</h1>
    <span class="kora-badge">OpenAPI 3.1.0 · Auto-Generated by Kora Engine</span>
  </div>
  <div id="swagger-ui"></div>
  <script src="https://unpkg.com/swagger-ui-dist@5/swagger-ui-bundle.js"></script>
  <script src="https://unpkg.com/swagger-ui-dist@5/swagger-ui-standalone-preset.js"></script>
  <script>
    window.onload = () => {{
      window.ui = SwaggerUIBundle({{
        spec: {spec_str},
        dom_id: '#swagger-ui',
        deepLinking: true,
        presets: [
          SwaggerUIBundle.presets.apis,
          SwaggerUIStandalonePreset
        ],
        plugins: [
          SwaggerUIBundle.plugins.DownloadUrl
        ],
        layout: "StandaloneLayout"
      }});
    }};
  </script>
</body>
</html>"#
    )
}
