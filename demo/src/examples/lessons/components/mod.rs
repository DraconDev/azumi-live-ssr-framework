pub mod auth_infra;
pub mod layout;
pub mod lesson_nav;

// Note: No shared LessonPage component. Each lesson is intentionally
// self-contained so new acolytes can read ONE file and understand it.
// The CSS duplication (~50 lines per lesson) is an acceptable tradeoff
// for learnability. A shared component would require complex props
// (title, subtitle, gradient, key_points, examples, nav) and make
// each lesson harder to follow.
