use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <main>
            { hero_section() }
            { overview_section() }
            { destinations_section() }
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
            
            <div class="glass-panel">
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
            
            <div class="destinations-list">
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

fn booking_section() -> Html {
    html! {
        <section id="booking" class="booking-section">
            <div class="price-box">
                <div class="price-label">{ "Стоимость тура" }</div>
                <div class="price-amount">{ "1 390 000 ₸" }</div>
                <div class="price-person">{ "на одного человека" }</div>
            </div>
            
            <p class="mb-lg" style="color: var(--text-main); font-size: 1.1rem;">
                { "Количество мест ограничено! Бронируйте свое место в этом невероятном приключении прямо сейчас." }
            </p>
            
            <a href="https://wa.me/+77013651063" target="_blank" class="whatsapp-button">
                <i class="fa-brands fa-whatsapp whatsapp-icon"></i>
                { "Написать в WhatsApp" }
            </a>
            
            <div class="scarcity-text">
                <i class="fa-solid fa-fire" style="color: #ef4444;"></i>
                { "Осталось мало мест" }
            </div>
        </section>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
