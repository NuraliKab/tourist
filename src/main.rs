use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <main>
            { hero_section() }
            { overview_section() }
            { destinations_section() }
            { gallery_section() }
            { guides_section() }
            { itinerary_section() }
            { included_section() }
            { booking_section() }
        </main>
    }
}

fn hero_section() -> Html {
    html! {
        <section class="hero">
            <div class="hero-content">
                <div class="hero-subtitle">
                    <i class="fa-solid fa-star"></i>
                    { "Эксклюзивный маршрут" }
                </div>
                <h1 class="hero-title">{ "Большое турне" }</h1>
                <div class="hero-dates">
                    <i class="fa-regular fa-calendar" style="color: var(--accent-gold);"></i>
                    { "29 июня — 9 июля" }
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
            <h2 class="section-title gradient-text">{ "Ваш маршрут мечты" }</h2>
            <p class="text-center mb-xl" style="color: var(--text-muted);">
                { "Хотите увидеть главные жемчужины Европы и окунуться в восточную сказку? У нас есть маршрут вашей мечты!" }
            </p>
            
            <div class="overview-grid">
                <div class="highlight-box">
                    <div class="highlight-icon"><i class="fa-solid fa-map-location-dot"></i></div>
                    <div>
                        <h3 class="gold-text">{ "5 стран за 11 дней" }</h3>
                        <p class="text-xs" style="color: var(--text-muted);">{ "Франция, Бельгия, Нидерланды, Италия, Турция" }</p>
                    </div>
                </div>
                <div class="highlight-box">
                    <div class="highlight-icon"><i class="fa-solid fa-plane-departure"></i></div>
                    <div>
                        <h3 class="gold-text">{ "Трансфер включен" }</h3>
                        <p class="text-xs" style="color: var(--text-muted);">{ "Вылет и прилет из Астаны" }</p>
                    </div>
                </div>
                <div class="highlight-box">
                    <div class="highlight-icon"><i class="fa-solid fa-hotel"></i></div>
                    <div>
                        <h3 class="gold-text">{ "Проживание" }</h3>
                        <p class="text-xs" style="color: var(--text-muted);">{ "Отели 3-4 звезды с завтраками" }</p>
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
        Destination {
            name: "Париж",
            desc: "Вы увидите главный символ любви — Эйфелеву башню, прогуляетесь по Елисейским полям и окунетесь в мир детства в Диснейленде! 🏰✨",
            img: "/paris.png",
            icon: "fa-solid fa-heart",
        },
        Destination {
            name: "Брюссель",
            desc: "Концерт BTS, площадь Гран-Плас и вкуснейшие вафли. 🧇",
            img: "/brussels.png",
            icon: "fa-solid fa-music",
        },
        Destination {
            name: "Амстердам",
            desc: "Незабываемая экскурсия на лодках по живописным каналам, средневековая архитектура и атмосфера свободы. 🚤🏠",
            img: "/amsterdam.png",
            icon: "fa-solid fa-water",
        },
        Destination {
            name: "Милан",
            desc: "Вы оцените величие Дуомо и настоящую красоту Милана — мировую столицу моды и стиля. 👗🇮🇹",
            img: "/milan.png",
            icon: "fa-solid fa-shirt",
        },
        Destination {
            name: "Стамбул",
            desc: "Завершим путешествие на азиатско-европейском разломе! Вас ждет не только история, но и отличный шоппинг на знаменитых базарах. 🕌🛍",
            img: "/istanbul.png",
            icon: "fa-solid fa-moon",
        },
    ];

    html! {
        <section class="destinations-section">
            <h2 class="section-title gradient-text">{ "Незабываемые локации" }</h2>
            
            <div class="destinations-grid">
                {
                    destinations.into_iter().map(|dest| {
                        html! {
                            <div class="destination-card">
                                <img src={dest.img} alt={dest.name} class="destination-img" />
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
        "/g_paris.png",
        "/g_belgium.png",
        "/g_amsterdam.png",
        "/g_milan.png",
        "/g_istanbul.png",
    ];

    html! {
        <section class="gallery-section">
            <div class="container">
                <h2 class="section-title gradient-text">{ "Атмосфера путешествия" }</h2>
                
                <div class="gallery-grid">
                    {
                        images.into_iter().map(|img| {
                            html! {
                                <div class="gallery-item">
                                    <img src={img} alt="Gallery image" />
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
    html! {
        <section class="itinerary-section" style="background: var(--bg-darker); padding-top: var(--spacing-xxl); padding-bottom: var(--spacing-xxl);">
            <div class="container">
                <h2 class="section-title gradient-text">{ "Анонс программы" }</h2>
                <div class="glass-panel-gold">
                    <p style="color: var(--text-muted); margin-bottom: var(--spacing-md);">
                        <strong class="gold-text">{ "День 1-3: " }</strong> { "Париж. Прилет, размещение, прогулка по Монмартру. Экскурсия по Сене, Эйфелева башня и Лувр. Целый день в Диснейленде!" }
                    </p>
                    <p style="color: var(--text-muted); margin-bottom: var(--spacing-md);">
                        <strong class="gold-text">{ "День 4: " }</strong> { "Брюссель. Переезд во Францию, пешеходная экскурсия по центру, Гран-Плас, знаменитый писающий мальчик и дегустация вафель." }
                    </p>
                    <p style="color: var(--text-muted); margin-bottom: var(--spacing-md);">
                        <strong class="gold-text">{ "День 5-6: " }</strong> { "Амстердам. Каналы, Рейксмюсеум, площадь Дам и свободное время для шопинга." }
                    </p>
                    <p style="color: var(--text-muted); margin-bottom: var(--spacing-md);">
                        <strong class="gold-text">{ "День 7-8: " }</strong> { "Милан. Площадь Дуомо, Галерея Виктора Эммануила II, замок Сфорца. Время для модных бутиков." }
                    </p>
                    <p style="color: var(--text-muted);">
                        <strong class="gold-text">{ "День 9-11: " }</strong> { "Стамбул. Прилет в Турцию. Голубая мечеть, Айя-София, Гранд Базар. Прощальный ужин на Босфоре и вылет домой." }
                    </p>
                </div>
            </div>
        </section>
    }
}

fn included_section() -> Html {
    html! {
        <section class="included-section" style="background: var(--bg-dark); padding-top: var(--spacing-xl); padding-bottom: var(--spacing-xxl);">
            <div class="container grid-2-col">
                <div class="glass-panel" style="border-top: 3px solid #10b981; margin-bottom: var(--spacing-md);">
                    <h3 style="color: #10b981; margin-bottom: var(--spacing-md);">
                        <i class="fa-solid fa-circle-check" style="margin-right: 8px;"></i>
                        { "Что включено" }
                    </h3>
                    <ul style="list-style: none; color: var(--text-muted); padding-left: 0; line-height: 1.8;">
                        <li><i class="fa-solid fa-check" style="color: #10b981; margin-right: 8px; font-size: 0.8em;"></i>{ "Авиаперелеты (Астана - Европа - Стамбул - Астана)" }</li>
                        <li><i class="fa-solid fa-check" style="color: #10b981; margin-right: 8px; font-size: 0.8em;"></i>{ "Проживание в отелях 3-4* с завтраками" }</li>
                        <li><i class="fa-solid fa-check" style="color: #10b981; margin-right: 8px; font-size: 0.8em;"></i>{ "Трансферы между городами по программе" }</li>
                        <li><i class="fa-solid fa-check" style="color: #10b981; margin-right: 8px; font-size: 0.8em;"></i>{ "Обзорные экскурсии" }</li>
                    </ul>
                </div>
                
                <div class="glass-panel" style="border-top: 3px solid #94a3b8; margin-bottom: var(--spacing-md);">
                    <h3 style="color: #cbd5e1; margin-bottom: var(--spacing-md);">
                        <i class="fa-solid fa-wallet" style="margin-right: 8px; color: #94a3b8;"></i>
                        { "Дополнительно оплачивается" }
                    </h3>
                    <ul style="list-style: none; color: var(--text-muted); padding-left: 0; line-height: 1.8;">
                        <li><i class="fa-solid fa-plus" style="color: #94a3b8; margin-right: 8px; font-size: 0.8em;"></i>{ "Билет в Диснейленд" }</li>
                        <li><i class="fa-solid fa-plus" style="color: #94a3b8; margin-right: 8px; font-size: 0.8em;"></i>{ "Оформление Шенгенской визы" }</li>
                        <li><i class="fa-solid fa-plus" style="color: #94a3b8; margin-right: 8px; font-size: 0.8em;"></i>{ "Медицинская страховка" }</li>
                        <li><i class="fa-solid fa-plus" style="color: #94a3b8; margin-right: 8px; font-size: 0.8em;"></i>{ "Обеды и ужины" }</li>
                        <li><i class="fa-solid fa-plus" style="color: #94a3b8; margin-right: 8px; font-size: 0.8em;"></i>{ "Личные расходы и сувениры" }</li>
                    </ul>
                </div>
            </div>
        </section>
    }
}

fn guides_section() -> Html {
    let guides = vec![
        ("Гид 1", "Биография скоро появится..."),
        ("Гид 2", "Биография скоро появится..."),
        ("Гид 3", "Биография скоро появится..."),
    ];

    html! {
        <section class="guides-section" style="background: var(--bg-dark);">
            <div class="container">
                <h2 class="section-title gradient-text">{ "Ваши сопровождающие" }</h2>
                <p class="text-center mb-xl" style="color: var(--text-muted);">
                    { "Команда профессионалов, которые сделают ваше путешествие легким и незабываемым" }
                </p>
                
                <div class="guides-grid">
                    {
                        guides.into_iter().map(|(name, bio)| {
                            html! {
                                <div class="guide-card glass-panel">
                                    <div class="guide-photo-placeholder">
                                        <i class="fa-solid fa-user"></i>
                                    </div>
                                    <h3 class="guide-name">{ name }</h3>
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

fn booking_section() -> Html {
    html! {
        <>
        <section id="booking" class="booking-section">
            <div class="booking-container booking-info-desktop">
                <div class="price-box">
                    <div class="price-label">{ "Стоимость тура" }</div>
                    <div class="price-amount">{ "1 390 000 ₸" }</div>
                    <div class="price-person">{ "на одного человека" }</div>
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

fn main() {
    yew::Renderer::<App>::new().render();
}
