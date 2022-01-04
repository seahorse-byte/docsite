use dioxus::prelude::*;

pub fn FeaturedExamples(cx: Scope) -> Element {
    cx.render(rsx! {
        section { class: "bg-gray-800 body-font",
            div { class: "container px-6 lg:px-40 py-12 mx-auto",
                div { class: "flex flex-col text-center w-full mb-10",
                    h1 { class: "sm:text-3xl text-2xl font-medium title-font mb-4 text-white",
                        "Feature-packed examples"
                    }
                }
                div { class: "flex flex-wrap -m-4",
                    FeaturedExample {
                        title: "TodoMVC",
                        subtitle: "Web",
                        description: "The classic demonstration of a web framework. Fits in one file in less than 150 lines of code.",
                        link: "https://github.com/DioxusLabs/example-projects/tree/master/todomvc",
                        img: "https://dioxuslabs.com/static/todos_cropped.png",
                        img_alt: "",
                    }
                    FeaturedExample {
                        title: "E-Commerce Site",
                        subtitle: "Web",
                        description: "A complex website leveraging Tailwind, global state, animations, pre-rendering, and project structure.",
                        link: "https://github.com/DioxusLabs/example-projects/tree/master/ecommerce-site",
                        img: "https://dioxuslabs.com/static/marktplace_cropped.png",
                        img_alt: "",
                    }
                    FeaturedExample {
                        title: "File Explorer",
                        subtitle: "Desktop",
                        description: "Interact with native APIs directly from your UI. Works with a simple `cargo run` and is bundle-ready.",
                        link: "https://github.com/DioxusLabs/example-projects/tree/master/file-explorer",
                        img: "https://dioxuslabs.com/static/file_explorer.png",
                        img_alt: "",
                    }
                    FeaturedExample {
                        title: "WiFi Scanner",
                        subtitle: "Desktop, Mobile",
                        description: "Spawn native Tokio tasks and interact with hardware directly from your app with Dioxus coroutines.",
                        link: "https://github.com/DioxusLabs/example-projects/tree/master/wifi-scanner",
                        img: "https://dioxuslabs.com/static/scanner.png",
                        img_alt: "",
                    }
                    FeaturedExample {
                        title: "Documentation Site",
                        subtitle: "SSR",
                        description: "This very site is written in Dioxus, pre-rendered with SSR, and rehydrated with Dioxus-Web!",
                        link: "https://github.com/DioxusLabs/docsite",
                        img: "https://dioxuslabs.com/static/docsite_cropped.png",
                        img_alt: "",
                    }
                    FeaturedExample {
                        title: "JS Benchmark",
                        subtitle: "Web",
                        description: "The classic performance benchmark for web frameworks. Dioxus ranks extraordinarily high!",
                        link: "",
                        img: "https://dioxuslabs.com/static/jsbenchmark_cropped.png",
                        img_alt: "",
                    }
                }
            }
        }
    })
}

#[derive(PartialEq, Props)]
struct FeaturedExampleProps {
    title: &'static str,
    subtitle: &'static str,
    description: &'static str,
    link: &'static str,
    img: &'static str,
    img_alt: &'static str,
}

fn FeaturedExample(cx: Scope<FeaturedExampleProps>) -> Element {
    let FeaturedExampleProps {
        subtitle,
        title,
        description,
        link,
        img,
        img_alt,
    } = cx.props;

    cx.render(rsx!{
        div { class: "lg:w-1/3 sm:w-1/2 p-4",
            a { href: "{link}",
                div { class: "flex relative",
                    img { class: "absolute inset-0 w-full h-full object-cover object-center",
                        alt: "{img_alt}",
                        src: "{img}",
                    }
                    div { class: "px-8 py-10 relative z-10 w-full border-4 border-gray-200 bg-white opacity-0 hover:opacity-100",
                        h2 { class: "tracking-widest text-sm title-font font-medium text-indigo-500 mb-1",
                            "{subtitle}"
                        }
                        h1 { class: "title-font text-lg font-medium text-gray-900 mb-3",
                            "{title}"
                        }
                        p { class: "leading-relaxed",
                            "{description}"
                        }
                    }
                }
            }
        }
    })
}
