use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::components::{
    badge::{Badge, BadgeVariant},
    checkbox::Checkbox,
    table::{
        Table, TableBody, TableCaption, TableCell, TableFooter, TableHead, TableHeader, TableRow,
    },
};

const DEFAULT: &str = r#"<Table>
    <TableHeader>
        <TableRow>
            <TableHead>"Invoice"</TableHead>
            <TableHead>"Status"</TableHead>
            <TableHead class="text-right">"Amount"</TableHead>
        </TableRow>
    </TableHeader>
    <TableBody>
        <TableRow>
            <TableCell class="font-medium">"INV-001"</TableCell>
            <TableCell>
                <Badge variant=BadgeVariant::Secondary>"Paid"</Badge>
            </TableCell>
            <TableCell class="text-right">"$250.00"</TableCell>
        </TableRow>
    </TableBody>
</Table>"#;

const WITH_CAPTION: &str = r#"<Table>
    <TableCaption>"Invoices from the last quarter."</TableCaption>
    ...
    <TableFooter>
        <TableRow>
            <TableCell attr:colspan="2">"Total"</TableCell>
            <TableCell class="text-right">"$1,050.00"</TableCell>
        </TableRow>
    </TableFooter>
</Table>"#;

const SELECTABLE: &str = r#"<TableRow selected=selected>
    <TableCell>
        <Checkbox checked=selected />
    </TableCell>
    <TableCell>"Ada Lovelace"</TableCell>
    <TableCell>"Owner"</TableCell>
</TableRow>"#;

const WIDE: &str = r#"<Table>
    <TableHeader>
        <TableRow>
            <TableHead>"Service"</TableHead>
            <TableHead>"Region"</TableHead>
            <TableHead>"Instance"</TableHead>
            <TableHead>"Uptime"</TableHead>
            <TableHead>"Requests"</TableHead>
            <TableHead>"Latency (p95)"</TableHead>
            <TableHead>"Error rate"</TableHead>
            <TableHead>"Saturation"</TableHead>
            <TableHead>"Version"</TableHead>
            <TableHead>"Last deploy"</TableHead>
        </TableRow>
    </TableHeader>
    <TableBody>
        <TableRow>
            <TableCell>"checkout-api"</TableCell>
            <TableCell>"eu-west-1"</TableCell>
            <TableCell>"c7g.2xlarge"</TableCell>
            <TableCell>"99.98%"</TableCell>
            <TableCell>"1.2M"</TableCell>
            <TableCell>"142ms"</TableCell>
            <TableCell>"0.04%"</TableCell>
            <TableCell>"61%"</TableCell>
            <TableCell>"v4.18.2"</TableCell>
            <TableCell>"2 hours ago"</TableCell>
        </TableRow>
    </TableBody>
</Table>"#;

