use yew::{prelude::*, props};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <Header />

            <div class="page">
                <div class="page-content">
                    <Cv />
                </div>
            </div>

        </div>
    }
}

#[derive(Copy, Clone, Properties, PartialEq)]
pub struct MobileProps {
    is_mobile: bool,
}

#[function_component(Cv)]
pub fn cv() -> Html {
    let inner_width = web_sys::window()
        .unwrap()
        .inner_width()
        .unwrap()
        .as_f64()
        .unwrap();

    let is_mobile = inner_width <= 700.0;

    let props = props! {
        MobileProps {
            is_mobile
        }
    };

    html! {
        <main id="cv">
            <div class="col">
                <Intro />
                <div class={ if !is_mobile { "appear2" } else { "" } }>
                    <ContactInfo ..props />
                    <AboutMe ..props />
                    <Skills ..props />
                    <Education  ..props />
                </div>
            </div>
            <div class="col">
                <div class={ if !is_mobile { "appear3" } else { "" } }>
                    <Experience ..props />
                    <Projects ..props />
                </div>
            </div>
        </main>
    }
}

/*
==============
|| Column 1 ||
==============
*/

#[function_component(Intro)]
pub fn intro() -> Html {
    html! {
        <>
            <div class="name appear0">{ "Ilay Ron" }</div>
            <div class="headline appear1">{ "Self-Taught Programmer" }</div>
        </>
    }
}

#[function_component(ContactInfo)]
pub fn contact_info(props: &MobileProps) -> Html {
    html! {
        <div class={ if props.is_mobile { "zero-opacity" } else { "" } }>
            <div class="contact-datum">{ "ilayron01@gmail.com" }</div>
            <div class="contact-datum">{ "+45 91 44 49 12" }</div>
            <div class="contact-datum">{ "Copenhagen, Denmark" }</div>

            <div class="contact-datum"><a href="https://www.linkedin.com/in/Kethas">{ "LinkedIn" } <span class="link">{ "(https://www.linkedin.com/in/Kethas)" }</span></a></div>
            <div class="contact-datum"><a href="https://github.com/Kethas">{ "GitHub" } <span class="link">{ "(https://github.com/Kethas)" }</span></a></div>
            <div class="contact-datum print-only"><a href="https://portfolio.asphyx.dev">{ "Portfolio" }<span class="link">{ "(https://portfolio.asphyx.dev)" }</span></a></div>


        </div>
    }
}

#[function_component(AboutMe)]
pub fn about_me(props: &MobileProps) -> Html {
    html! {
        <div class={ if props.is_mobile { "zero-opacity" } else { "" } }>
            <h1>{ "About Me" }</h1>
            <p>
                {
                    "I began programming at the age of 12, encouraged by my brothers though driven by my own curiosity. \
                    When computer science and programming classes were introduced in my school, I had already gained proficiency in Java and Lua through self-learning."
                }
                <br />
                {
                    "My approach to acquiring new skills is rooted in a deep curiosity and a passion for learning. \
                    Over time, I've independently explored various programming languages, frameworks, and paradigms (as seen below), allowing me to develop the ability to quickly adapt to new challenges. \
                    This adaptability extends beyond the technical realm and includes areas such as hardware, computer graphics, and interpersonal skills."
                }
                <br />
                {
                    "My primary goal is to contribute positively to the team and work environment. \
                    I'm genuinely motivated and committed to personal growth, always seeking opportunities to learn, reflect, and enhance my abilities."
                }
            </p>
        </div>
    }
}

