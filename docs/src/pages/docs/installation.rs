//! The installation page: why the styled layer is copied rather than depended on, and the CLI
//! that does the copying.

use crate::components::demo_section::DemoSection;
use crate::layout::doc_layout::DocLayout;
use leptos::prelude::*;
use nocomponents::{
    components::{code::CodeBlock, separator::Separator},
    utils::types::Language,
};

const INSTALL: &str = r#"cargo install nocomponents-cli"#;

const INIT: &str = r#"cd my-leptos-app
cargo nocli init"#;

const ADD: &str = r#"cargo nocli components add button dialog

#   added     src/components/nc/button.rs
#   added     src/components/nc/dialog.rs"#;

const ADD_DEPS: &str = r#"cargo nocli components add sidebar

#   added     src/components/nc/sidebar.rs
#   needed by src/components/nc/input.rs
#   needed by src/components/nc/separator.rs
#   needed by src/components/nc/sheet.rs
#   needed by src/components/nc/skeleton.rs
#   needed by src/components/nc/tooltip.rs
#   needed by src/components/nc/button.rs"#;

const CONFIG: &str = r#"[style]
css = "styles/globals.css"

[paths]
components = "src/components/nc""#;

const CARGO: &str = r#"[dependencies]
leptos = { version = "0.8", features = ["csr"] }
nocomponents = { version = "0.1", features = [
    "primitives", "icons", "theme",
    "button", "dialog", "sidebar",
] }"#;

const USE: &str = r#"use crate::components::nc::button::{Button, ButtonVariant};
use leptos::prelude::*;

#[component]
pub fn Toolbar() -> impl IntoView {
    view! {
        <Button variant=ButtonVariant::Outline>"Save"</Button>
    }
}"#;

const PRIMITIVES_ONLY: &str = r#"# No CLI, no copied files — just the behaviour, styled by you.
cargo add nocomponents --features primitives,dialog,popover"#;

const FROM: &str = r#"cargo nocli components add sidebar --from ../nocomponents"#;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout
            title="Installation"
            description="The styled layer is copied into your project, not depended on. Here is why, and how."
        >
            <div class="flex flex-col gap-8">
                <p class="text-sm leading-relaxed text-muted-foreground">
                    "A Tailwind build only emits a class if it can see the file that class is
                    written in, and it does not look inside other crates. So a project that merely
                    " <span class="font-medium text-foreground">"depends"</span>
                    " on this library would get the styled layer's markup with none of its CSS —
                    and there would be nothing the project could do about it, since "
                    <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                        "@source"
                    </code> " would have to point inside "
                    <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                        "~/.cargo/registry"
                    </code>
                    " and the tokens the classes are written against would still be missing."
                </p>
                <p class="text-sm leading-relaxed text-muted-foreground">
                    "So "
                    <span class="font-medium text-foreground">
                        "the styled layer is copied
                        into your own "
                    </span>
                    <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">"src/"</code>
                    ", where Tailwind already scans it, and it is yours to edit afterwards. What
                    stays an ordinary dependency is everything with no classes in it: the behaviour
                    primitives, the icons, "
                    <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                        "utils"
                    </code> " and "
                    <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                        "middleware"
                    </code> "."
                </p>

                <Separator />

                <DemoSection
                    title="1. Install the CLI"
                    description="A cargo subcommand. The binary is cargo-nocli, which is what makes `cargo nocli` work."
                >
                    <CodeBlock code=INSTALL language=Language::Shell />
                </DemoSection>

                <DemoSection
                    title="2. Set the project up"
                    description="Writes nocomponents.toml, installs globals.css and animate.css beside it, adds tailwindcss to Trunk.toml, and adds the dependency."
                >
                    <CodeBlock code=INIT language=Language::Shell />
                </DemoSection>

                <DemoSection
                    title="3. Add components"
                    description="Each one is copied into the install directory, and the matching Cargo features are switched on."
                >
                    <CodeBlock code=ADD language=Language::Shell />
                </DemoSection>

                <DemoSection
                    title="Components bring what they are built on"
                    description="A sidebar is built out of five other components. They are read out of the source as it is copied, so the list cannot fall out of step with the code."
                >
                    <CodeBlock code=ADD_DEPS language=Language::Shell />
                </DemoSection>

                <Separator />

                <DemoSection
                    title="What you end up with"
                    description="One dependency, and a directory of components you own."
                >
                    <CodeBlock code=CARGO language=Language::Rust label="Cargo.toml" />
                </DemoSection>

                <DemoSection
                    title="Using one"
                    description="They are ordinary modules in your crate, imported from wherever you told the CLI to put them."
                >
                    <CodeBlock code=USE language=Language::Rust />
                </DemoSection>

                <DemoSection
                    title="Configuration"
                    description="paths.components has to be under src/ — a component built on another needs to be able to name it, and only a directory under src/ has a module path. The mod.rs in there is rewritten on every add; the components beside it are yours."
                >
                    <CodeBlock code=CONFIG language=Language::Rust label="nocomponents.toml" />
                </DemoSection>

                <Separator />

                <DemoSection
                    title="Just the primitives"
                    description="The behaviour layer carries no classes, so it needs no copying and no stylesheet. If you are writing your own styling, this is the whole install."
                >
                    <CodeBlock code=PRIMITIVES_ONLY language=Language::Shell />
                </DemoSection>

                <DemoSection
                    title="Working against a checkout"
                    description="Reads the components from a local clone rather than the repository, and points the dependency it writes at that clone."
                >
                    <CodeBlock code=FROM language=Language::Shell />
                </DemoSection>

                <div class="rounded-lg border bg-muted/40 p-4">
                    <p class="text-xs leading-relaxed text-muted-foreground">
                        <span class="font-medium text-foreground">"An add never overwrites."</span>
                        " Copying in a component that is built on one you have already edited leaves
                        your version alone and says so — the point of owning the source is that it
                        stays yours. Pass "
                        <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                            "--force"
                        </code>
                        " when you do want it replaced."
                    </p>
                </div>
            </div>
        </DocLayout>
    }
}
