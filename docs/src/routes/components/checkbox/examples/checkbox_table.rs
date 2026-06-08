use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CheckboxTableExample() -> impl IntoView {
    #[derive(Clone)]
    struct Employee {
        id: String,
        name: String,
        email: String,
        role: String,
    }

    let employees = vec![
        Employee {
            id: "1".to_string(),
            name: "Sarah Chen".to_string(),
            email: "sarah.chen@example.com".to_string(),
            role: "Admin".to_string(),
        },
        Employee {
            id: "2".to_string(),
            name: "Marcus Rodriguez".to_string(),
            email: "marcus.rodriguez@example.com".to_string(),
            role: "User".to_string(),
        },
        Employee {
            id: "3".to_string(),
            name: "Priya Patel".to_string(),
            email: "priya.patel@example.com".to_string(),
            role: "User".to_string(),
        },
        Employee {
            id: "4".to_string(),
            name: "David Kim".to_string(),
            email: "david.kim@example.com".to_string(),
            role: "Editor".to_string(),
        },
    ];

    let select_all = RwSignal::new(false);

    view! {
        <Table>
            <TableHeader>
                <TableRow>
                    <TableHead class="w-8">
                        <Checkbox
                            id="select-all-checkbox"
                            name="select-all-checkbox"
                            checked=select_all
                        />
                    </TableHead>
                    <TableHead>"Name"</TableHead>
                    <TableHead>"Email"</TableHead>
                    <TableHead>"Role"</TableHead>
                </TableRow>
            </TableHeader>
            <TableBody>
                <For
                    each=move || employees.clone()
                    key=|employee| employee.id.clone()
                    let(employee)
                >
                    <TableRow>
                        <TableCell>
                            <Checkbox value=employee.id />
                        </TableCell>
                        <TableCell class="font-medium">{employee.name}</TableCell>
                        <TableCell>{employee.email}</TableCell>
                        <TableCell>{employee.role}</TableCell>
                    </TableRow>
                </For>
            </TableBody>
        </Table>
    }
}
