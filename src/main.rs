use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            { navbar() }
            <main>
                { hero_section() }
                { overview_section() }
                { stats_section() }
                { destinations_section() }
                { gallery_section() }
                { itinerary_section() }
                { included_section() }
                { guides_section() }
                { testimonials_section() }
                { faq_section() }
                { booking_section() }
            </main>
            { footer_section() }
        </>
    }
}

fn navbar() -> Html {
    html! {
        <nav class="navbar">
            <div class="navbar-inner container">
                <a href="#hero" class="navbar-brand">
                    <i class="fa-solid fa-compass"></i>
                    <span>{ "Большое турне" }</span>
                </a>
                <div class="navbar-links">
                    <a href="#destinations" class="nav-link">{ "Маршрут" }</a>
                    <a href="#itinerary" class="nav-link">{ "Программа" }</a>
                    <a href="#included" class="nav-link">{ "Услуги" }</a>
                    <a href="#faq" class="nav-link">{ "FAQ" }</a>
                    <a href="#booking" class="nav-cta">{ "Забронировать" }</a>
                </div>
            </div>
        </nav>
    }
}

fn hero_section() -> Html {
    html! {
        <section class="hero" id="hero">
            <div class="hero-content">
                <div class="hero-subtitle">
                    <i class="fa-solid fa-star"></i>
                    { "Эксклюзивный маршрут" }
                </div>
                <h1 class="hero-title">{ "Большое турне" }</h1>
                <p class="hero-tagline">{ "Европа + Стамбул" }</p>
                <div class="hero-dates">
                    <i class="fa-regular fa-calendar" style="color: var(--accent-gold);"></i>
                    { "29 июня — 9 июля 2026" }
                </div>
                <a href="#booking" class="button-primary">
                    { "Забронировать место" }
                    <i class="fa-solid fa-arrow-right"></i>
                </a>
            </div>
        </section>
    }
}