#[function_component(Skills)]
pub fn skills(props: &MobileProps) -> Html {
    html! {
        <div class={ if props.is_mobile { "zero-opacity" } else { "" } }>
            <h1>{ "Skills" }</h1>
            <div class="skills-container">
                <div class="skills-col">
                    <div>
                        <h2>{ "Languages" }</h2>
                        <ul class="skills-list">
                            <li>{ "Lua" }</li>
                            <li>{ "Java" }</li>
                            <li>{ "C#" }</li>
                            <li>{ "C/C++" }</li>
                            <li>{ "Rust" }</li>
                            <li>{ "JavaScript/\u{200B}TypeScript" }</li>
                            <li>{ "HTML" }</li>
                            <li>{ "CSS" }</li>
                            <li>{ "NodeJS" }</li>
                            <li>{ "Python" }</li>
                        </ul>
                    </div>
                    <div>
                        <h2>{ "Cloud Platforms" }</h2>
                        <ul class="skills-list">
                            <li>{ "Heroku" }</li>
                        </ul>
                    </div>
                </div>
                <div class="skills-col">
                    <div>
                        <h2>{ "Frameworks" }</h2>
                        <ul class="skills-list">
                            <li>{ "React, Angular, Svelte, Vue, Yew, etc." }</li>
                            <li>{ "WebGPU/WGPU for Rust" }</li>
                            <li>{ "OpenGL" }</li>
                            <li>{ "Unity Game Engine" }</li>
                        </ul>
                    </div>
                    <div>
                        <h2>{ "Databases" }</h2>
                        <ul class="skills-list">
                            <li>{ "MongoDB" }</li>
                            <li>{ "SQL" }</li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[function_component(Education)]
pub fn education(props: &MobileProps) -> Html {
    html! {
        <div class={ if props.is_mobile { "zero-opacity" } else { "" } }>
            <h1>{ "Education" }</h1>
            <h2>{ "International Baccalaureate (IB)" }</h2>
            <div class="subtitle">{ "Copenhagen International School 2018-2020" }</div>
            <p>{ "Including Higher Level Computer Science, Math and Physics" }</p>
        </div>
    }
}

/*
==============
|| Column 2 ||
==============
*/

#[function_component(Experience)]
pub fn experience(props: &MobileProps) -> Html {
    html! {
        <>
            <h1 class={ if props.is_mobile { "zero-opacity" } else { "" } }>{ "Experience" }</h1>

            <div class={ if props.is_mobile { "zero-opacity" } else { "" } }>
                <h2>{ "Freelance Software Developer \u{2014} International Clients" }</h2>
                <div class="subtitle">{ "April 2021 - Present / Remote / Self-Employed" }</div>
                <ul class="bullet-points">
                    <li>
                    {
                        "Successfully completed a variety of projects as a freelance software developer, collaborating with international clients and consistently receiving great reviews for delivering high-quality solutions."
                    }
                    </li>
                    <li>
                    {
                        "Demonstrated versatility and adaptability by working on diverse projects, such as building a smart contract for cryptocurrencies, developing platform-specific applications for both Mac and Windows by utilizing a cross-platform code base in Rust and showcasing my ability to modify existing Rust codebases to meet specific project requirements."
                    }
                    </li>
                    <li>
                    {
                        "Leveraged strong problem-solving and communication skills to understand client requirements, provide technical expertise, and deliver high-quality solutions."
                    }
                    </li>
                    <li>
                    {
                        "Demonstrated self-motivation and a proactive approach in managing projects independently, including task prioritization, time management, and meeting project deadlines."
                    }
                    </li>
                </ul>
            </div>
        </>
    }
}

#[function_component(Projects)]
pub fn projects(props: &MobileProps) -> Html {
    html! {
        <>
            <h1 class={ if props.is_mobile { "zero-opacity" } else { "" } }>{ "Projects" }</h1>

            <div class={ if props.is_mobile { "zero-opacity" } else { "" } }>
                <h2>{ "Moinkroft" }</h2>
                <div class="subtitle">{ "Minecraft Clone written in Rust + WGPU" }</div>
                <ul class="bullet-points">
                    <li>
                    {
                        "Implemented key game mechanics, including terrain generation, player movement, and interaction with game objects."
                    }
                    </li>
                    <li>
                    {
                        "Utilized Rust's strong memory safety features to ensure stability and performance."
                    }
                    </li>
                </ul>
            </div>

            <div class={ if props.is_mobile { "zero-opacity" } else { "" } }>
                <h2>{ "RailsPlus" }</h2>
                <div class="subtitle">{ "Minecraft Mod with over 150 000 downloads" }</div>
                <ul class="bullet-points">
                    <li>
                    {
                        "Created RailsPlus in 2 weeks during high school, a Minecraft Mod written in Java adding new types of interactable rails to the game."
                    }
                    </li>
                </ul>
            </div>

            <div class={ if props.is_mobile { "zero-opacity" } else { "" } }>
                <h2>{ "Psi" }</h2>
                <div class="subtitle">{ "User Friendly parser written in Rust" }</div>
                <ul class="bullet-points">
                    <li>
                    {
                        "Developed Psi, a parser prioritizing ease of use over efficiency."
                    }
                    </li>
                    <li>
                    {
                        "Utilized Rust's expressive syntax and memory safety features to explore my passion towards programming languages and created a reliable and maintainable parser.   "
                    }
                    </li>
                </ul>
            </div>
        </>
    }
}

/*
==============
||  Header  ||
==============
*/

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div class="header appear3 screen-only">
            <div><a href="https://github.com/Kethas/cv/">{ "Source Code on Github" }</a></div>
            <span class="bullet" />
            <div><a href="#">{ "View my Portfolio" }</a></div>
        </div>
    }
}
