use yew::{prelude::*, props};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="container">
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
                    <Projects ..props />
                </div>
            </div>
            <div class="col">
                <div class={ if !is_mobile { "appear3" } else { "" } }>
                    <Experience ..props />
                    <Skills ..props />
                    <Education  ..props />
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

            <div class="contact-datum"><a href="https://www.linkedin.com/in/Kethas">{ "LinkedIn" } <span class="link">{ "(linkedin.com/in/Kethas)" }</span></a></div>
            <div class="contact-datum"><a href="https://github.com/Kethas">{ "GitHub" } <span class="link">{ "(github.com/Kethas)" }</span></a></div>
            <div class="contact-datum print-only"><a href="https://cv.asphyx.dev">{ "View CV Online" } <span class="link">{ "(cv.asphyx.dev)" }</span></a></div>
            <div class="contact-datum print-only"><a href="https://portfolio.asphyx.dev">{ "View Portfolio Online" }<span class="link">{ "(portfolio.asphyx.dev)" }</span></a></div>


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
                    "I began programming at the age of 12, encouraged by my brothers though driven by my own curiosity.
                    When computer science and programming classes were introduced in my school, I had already gained proficiency in Java and Lua through self-learning."
                }
            </p>
            <p>
                {
                    "My approach to acquiring new skills is rooted in a deep curiosity and a passion for learning.
                    Over time, I've independently explored various programming languages, frameworks, and paradigms (as seen below), allowing me to develop the ability to quickly adapt to new challenges. 
                    This adaptability extends beyond the technical realm and includes areas such as hardware, computer graphics, and interpersonal skills."
                }
            </p>
            <p>
                {
                    "My primary goal is to contribute positively to the team and work environment.
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
                            <li>{ "Lua (10 Yrs)" }</li>
                            <li>{ "Java (10 Yrs)" }</li>
                            <li>{ "C# (8 Yrs)" }</li>
                            <li>{ "JavaScript/\u{200B}TypeScript (8\u{202F}Yrs)" }</li>
                            <li>{ "HTML (8 Yrs)" }</li>
                            <li>{ "CSS (8 Yrs)" }</li>
                            <li>{ "C/C++ (7 Yrs)" }</li>
                            <li>{ "NodeJS (6 Yrs)" }</li>
                            <li>{ "Python (6 Yrs)" }</li>
                            <li>{ "Rust (5 Yrs)" }</li>

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
                    <div>
                        <h2>{ "Cloud Platforms" }</h2>
                        <ul class="skills-list">
                            <li>{ "Heroku" }</li>
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
            </div>
            <div class={ if props.is_mobile { "zero-opacity" } else { "" } }>
                <p>
                {
                    "As a Freelance Software Developer, I successfully delivered many complex projects for a diverse international clientele,
                    earning consistently positive feedback for exceptional quality, timely execution, and robust implementation. 
                    Collaborating closely with clients honed my communicative and entrepreneurial skills, 
                    creating solutions that precisely matched their visions and requirements. 
                    Often projects required expert level understanding of technologies I was not familiar with at the time, which I showed great dedication in learning and improving my professional skills
                    managing to complete these projects, while designing functional front- and back-end expertly as expected by my clientele.
                    "
                }
                </p>
            </div>
            <div class={ if props.is_mobile { "zero-opacity" } else { "" } }>
                <h3>
                {
                    "Customization of Open Source GitHub software in Rust"
                }
                </h3>
                <p>
                {
                    "I skillfully customized existing open-source repositories using Rust, expertly adapting and augmenting features within their codebase
                    while preserving functional integrity. A crucial aspect of my work involved enhancing the IPC of the program,
                    ensuring a seamless transition without compromising its original functionality. To elevate user engagement and satisfaction, I revamped the UI design using HTML, CSS, and domain-specific languages like TIScript, fulfilling my client's request for a complete application rebrand.
                    "
                }
                </p>
            </div>
            <div class={ if props.is_mobile { "zero-opacity" } else { "" } }>
                <h3>
                {
                    "Translation of Ethereum contract into Aptos Move"
                }
                </h3>
                <p>
                {
                    "I undertook the translation of a substantial Ethereum smart contract from Solidity to Aptos Move,
                    simultaneously acquiring proficiency in both languages and expanding my technical expertise in cryptocurrency operations. 
                    Upon completion, I gained comprehensive knowledge in these domains, delving into the intricacies of their complex blockchains and attaining expert-level insights. 
                    I delivered substantial value to my client by providing a detailed list of translation differences between the two contracts.
                    
                    "
                }
                </p>
            </div>
            // <div class={ if props.is_mobile { "zero-opacity" } else { "" } }>
            //     <h3>
            //     {
            //         "Development of various terminal/background apps using OS specific APIs for Windows, Linux, and MacOS"
            //     }
            //     </h3>
            //     <p>
            //     {
            //         "Many apps I have developed had led me to learn and research OS specific APIs."
            //     }
            //     </p>
            // </div>
            <div class={ if props.is_mobile { "zero-opacity" } else { "" } }>
                <h3>
                {
                    "Contribution to Open Source project 'egui.info'"
                }
                </h3>
                <p>
                {
                    "I actively contributed to the open-source project 'egui.info' by collaborating through Git,
                    submitting Pull Requests on GitHub, and successfully upgrading the project's dependency version.
                    Utilizing Rust, I meticulously enhanced and fine-tuned the entire collection of examples, elevating the overall quality of the project.
                    "
                }
                </p>
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
                <p>
                {
                    "Implemented key game mechanics, including terrain generation, player movement, and interaction with game objects."
                }
                </p>
                <p>
                {
                    "Utilized Rust's strong memory safety features to ensure stability and performance."
                }
                </p>
            </div>

            <div class={ if props.is_mobile { "zero-opacity" } else { "" } }>
                <h2>{ "RailsPlus" }</h2>
                <div class="subtitle">{ "Minecraft Mod with over 150 000 downloads" }</div>
                <p>
                {
                    "Created RailsPlus in 2 weeks during high school, a Minecraft Mod written in Java adding new types of interactable rails to the game."
                }
                </p>
            </div>

            <div class={ if props.is_mobile { "zero-opacity" } else { "" } }>
                <h2>{ "Psi" }</h2>
                <div class="subtitle">{ "User Friendly parser written in Rust" }</div>
                <p>
                {
                    "Developed Psi, a parser prioritizing ease of use over efficiency."
                }
                </p>
                <p>
                {
                    "Utilized Rust's expressive syntax and memory safety features to explore my passion towards programming languages and created a reliable and maintainable parser.   "
                }
                </p>
            </div>

            <div class={ if props.is_mobile { "zero-opacity" } else { "" } }>
                <h2>{ "CV and Portfolio" }</h2>
                <div class="subtitle">{ "Animated webpages showcasing my skills and projects" }</div>
                <p>
                {
                    "Created this CV and its accompanying portfolio as static webpages from conception to a finished product within a week."
                }
                </p>
                <p>
                {
                    "Learned to master HTML and CSS and CSS animations and solved challenging issues that arose due to using Rust for web."
                }
                </p>
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
            <div><a href="https://portfolio.asphyx.dev">{ "View my Portfolio" }</a></div>
        </div>
    }
}