#[component]
pub fn Page() -> impl IntoView {
    let ada = RwSignal::new(true);
    let grace = RwSignal::new(false);

    view! {
        <DocLayout title="Table" description="Rows and columns of data.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="Thin wrappers over the native table elements, so colspan, scope and the rest keep working."
                    code=DEFAULT
                >
                    <Table>
                        <TableHeader>
                            <TableRow>
                                <TableHead>"Invoice"</TableHead>
                                <TableHead>"Status"</TableHead>
                                <TableHead class="text-right">"Amount"</TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            <TableRow>
                                <TableCell class="font-medium">"INV-001"</TableCell>
                                <TableCell>
                                    <Badge variant=BadgeVariant::Secondary>"Paid"</Badge>
                                </TableCell>
                                <TableCell class="text-right">"$250.00"</TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell class="font-medium">"INV-002"</TableCell>
                                <TableCell>
                                    <Badge variant=BadgeVariant::Outline>"Pending"</Badge>
                                </TableCell>
                                <TableCell class="text-right">"$150.00"</TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell class="font-medium">"INV-003"</TableCell>
                                <TableCell>
                                    <Badge variant=BadgeVariant::Destructive>"Overdue"</Badge>
                                </TableCell>
                                <TableCell class="text-right">"$650.00"</TableCell>
                            </TableRow>
                        </TableBody>
                    </Table>
                </DemoSection>

                <DemoSection
                    title="Caption and footer"
                    description="The caption sits below the table; the footer is where a total belongs."
                    code=WITH_CAPTION
                >
                    <Table>
                        <TableCaption>"Invoices from the last quarter."</TableCaption>
                        <TableHeader>
                            <TableRow>
                                <TableHead>"Invoice"</TableHead>
                                <TableHead>"Customer"</TableHead>
                                <TableHead class="text-right">"Amount"</TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            <TableRow>
                                <TableCell class="font-medium">"INV-001"</TableCell>
                                <TableCell>"Wayfarer Ltd"</TableCell>
                                <TableCell class="text-right">"$250.00"</TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell class="font-medium">"INV-002"</TableCell>
                                <TableCell>"Northwind"</TableCell>
                                <TableCell class="text-right">"$150.00"</TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell class="font-medium">"INV-003"</TableCell>
                                <TableCell>"Blue Yonder"</TableCell>
                                <TableCell class="text-right">"$650.00"</TableCell>
                            </TableRow>
                        </TableBody>
                        <TableFooter>
                            <TableRow>
                                <TableCell attr:colspan="2">"Total"</TableCell>
                                <TableCell class="text-right">"$1,050.00"</TableCell>
                            </TableRow>
                        </TableFooter>
                    </Table>
                </DemoSection>

                <DemoSection
                    title="Selectable rows"
                    description="Pass selected to a row to render data-state=\"selected\", which the row styles itself from."
                    code=SELECTABLE
                >
                    <Table>
                        <TableHeader>
                            <TableRow>
                                <TableHead class="w-10" />
                                <TableHead>"Name"</TableHead>
                                <TableHead>"Role"</TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            <TableRow selected=ada>
                                <TableCell>
                                    <Checkbox checked=ada />
                                </TableCell>
                                <TableCell>"Ada Lovelace"</TableCell>
                                <TableCell>"Owner"</TableCell>
                            </TableRow>
                            <TableRow selected=grace>
                                <TableCell>
                                    <Checkbox checked=grace />
                                </TableCell>
                                <TableCell>"Grace Hopper"</TableCell>
                                <TableCell>"Admin"</TableCell>
                            </TableRow>
                        </TableBody>
                    </Table>
                </DemoSection>

                <DemoSection
                    title="Wide tables"
                    description="The table sits in its own horizontally scrolling container, so it never stretches the page."
                    code=WIDE
                >
                    <Table>
                        <TableHeader>
                            <TableRow>
                                <TableHead>"Service"</TableHead>
                                <TableHead>"Region"</TableHead>
                                <TableHead>"Instance"</TableHead>
                                <TableHead>"Uptime"</TableHead>
                                <TableHead>"Requests"</TableHead>
                                <TableHead>"Latency (p95)"</TableHead>
                                <TableHead>"Error rate"</TableHead>
                                <TableHead>"Saturation"</TableHead>
                                <TableHead>"Version"</TableHead>
                                <TableHead>"Last deploy"</TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            <TableRow>
                                <TableCell>"checkout-api"</TableCell>
                                <TableCell>"eu-west-1"</TableCell>
                                <TableCell>"c7g.2xlarge"</TableCell>
                                <TableCell>"99.98%"</TableCell>
                                <TableCell>"1.2M"</TableCell>
                                <TableCell>"142ms"</TableCell>
                                <TableCell>"0.04%"</TableCell>
                                <TableCell>"61%"</TableCell>
                                <TableCell>"v4.18.2"</TableCell>
                                <TableCell>"2 hours ago"</TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>"search-indexer"</TableCell>
                                <TableCell>"us-east-2"</TableCell>
                                <TableCell>"m6i.4xlarge"</TableCell>
                                <TableCell>"99.71%"</TableCell>
                                <TableCell>"430k"</TableCell>
                                <TableCell>"318ms"</TableCell>
                                <TableCell>"0.22%"</TableCell>
                                <TableCell>"88%"</TableCell>
                                <TableCell>"v2.9.0"</TableCell>
                                <TableCell>"yesterday"</TableCell>
                            </TableRow>
                        </TableBody>
                    </Table>
                </DemoSection>
            </div>
        </DocLayout>
    }
}