fn overview_section() -> Html {
    html! {
        <section class="overview-section">
            <div class="container">
                <h2 class="section-title gradient-text">{ "Ваш маршрут мечты" }</h2>
                <p class="overview-lead-text text-center" style="color: var(--text-muted); margin-bottom: var(--spacing-xl);">
                    { "Хотите увидеть главные жемчужины Европы и окунуться в восточную сказку? У нас есть маршрут вашей мечты!" }
                </p>
                <div class="overview-grid">
                    <div class="highlight-box">
                        <div class="highlight-icon"><i class="fa-solid fa-map-location-dot"></i></div>
                        <div>
                            <h3 class="gold-text">{ "5 стран за 11 дней" }</h3>
                            <p style="color: var(--text-muted); font-size: 0.9rem;">{ "Франция, Бельгия, Нидерланды, Италия, Турция" }</p>
                        </div>
                    </div>
                    <div class="highlight-box">
                        <div class="highlight-icon"><i class="fa-solid fa-plane-departure"></i></div>
                        <div>
                            <h3 class="gold-text">{ "Трансфер включен" }</h3>
                            <p style="color: var(--text-muted); font-size: 0.9rem;">{ "Вылет и прилет из Астаны" }</p>
                        </div>
                    </div>
                    <div class="highlight-box">
                        <div class="highlight-icon"><i class="fa-solid fa-hotel"></i></div>
                        <div>
                            <h3 class="gold-text">{ "Проживание" }</h3>
                            <p style="color: var(--text-muted); font-size: 0.9rem;">{ "Отели 3-4 звезды с завтраками" }</p>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

fn stats_section() -> Html {
    html! {
        <section class="stats-section">
            <div class="container">
                <div class="stats-grid">
                    <div class="stat-item">
                        <div class="stat-icon"><i class="fa-solid fa-earth-europe"></i></div>
                        <div class="stat-number gradient-text">{ "5" }</div>
                        <div class="stat-label">{ "стран" }</div>
                    </div>
                    <div class="stat-item">
                        <div class="stat-icon"><i class="fa-solid fa-calendar-days"></i></div>
                        <div class="stat-number gradient-text">{ "11" }</div>
                        <div class="stat-label">{ "дней" }</div>
                    </div>
                    <div class="stat-item">
                        <div class="stat-icon"><i class="fa-solid fa-award"></i></div>
                        <div class="stat-number gradient-text">{ "3+" }</div>
                        <div class="stat-label">{ "лет опыта" }</div>
                    </div>
                    <div class="stat-item">
                        <div class="stat-icon"><i class="fa-solid fa-headset"></i></div>
                        <div class="stat-number gradient-text">{ "24/7" }</div>
                        <div class="stat-label">{ "поддержка" }</div>
                    </div>
                </div>
            </div>
        </section>
    }
}

struct Destination {
    name: &'static str,
    desc: &'static str,
    img: &'static str,
    icon: &'static str,
}

fn destinations_section() -> Html {
    let destinations = vec![
        Destination { name: "Париж", desc: "Вы увидите главный символ любви — Эйфелеву башню, прогуляетесь по Елисейским полям и окунетесь в мир детства в Диснейленде! 🏰✨", img: "/paris.png", icon: "fa-solid fa-heart" },
        Destination { name: "Брюссель", desc: "Площадь Гран-Плас, знаменитый писающий мальчик и вкуснейшие бельгийские вафли. 🧇", img: "/brussels.png", icon: "fa-solid fa-music" },
        Destination { name: "Амстердам", desc: "Незабываемая экскурсия на лодках по живописным каналам, средневековая архитектура и атмосфера свободы. 🚤🏠", img: "/amsterdam.png", icon: "fa-solid fa-water" },
        Destination { name: "Милан", desc: "Вы оцените величие Дуомо и настоящую красоту Милана — мировую столицу моды и стиля. 👗🇮🇹", img: "/milan.png", icon: "fa-solid fa-shirt" },
        Destination { name: "Стамбул", desc: "Завершим путешествие на азиатско-европейском разломе! Вас ждет не только история, но и отличный шоппинг на знаменитых базарах. 🕌🛍", img: "/istanbul.png", icon: "fa-solid fa-moon" },
    ];

    html! {
        <section id="destinations" class="destinations-section">
            <h2 class="section-title gradient-text">{ "Незабываемые локации" }</h2>
            <div class="destinations-grid">
                {
                    destinations.into_iter().map(|dest| {
                        html! {
                            <div class="destination-card">
                                <img src={dest.img} alt={dest.name} class="destination-img" loading="lazy" />
                                <div class="destination-overlay"></div>
                                <div class="destination-content">
                                    <h3 class="destination-name">
                                        <i class={dest.icon} style="color: var(--accent-gold); font-size: 1.2rem;"></i>
                                        { dest.name }
                                    </h3>
                                    <p class="destination-desc">{ dest.desc }</p>
                                </div>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </section>
    }
}

fn gallery_section() -> Html {
    let images = vec![
        ("/g_paris.png", "Париж"),
        ("/g_belgium.png", "Бельгия"),
        ("/g_amsterdam.png", "Амстердам"),
        ("/g_milan.png", "Милан"),
        ("/g_istanbul.png", "Стамбул"),
    ];

    html! {
        <section class="gallery-section">
            <div class="container">
                <h2 class="section-title gradient-text">{ "Атмосфера путешествия" }</h2>
                <div class="gallery-grid">
                    {
                        images.into_iter().map(|(img, alt)| {
                            html! {
                                <div class="gallery-item">
                                    <img src={img} alt={alt} loading="lazy" />
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>
        </section>
    }
}

fn itinerary_section() -> Html {
    let days = vec![
        ("День 1-3", "Париж", "Прилет, размещение, прогулка по Монмартру. Экскурсия по Сене, Эйфелева башня и Лувр. Целый день в Диснейленде!", "fa-solid fa-heart"),
        ("День 4", "Брюссель", "Переезд, пешеходная экскурсия по центру, Гран-Плас, знаменитый писающий мальчик и дегустация вафель.", "fa-solid fa-music"),
        ("День 5-6", "Амстердам", "Каналы, Рейксмюсеум, площадь Дам и свободное время для шопинга.", "fa-solid fa-water"),
        ("День 7-8", "Милан", "Площадь Дуомо, Галерея Виктора Эммануила II, замок Сфорца. Время для модных бутиков.", "fa-solid fa-shirt"),
        ("День 9-11", "Стамбул", "Голубая мечеть, Айя-София, Гранд Базар. Прощальный ужин на Босфоре и вылет домой.", "fa-solid fa-moon"),
    ];

    html! {
        <section id="itinerary" class="itinerary-section">
            <div class="container">
                <h2 class="section-title gradient-text">{ "Программа тура" }</h2>
                <p class="text-center" style="color: var(--text-muted); margin-bottom: var(--spacing-xl);">
                    { "Каждый день — новые впечатления и открытия" }
                </p>
                <div class="timeline">
                    {
                        days.into_iter().map(|(day, city, desc, icon)| {
                            html! {
                                <div class="timeline-item">
                                    <div class="timeline-marker">
                                        <i class={icon}></i>
                                    </div>
                                    <div class="timeline-content glass-panel">
                                        <span class="timeline-day">{ day }</span>
                                        <h3 class="timeline-city gold-text">{ city }</h3>
                                        <p class="timeline-desc">{ desc }</p>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>
        </section>
    }
}

fn included_section() -> Html {
    html! {
        <section id="included" class="included-section" style="background: var(--bg-dark); padding-top: var(--spacing-xl); padding-bottom: var(--spacing-xxl);">
            <div class="container">
                <h2 class="section-title gradient-text">{ "Что входит в тур" }</h2>
                <div class="grid-2-col">
                    <div class="glass-panel" style="border-top: 3px solid #10b981; margin-bottom: var(--spacing-md);">
                        <h3 style="color: #10b981; margin-bottom: var(--spacing-md);">
                            <i class="fa-solid fa-circle-check" style="margin-right: 8px;"></i>
                            { "Что включено" }
                        </h3>
                        <ul style="list-style: none; color: var(--text-muted); padding-left: 0; line-height: 2;">
                            <li><i class="fa-solid fa-check" style="color: #10b981; margin-right: 8px; font-size: 0.8em;"></i>{ "Авиаперелеты (Астана — Европа — Стамбул — Астана)" }</li>
                            <li><i class="fa-solid fa-check" style="color: #10b981; margin-right: 8px; font-size: 0.8em;"></i>{ "Проживание в отелях 3-4★ с завтраками" }</li>
                            <li><i class="fa-solid fa-check" style="color: #10b981; margin-right: 8px; font-size: 0.8em;"></i>{ "Трансферы между городами по программе" }</li>
                            <li><i class="fa-solid fa-check" style="color: #10b981; margin-right: 8px; font-size: 0.8em;"></i>{ "Обзорные экскурсии в каждом городе" }</li>
                            <li><i class="fa-solid fa-check" style="color: #10b981; margin-right: 8px; font-size: 0.8em;"></i>{ "Сопровождение на протяжении всего тура" }</li>
                        </ul>
                    </div>

                    <div class="glass-panel" style="border-top: 3px solid #94a3b8; margin-bottom: var(--spacing-md);">
                        <h3 style="color: #cbd5e1; margin-bottom: var(--spacing-md);">
                            <i class="fa-solid fa-wallet" style="margin-right: 8px; color: #94a3b8;"></i>
                            { "Дополнительно оплачивается" }
                        </h3>
                        <ul style="list-style: none; color: var(--text-muted); padding-left: 0; line-height: 2;">
                            <li><i class="fa-solid fa-plus" style="color: #94a3b8; margin-right: 8px; font-size: 0.8em;"></i>{ "Билет в Диснейленд" }</li>
                            <li><i class="fa-solid fa-plus" style="color: #94a3b8; margin-right: 8px; font-size: 0.8em;"></i>{ "Оформление Шенгенской визы" }</li>
                            <li><i class="fa-solid fa-plus" style="color: #94a3b8; margin-right: 8px; font-size: 0.8em;"></i>{ "Медицинская страховка" }</li>
                            <li><i class="fa-solid fa-plus" style="color: #94a3b8; margin-right: 8px; font-size: 0.8em;"></i>{ "Обеды и ужины" }</li>
                            <li><i class="fa-solid fa-plus" style="color: #94a3b8; margin-right: 8px; font-size: 0.8em;"></i>{ "Личные расходы и сувениры" }</li>
                        </ul>
                    </div>
                </div>
            </div>
        </section>
    }
}

fn guides_section() -> Html {
    let guides = vec![
        ("Организатор тура", "Ваш главный компаньон в путешествии. Позаботится обо всех деталях — от перелётов до ресторанов.", "fa-solid fa-user-tie"),
        ("Гид по Европе", "Знает все секреты европейских городов и покажет вам не только туристические места.", "fa-solid fa-map-location-dot"),
        ("Координатор", "Следит за тем, чтобы каждый день был спланирован идеально и без сюрпризов.", "fa-solid fa-clipboard-check"),
    ];

    html! {
        <section class="guides-section">
            <div class="container">
                <h2 class="section-title gradient-text">{ "Ваши сопровождающие" }</h2>
                <p class="text-center" style="color: var(--text-muted); margin-bottom: var(--spacing-xl);">
                    { "Команда профессионалов, которые сделают ваше путешествие лёгким и незабываемым" }
                </p>
                <div class="guides-grid">
                    {
                        guides.into_iter().map(|(role, bio, icon)| {
                            html! {
                                <div class="guide-card glass-panel">
                                    <div class="guide-photo-placeholder">
                                        <i class={icon}></i>
                                    </div>
                                    <h3 class="guide-name">{ role }</h3>
                                    <p class="guide-bio">{ bio }</p>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>
        </section>
    }
}

fn testimonials_section() -> Html {
    let reviews: Vec<(&str, &str, u8)> = vec![
        ("Алия К.", "Это было невероятное путешествие! Организация на высшем уровне — нам не пришлось ни о чём беспокоиться. Особенно запомнился Диснейленд и прогулки по Парижу!", 5),
        ("Максим Д.", "Каждый город был как отдельное приключение. Особенно поразил Амстердам — каналы, архитектура, атмосфера. Спасибо за идеальный маршрут!", 5),
        ("Динара С.", "Путешествовали с подругой — это лучший отпуск в моей жизни! Стамбул стал настоящим открытием. Отели отличные, трансферы вовремя. Едем снова!", 5),
    ];

    html! {
        <section class="testimonials-section">
            <div class="container">
                <h2 class="section-title gradient-text">{ "Отзывы путешественников" }</h2>
                <div class="testimonials-grid">
                    {
                        reviews.into_iter().map(|(name, text, stars)| {
                            html! {
                                <div class="testimonial-card glass-panel">
                                    <div class="testimonial-stars">
                                        { (0..stars).map(|_| html! { <i class="fa-solid fa-star"></i> }).collect::<Html>() }
                                    </div>
                                    <blockquote class="testimonial-text">{ text }</blockquote>
                                    <div class="testimonial-author">
                                        <i class="fa-solid fa-circle-user"></i>
                                        <span>{ name }</span>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>
        </section>
    }
}

fn faq_section() -> Html {
    let faqs = vec![
        ("Нужна ли виза для тура?", "Да, для посещения Франции, Бельгии, Нидерландов и Италии требуется Шенгенская виза. Мы поможем с подготовкой документов. Оформление визы оплачивается отдельно."),
        ("Как забронировать место?", "Напишите нам в WhatsApp, и мы расскажем все детали. Для бронирования потребуется предоплата. Количество мест ограничено!"),
        ("Можно ли поехать с детьми?", "Да, мы рады семейным путешественникам! Напишите нам, и мы обсудим все детали, включая размещение и активности для детей."),
        ("Что делать, если визу не одобрят?", "В случае отказа в визе мы вернём полную стоимость тура за вычетом визовых сборов. Ваши деньги в безопасности."),
        ("Какой багаж можно взять?", "Нормы багажа зависят от авиакомпании. Обычно это 23 кг зарегистрированного багажа и 8 кг ручной клади. Мы сообщим точные данные после бронирования."),
        ("Входит ли питание в стоимость?", "В стоимость включены завтраки в отелях. Обеды и ужины оплачиваются самостоятельно — это позволяет вам выбирать рестораны по вкусу."),
    ];

    html! {
        <section id="faq" class="faq-section">
            <div class="container">
                <h2 class="section-title gradient-text">{ "Частые вопросы" }</h2>
                <div class="faq-list">
                    {
                        faqs.into_iter().map(|(question, answer)| {
                            html! {
                                <details class="faq-item">
                                    <summary class="faq-question">
                                        <span>{ question }</span>
                                        <i class="fa-solid fa-chevron-down faq-chevron"></i>
                                    </summary>
                                    <div class="faq-answer">
                                        <p>{ answer }</p>
                                    </div>
                                </details>
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>
        </section>
    }
}

fn booking_section() -> Html {
    html! {
        <>
        <section id="booking" class="booking-section">
            <div class="booking-container booking-info-desktop">
                <div class="price-box">
                    <div class="price-label hide-on-desktop">{ "Стоимость тура" }</div>
                    <div class="price-amount">{ "1 390 000 ₸" }</div>
                    <div class="price-person hide-on-desktop">{ "на одного человека" }</div>
                    <div class="price-details hide-on-mobile">
                        <div class="price-label">{ "Стоимость тура" }</div>
                        <div class="price-person">{ "на одного человека" }</div>
                    </div>
                </div>

                <p class="mb-lg hide-on-desktop" style="color: var(--text-main); font-size: 1.1rem; line-height: 1.4;">
                    { "Количество мест ограничено! Бронируйте свое место в этом невероятном приключении прямо сейчас." }
                </p>
            </div>

            <div class="booking-action-desktop hide-on-mobile-unless-scrolled">
                <a href="https://wa.me/+77013651063" target="_blank" class="whatsapp-button">
                    <i class="fa-brands fa-whatsapp whatsapp-icon"></i>
                    { "Написать в WhatsApp" }
                </a>

                <div class="scarcity-text">
                    <i class="fa-solid fa-fire" style="color: #ef4444;"></i>
                    { "Осталось мало мест" }
                </div>
            </div>
        </section>

        <div class="mobile-sticky-cta hide-on-desktop">
            <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 10px;">
                <div class="price-amount" style="font-size: 1.25rem; margin-bottom: 0;">{ "1 390 000 ₸" }</div>
                <div class="scarcity-text" style="margin-top: 0; font-size: 0.8rem;">
                    <i class="fa-solid fa-fire" style="color: #ef4444;"></i>
                    { "Мало мест" }
                </div>
            </div>
            <a href="https://wa.me/+77013651063" target="_blank" class="whatsapp-button" style="padding: 12px; font-size: 1.05rem;">
                <i class="fa-brands fa-whatsapp whatsapp-icon"></i>
                { "Написать в WhatsApp" }
            </a>
        </div>
        </>
    }
}

fn footer_section() -> Html {
    html! {
        <footer class="site-footer">
            <div class="container">
                <div class="footer-grid">
                    <div class="footer-brand-col">
                        <h3 class="footer-title">
                            <i class="fa-solid fa-compass" style="color: var(--accent-gold); margin-right: 8px;"></i>
                            { "Большое турне" }
                        </h3>
                        <p style="color: var(--text-muted); font-size: 0.9rem; line-height: 1.6;">
                            { "Незабываемые путешествия по Европе из Казахстана. Мы организуем всё — вы просто наслаждаетесь." }
                        </p>
                    </div>
                    <div class="footer-contact-col">
                        <h4 class="footer-heading">{ "Связаться с нами" }</h4>
                        <a href="https://wa.me/+77013651063" target="_blank" class="footer-link">
                            <i class="fa-brands fa-whatsapp"></i>
                            { " +7 701 365 1063" }
                        </a>
                    </div>
                    <div class="footer-social-col">
                        <h4 class="footer-heading">{ "Мы в соцсетях" }</h4>
                        <div class="social-icons">
                            <a href="#" class="social-icon" aria-label="Instagram">
                                <i class="fa-brands fa-instagram"></i>
                            </a>
                            <a href="#" class="social-icon" aria-label="Telegram">
                                <i class="fa-brands fa-telegram"></i>
                            </a>
                        </div>
                    </div>
                </div>
                <div class="footer-bottom">
                    <p>{ "© 2026 Большое турне. Все права защищены." }</p>
                </div>
            </div>
        </footer>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
