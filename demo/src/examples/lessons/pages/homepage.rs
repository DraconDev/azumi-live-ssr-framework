use crate::examples::lessons::components::layout::DarkModernLayout;
use azumi::prelude::*;

#[azumi::page(route = "/")]
#[azumi::component]
pub fn homepage() -> impl Component {
    html! {
        @DarkModernLayout() {
            @Lessons()
        }
    }
}

#[azumi::component]
pub fn Lessons() -> impl Component {
    html! {
        <div class={page_wrapper}>
            <div class={bg_glow}></div>

            // Hero Section
            <header class={hero}>
                <div class={hero_badge}>"🚀 Server-Rendered HTML with Client Interactivity"</div>
                <h1 class={hero_title}>"Master Azumi"</h1>
                <p class={hero_subtitle}>
                    "All Rust. Zero custom JavaScript. No ecosystem churn.\n                    From basic components to production-ready applications."
                </p>

                <div class={hero_actions}>
                    <a href="/lesson-0" class={btn_primary}>"Start Learning"</a>
                    <a href="https://github.com/azumi-rs/azumi" class={btn_secondary}>"View on GitHub"</a>
                </div>

                <div class={hero_stats}>
                    <div class={stat}>
                        <span class={stat_number}>"20"</span>
                        <span class={stat_label}>"Lessons"</span>
                    </div>
                    <div class={stat_divider}></div>
                    <div class={stat}>
                        <span class={stat_number}>"∞"</span>
                        <span class={stat_label}>"Possibilities"</span>
                    </div>
                    <div class={stat_divider}></div>
                    <div class={stat}>
                        <span class={stat_number}>"0"</span>
                        <span class={stat_label}>"JS Required"</span>
                    </div>
                </div>
            </header>

            // Course Sections
            <section class={section}>
                <h2 class={section_title}>"🌱 Foundations"</h2>
                <div class={grid}>
                    @LessonCard(num="00", title="Introduction", desc="Understanding the component structure.", link="/lesson-0", accent="#10b981")
                    @LessonCard(num="01", title="Components", desc="Building your first reusable blocks.", link="/lesson-1", accent="#10b981")
                    @LessonCard(num="02", title="CSS Scoping", desc="How styles are isolated safely.", link="/lesson-2", accent="#10b981")
                    @LessonCard(num="03", title="Global Styles", desc="Managing global vs local CSS.", link="/lesson-3", accent="#10b981")
                </div>
            </section>

            <section class={section}>
                <h2 class={section_title}>"🏗️ Structure & Forms"</h2>
                <div class={grid}>
                    @LessonCard(num="04", title="HTML Structure", desc="Compile-time HTML validation.", link="/lesson-4", accent="#3b82f6")
                    @LessonCard(num="05", title="Accessibility", desc="Building inclusive interfaces.", link="/lesson-5", accent="#3b82f6")
                    @LessonCard(num="06", title="Basic Forms", desc="Standard form handling patterns.", link="/lesson-6", accent="#3b82f6")
                    @LessonCard(num="07", title="Event Handling", desc="Interactivity with event listeners.", link="/lesson-7", accent="#3b82f6")
                </div>
            </section>

            <section class={section}>
                <h2 class={section_title}>"⚡ Advanced Patterns"</h2>
                <div class={grid}>
                    @LessonCard(num="08", title="State Management", desc="Managing complex component state.", link="/lesson-8", accent="#8b5cf6")
                    @LessonCard(num="09", title="Advanced Patterns", desc="Composition and slots.", link="/lesson-9", accent="#8b5cf6")
                    @LessonCard(num="10", title="Performance", desc="Optimizing rendering and updates.", link="/lesson-10", accent="#8b5cf6")
                    @LessonCard(num="11", title="Async Patterns", desc="Loading states and error handling.", link="/lesson-11", accent="#8b5cf6")
                </div>
            </section>

            <section class={section}>
                <h2 class={section_title}>"🔥 Production Ready"</h2>
                <div class={grid}>
                    @LessonCard(num="12", title="Image Optimization", desc="Automatic lazy loading & attributes.", link="/lesson-12", accent="#f59e0b")
                    @LessonCard(num="13", title="Live Forms", desc="Real-time validation and feedback.", link="/lesson-13", accent="#f59e0b")
                    @LessonCard(num="14", title="Composition", desc="Building complex Live UIs.", link="/lesson-14", accent="#f59e0b")
                    @LessonCard(num="15", title="Full Application", desc="Interactive Todo App (In-Memory).", link="/lesson-15-sql-basics", accent="#f59e0b")
                </div>
            </section>

            <section class={section}>
                <h2 class={section_title}>"🛡️ Enterprise Features"</h2>
                <div class={grid}>
                    @LessonCard(num="16", title="Async Database", desc="SQLite integration with optimistic UI.", link="/lesson-16-async-db", accent="#ef4444")
                    @LessonCard(num="17", title="Testing", desc="Unit and Integration testing strategies.", link="/lesson-17-testing", accent="#ef4444")
                    @LessonCard(num="18", title="Security", desc="XSS prevention and safe practices.", link="/lesson-18-security", accent="#ef4444")
                    @LessonCard(num="19", title="Authentication", desc="User sessions and login flows.", link="/lesson-19-auth", accent="#ef4444")
                    @LessonCard(num="20", title="Premium Sliders", desc="Modernized UI with 'appearance' property.", link="/lesson-20", accent="#ef4444")
                </div>
            </section>
        </div>

        <style>
            .page_wrapper {
                padding-bottom: "6rem";
                position: "relative";
            }

            .bg_glow {
                position: "absolute";
                top: "-10%";
                left: "50%";
                transform: "translateX(-50%)";
                width: "120%";
                height: "600px";
                background: "radial-gradient(circle at center, rgba(99, 102, 241, 0.15) 0%, rgba(15, 23, 42, 0) 70%)";
                pointer-events: "none";
                z-index: "-1";
            }

            // Hero
            .hero {
                text-align: "center";
                padding: "6rem 1rem 5rem";
                position: "relative";
            }
            .hero_badge {
                display: "inline-flex";
                align-items: "center";
                padding: "0.5rem 1rem";
                background: "rgba(99, 102, 241, 0.1)";
                border: "1px solid rgba(99, 102, 241, 0.2)";
                border-radius: "9999px";
                font-size: "0.875rem";
                font-weight: "500";
                color: "#818cf8";
                margin-bottom: "2rem";
                animation: "fadeInDown 0.6s ease-out";
                box-shadow: "0 0 20px -5px rgba(99, 102, 241, 0.2)";
            }
            .hero_title {
                font-size: "clamp(3rem, 8vw, 6rem)";
                font-weight: "800";
                line-height: "1.1";
                margin: "0 auto 1.5rem";
                max-width: "1000px";
                background: "linear-gradient(to right bottom, #fff 30%, #a5b4fc 100%)";
                -webkit-background-clip: "text";
                -webkit-text-fill-color: "transparent";
                animation: "fadeInUp 0.6s ease-out 0.1s both";
                letter-spacing: "-0.04em";
                text-shadow: "0 20px 40px rgba(0,0,0,0.2)";
            }
            .hero_subtitle {
                font-size: "1.25rem";
                color: "#94a3b8";
                max-width: "640px";
                margin: "0 auto 2.5rem";
                line-height: "1.6";
                animation: "fadeInUp 0.6s ease-out 0.2s both";
            }

            // Hero Actions
            .hero_actions {
                display: "flex";
                gap: "1rem";
                justify-content: "center";
                margin-bottom: "4rem";
                animation: "fadeInUp 0.6s ease-out 0.3s both";
            }
            .btn_primary {
                background: "#6366f1";
                color: "white";
                padding: "0.875rem 2rem";
                border-radius: "0.5rem";
                font-weight: "600";
                text-decoration: "none";
                transition: "all 0.2s";
                box-shadow: "0 4px 12px rgba(99, 102, 241, 0.3)";
            }
            .btn_primary:hover {
                background: "#4f46e5";
                transform: "translateY(-2px)";
                box-shadow: "0 8px 16px rgba(99, 102, 241, 0.4)";
            }
            .btn_secondary {
                background: "rgba(30, 41, 59, 0.5)";
                color: "#e2e8f0";
                padding: "0.875rem 2rem";
                border-radius: "0.5rem";
                font-weight: "600";
                text-decoration: "none";
                border: "1px solid rgba(255,255,255,0.1)";
                transition: "all 0.2s";
            }
            .btn_secondary:hover {
                background: "rgba(30, 41, 59, 0.8)";
                border-color: "rgba(255,255,255,0.2)";
            }

            .hero_stats {
                display: "inline-flex";
                align-items: "center";
                justify-content: "center";
                gap: "2rem";
                animation: "fadeInUp 0.6s ease-out 0.4s both";
                background: "rgba(15, 23, 42, 0.5)";
                padding: "1.5rem 3rem";
                border-radius: "1rem";
                border: "1px solid rgba(255,255,255,0.05)";
                backdrop-filter: "blur(10px)";
            }
            .stat {
                display: "flex";
                flex-direction: "column";
                align-items: "center";
            }
            .stat_divider {
                width: "1px";
                height: "40px";
                background: "rgba(255,255,255,0.1)";
            }
            .stat_number {
                font-size: "2rem";
                font-weight: "700";
                color: "#f8fafc";
                line-height: "1";
                margin-bottom: "0.25rem";
            }
            .stat_label {
                font-size: "0.875rem";
                color: "#64748b";
                font-weight: "600";
                text-transform: "uppercase";
                letter-spacing: "0.05em";
            }

            // Sections
            .section {
                max-width: "1200px";
                margin: "0 auto 4rem";
                padding: "0 1.5rem";
            }
            .section_title {
                font-size: "1.5rem";
                font-weight: "700";
                color: "#f1f5f9";
                margin-bottom: "2rem";
                display: "flex";
                align-items: "center";
                gap: "0.75rem";
            }
            .section_title::after {
                content: "''";
                flex-grow: "1";
                height: "1px";
                background: "linear-gradient(to right, rgba(255,255,255,0.1), transparent)";
                margin-left: "1rem";
            }

            // Grid
            .grid {
                display: "grid";
                grid-template-columns: "repeat(auto-fill, minmax(450px, 1fr))";
                gap: "1.5rem";
            }

            // Animations
            @keyframes fadeInUp {
                from { opacity: "0"; transform: "translateY(20px)"; }
                to { opacity: "1"; transform: "translateY(0)"; }
            }
            @keyframes fadeInDown {
                from { opacity: "0"; transform: "translateY(-10px)"; }
                to { opacity: "1"; transform: "translateY(0)"; }
            }
        </style>

    }
}

