use askama::Template;
use axum::{
    Form,
    extract::{Path, Query, State},
    http::HeaderMap,
    response::{Html, Redirect},
};

use crate::models::{AppState, CreateTask, DashboardStats, Task, TaskFilter};

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    active_page: String,
    stats: DashboardStats,
    recent_tasks: Vec<Task>,
}

#[derive(Template)]
#[template(path = "tasks.html")]
struct TasksTemplate {
    active_page: String,
    tasks: Vec<Task>,
    search: String,
    status: String,
    priority: String,
}

#[derive(Template)]
#[template(path = "tasks_table_body.html")]
struct TasksTableBodyTemplate {
    tasks: Vec<Task>,
}

#[derive(Template)]
#[template(path = "tasks_create.html")]
struct TasksCreateTemplate {
    active_page: String,
}

#[derive(Template)]
#[template(path = "tasks_edit.html")]
struct TasksEditTemplate {
    active_page: String,
    task: Task,
}

pub async fn dashboard_handler(State(state): State<AppState>) -> Html<String> {
    let stats = sqlx::query_as::<_, (i64, i64, i64, i64)>(
        "SELECT
            COUNT(*) as total,
            SUM(CASE WHEN status = 'InProgress' THEN 1 ELSE 0 END) as in_progress,
            SUM(CASE WHEN status = 'Done' THEN 1 ELSE 0 END) as done,
            SUM(CASE WHEN status = 'Todo' THEN 1 ELSE 0 END) as todo
         FROM tasks",
    )
    .fetch_one(&state.db)
    .await
    .unwrap();

    let stats = DashboardStats {
        total: stats.0,
        in_progress: stats.1,
        done: stats.2,
        todo: stats.3,
    };

    let recent_tasks =
        sqlx::query_as::<_, Task>("SELECT * FROM tasks ORDER BY created_at DESC LIMIT 5")
            .fetch_all(&state.db)
            .await
            .unwrap();

    let template = DashboardTemplate {
        active_page: "dashboard".to_string(),
        stats,
        recent_tasks,
    };
    Html(template.render().unwrap())
}

pub async fn tasks_handler(
    State(state): State<AppState>,
    Query(filter): Query<TaskFilter>,
    headers: HeaderMap,
) -> Html<String> {
    let mut sql = String::from("SELECT * FROM tasks WHERE 1=1");
    let mut binds: Vec<String> = Vec::new();

    if let Some(ref search) = filter.search
        && !search.is_empty()
    {
        sql.push_str(" AND name LIKE ?");
        binds.push(format!("%{search}%"));
    }
    if let Some(ref status) = filter.status
        && !status.is_empty()
    {
        sql.push_str(" AND status = ?");
        binds.push(status.clone());
    }
    if let Some(ref priority) = filter.priority
        && !priority.is_empty()
    {
        sql.push_str(" AND priority = ?");
        binds.push(priority.clone());
    }
    sql.push_str(" ORDER BY created_at DESC");

    let mut query = sqlx::query_as::<_, Task>(&sql);
    for bind in &binds {
        query = query.bind(bind);
    }
    let tasks = query.fetch_all(&state.db).await.unwrap();

    if headers.get("HX-Request").is_some() {
        let template = TasksTableBodyTemplate { tasks };
        Html(template.render().unwrap())
    } else {
        let template = TasksTemplate {
            active_page: "tasks".to_string(),
            tasks,
            search: filter.search.unwrap_or_default(),
            status: filter.status.unwrap_or_default(),
            priority: filter.priority.unwrap_or_default(),
        };
        Html(template.render().unwrap())
    }
}

pub async fn tasks_create_handler() -> Html<String> {
    let template = TasksCreateTemplate {
        active_page: "tasks".to_string(),
    };
    Html(template.render().unwrap())
}

pub async fn tasks_create_action(
    State(state): State<AppState>,
    Form(input): Form<CreateTask>,
) -> Redirect {
    sqlx::query("INSERT INTO tasks (name, status, priority) VALUES (?, ?, ?)")
        .bind(&input.name)
        .bind(&input.status)
        .bind(&input.priority)
        .execute(&state.db)
        .await
        .unwrap();

    Redirect::to("/tasks")
}

pub async fn tasks_edit_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Html<String> {
    let task = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
        .bind(id)
        .fetch_one(&state.db)
        .await
        .unwrap();

    let template = TasksEditTemplate {
        active_page: "tasks".to_string(),
        task,
    };
    Html(template.render().unwrap())
}

pub async fn tasks_update_action(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(input): Form<CreateTask>,
) -> Redirect {
    sqlx::query("UPDATE tasks SET name = ?, status = ?, priority = ?, updated_at = datetime('now') WHERE id = ?")
        .bind(&input.name)
        .bind(&input.status)
        .bind(&input.priority)
        .bind(id)
        .execute(&state.db)
        .await
        .unwrap();

    Redirect::to("/tasks")
}

pub async fn tasks_delete_action(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Html<String> {
    sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await
        .unwrap();

    Html(String::new())
}
