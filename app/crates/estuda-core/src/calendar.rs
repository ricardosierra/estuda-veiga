use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::canvas::Course;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEvent {
    pub id: String,
    pub semester: String,
    pub course_id: Option<String>,
    pub course_title: Option<String>,
    pub title: String,
    pub starts_at: String,
    pub all_day: bool,
    pub source_kind: String,
    pub source_url: String,
    pub source_label: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CanvasAssignment {
    pub id: u64,
    pub name: String,
    pub due_at: Option<String>,
    pub html_url: Option<String>,
    pub published: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarSyncReport {
    pub courses_checked: usize,
    pub events_saved: usize,
    pub errors: Vec<String>,
}

pub fn valid_semester(value: &str) -> bool {
    let Some((year, term)) = value.split_once('.') else {
        return false;
    };
    year.len() == 4 && year.bytes().all(|byte| byte.is_ascii_digit()) && matches!(term, "1" | "2")
}

pub fn assignment_events(
    course: &Course,
    semester: &str,
    assignments: Vec<CanvasAssignment>,
) -> Vec<CalendarEvent> {
    assignments
        .into_iter()
        .filter_map(|assignment| {
            if !assignment.published || assignment.name.trim().is_empty() {
                return None;
            }
            let starts_at = assignment.due_at.filter(|value| valid_timestamp(value))?;
            let all_day = !starts_at.contains('T');
            let source_url = assignment
                .html_url
                .filter(|value| valid_canvas_assignment_url(value, &course.id))
                .and_then(|value| url::Url::parse(&value).ok())
                .map(|mut url| {
                    url.set_query(None);
                    url.set_fragment(None);
                    url.to_string()
                })
                .unwrap_or_else(|| {
                    format!(
                        "{}/assignments/{}",
                        course.url.trim_end_matches('/'),
                        assignment.id
                    )
                });
            let identity = format!("canvas:{}:{}", course.id, assignment.id);
            let id = format!("{:x}", Sha256::digest(identity.as_bytes()));
            Some(CalendarEvent {
                id,
                semester: semester.to_owned(),
                course_id: Some(course.id.clone()),
                course_title: Some(course.title.clone()),
                title: assignment.name.trim().to_owned(),
                starts_at,
                all_day,
                source_kind: "canvas_assignment".into(),
                source_url,
                source_label: format!("Canvas · {}", course.title),
            })
        })
        .collect()
}

fn valid_timestamp(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() < 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return false;
    }
    let (Some(year), Some(month), Some(day)) = (
        digits(&bytes[0..4]),
        digits(&bytes[5..7]),
        digits(&bytes[8..10]),
    ) else {
        return false;
    };
    let leap_year = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let month_days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap_year => 29,
        2 => 28,
        _ => return false,
    };
    if day == 0 || day > month_days {
        return false;
    }
    if bytes.len() == 10 {
        return true;
    }
    if bytes.get(10) != Some(&b'T')
        || bytes.len() < 20
        || bytes.get(13) != Some(&b':')
        || bytes.get(16) != Some(&b':')
    {
        return false;
    }
    let (Some(hour), Some(minute), Some(second)) = (
        digits(&bytes[11..13]),
        digits(&bytes[14..16]),
        digits(&bytes[17..19]),
    ) else {
        return false;
    };
    if hour > 23 || minute > 59 || second > 60 {
        return false;
    }
    let mut position = 19;
    if bytes.get(position) == Some(&b'.') {
        position += 1;
        let start = position;
        while bytes.get(position).is_some_and(u8::is_ascii_digit) {
            position += 1;
        }
        if position == start {
            return false;
        }
    }
    match bytes.get(position..) {
        Some([b'Z']) => true,
        Some(
            [
                sign @ (b'+' | b'-'),
                tz_hour @ b'0'..=b'2',
                tz_hour2 @ b'0'..=b'9',
                b':',
                tz_minute @ b'0'..=b'5',
                tz_minute2 @ b'0'..=b'9',
            ],
        ) => {
            let _ = sign;
            let offset_hour = digits(&[*tz_hour, *tz_hour2]).unwrap_or(u32::MAX);
            let offset_minute = digits(&[*tz_minute, *tz_minute2]).unwrap_or(u32::MAX);
            offset_hour <= 23 && offset_minute <= 59
        }
        _ => false,
    }
}

fn digits(value: &[u8]) -> Option<u32> {
    value.iter().try_fold(0, |number, byte| {
        byte.is_ascii_digit()
            .then_some(number * 10 + u32::from(byte - b'0'))
    })
}

fn valid_canvas_assignment_url(value: &str, course_id: &str) -> bool {
    let Ok(url) = url::Url::parse(value) else {
        return false;
    };
    let Some(assignment_id) = url
        .path()
        .strip_prefix(&format!("/courses/{course_id}/assignments/"))
    else {
        return false;
    };
    url.scheme() == "https"
        && url.host_str() == Some("uva.instructure.com")
        && url.username().is_empty()
        && url.password().is_none()
        && url.port().is_none()
        && !assignment_id.is_empty()
        && assignment_id.bytes().all(|byte| byte.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::CourseKind;

    fn course() -> Course {
        Course {
            id: "8765".into(),
            title: "Desenvolvimento Web".into(),
            url: "https://uva.instructure.com/courses/8765".into(),
            kind: CourseKind::Current,
        }
    }

    #[test]
    fn semester_and_due_date_validation_are_strict() {
        assert!(valid_semester("2027.1"));
        assert!(valid_semester("2026.2"));
        assert!(!valid_semester("2026.3"));
        assert!(!valid_semester("26.1"));
        assert!(valid_timestamp("2028-02-29"));
        assert!(valid_timestamp("2026-10-09T13:30:00-03:00"));
        assert!(!valid_timestamp("2027-02-29"));
        assert!(!valid_timestamp("2026-13-40"));
        assert!(!valid_timestamp("2026-10-09T27:90:80Z"));
        assert!(!valid_timestamp("2026-10-09T10:00"));
    }

    #[test]
    fn calendar_includes_published_deadlines_and_sanitizes_source_urls() {
        let events = assignment_events(
            &course(),
            "2026.2",
            vec![
                CanvasAssignment {
                    id: 20,
                    name: "  Projeto final  ".into(),
                    due_at: Some("2026-12-10T23:59:00-03:00".into()),
                    html_url: Some(
                        "https://uva.instructure.com/courses/8765/assignments/20?token=private"
                            .into(),
                    ),
                    published: true,
                },
                CanvasAssignment {
                    id: 21,
                    name: "Não publicado".into(),
                    due_at: Some("2026-12-11T23:59:00-03:00".into()),
                    html_url: None,
                    published: false,
                },
                CanvasAssignment {
                    id: 22,
                    name: "Data inválida".into(),
                    due_at: Some("2026-13-40".into()),
                    html_url: None,
                    published: true,
                },
            ],
        );
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].semester, "2026.2");
        assert_eq!(events[0].title, "Projeto final");
        assert_eq!(
            events[0].source_url,
            "https://uva.instructure.com/courses/8765/assignments/20"
        );
        assert!(!events[0].source_url.contains("private"));
    }
}