#[azumi::component]
#[allow(non_snake_case)]
fn LessonCard<'a>(
    num: &'a str,
    title: &'a str,
    desc: &'a str,
    link: &'a str,
    accent: &'a str,
) -> impl Component + 'a {
    html! {
        <a href={link} class={lesson_card} style={format!("--accent: {}", accent)}>
            <div class={card_glow}></div>
            <div class={card_content}>
                <div class={card_header}>
                    <span class={card_number}>{num}</span>
                    <div class={card_icon}>"→"</div>
                </div>
                <h3 class={card_title}>{title}</h3>
                <p class={card_desc}>{desc}</p>
                <div class={card_footer}>
                    <span class={learn_more}>"Start Lesson"</span>
                </div>
            </div>
        </a>
        <style>
            .lesson_card {
                display: "block";
                position: "relative";
                background: "rgba(30, 41, 59, 0.4)";
                border: "1px solid rgba(255, 255, 255, 0.05)";
                border-radius: "16px";
                text-decoration: "none";
                overflow: "hidden";
                transition: "all 0.4s cubic-bezier(0.4, 0, 0.2, 1)";
                height: "100%";
            }
            .card_glow {
                position: "absolute";
                top: "0";
                left: "0";
                right: "0";
                height: "100%";
                background: "radial-gradient(circle at top right, var(--accent), transparent 60%)";
                opacity: "0";
                transition: "opacity 0.4s";
                mix-blend-mode: "screen";
                pointer-events: "none";
            }
            .card_content {
                position: "relative";
                padding: "1.75rem";
                height: "100%";
                display: "flex";
                flex-direction: "column";
                z-index: "1";
            }

            .lesson_card:hover {
                transform: "translateY(-4px)";
                background: "rgba(30, 41, 59, 0.8)";
                border-color: "rgba(255,255,255,0.1)";
                box-shadow: "0 20px 40px -10px rgba(0, 0, 0, 0.5)";
            }
            .lesson_card:hover .card_glow {
                opacity: "0.15";
            }

            .card_header {
                display: "flex";
                justify-content: "space-between";
                align-items: "center";
                margin-bottom: "1rem";
            }
            .card_number {
                font-family: "monospace";
                font-size: "0.875rem";
                color: "var(--accent)";
                background: "rgba(255,255,255,0.05)";
                padding: "0.25rem 0.5rem";
                border-radius: "4px";
            }
            .card_icon {
                color: "var(--accent)";
                opacity: "0";
                transform: "translateX(-10px)";
                transition: "all 0.3s ease";
            }
            .lesson_card:hover .card_icon {
                opacity: "1";
                transform: "translateX(0)";
            }

            .card_title {
                font-size: "1.25rem";
                font-weight: "700";
                color: "#f1f5f9";
                margin: "0 0 0.5rem";
                line-height: "1.3";
            }
            .card_desc {
                color: "#94a3b8";
                font-size: "0.95rem";
                margin-bottom: "1.5rem";
                flex-grow: "1";
                line-height: "1.6";
            }

            .card_footer {
                margin-top: "auto";
            }
            .learn_more {
                font-size: "0.875rem";
                font-weight: "600";
                color: "var(--accent)";
                display: "flex";
                align-items: "center";
                gap: "0.5rem";
            }
        </style>
    }
}

pub async fn homepage_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&homepage()))
}
