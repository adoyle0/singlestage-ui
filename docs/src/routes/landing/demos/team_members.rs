use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TeamMembers() -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <CardTitle>"Team Members"</CardTitle>
                <CardDescription>"Invite your team members to collaborate."</CardDescription>
            </CardHeader>
            <CardContent>
                <ul class="grid gap-4">
                    <li class="flex items-center gap-4">
                        <Avatar class="size-10">
                            <AvatarImage src="/avatar-1.png" />
                        </Avatar>

                        <div class="flex flex-col gap-1 mr-auto">
                            <p class="text-sm font-semibold leading-none">"Sofia Davis"</p>
                            <p class="text-sm text-(--muted-foreground)">"m@example.com"</p>
                        </div>

                        <Select default="owner">
                            <SelectItem value="viewer">"Viewer"</SelectItem>
                            <SelectItem value="developer">"Developer"</SelectItem>
                            <SelectItem value="billing">"Billing"</SelectItem>
                            <SelectItem value="owner">"Owner"</SelectItem>
                        </Select>
                    </li>

                    <li class="flex items-center gap-4">
                        <Avatar class="size-10">
                            <AvatarImage src="/avatar-2.png" />
                        </Avatar>

                        <div class="flex flex-col gap-1 mr-auto">
                            <p class="text-sm font-semibold leading-none">"Jackson Lee"</p>
                            <p class="text-sm text-(--muted-foreground)">"p@example.com"</p>
                        </div>

                        <Select>
                            <SelectItem value="empty">"Empty"</SelectItem>
                            <SelectItem value="viewer">"Viewer"</SelectItem>
                            <SelectItem value="developer">"Developer"</SelectItem>
                            <SelectItem value="billing">"Billing"</SelectItem>
                            <SelectItem value="owner">"Owner"</SelectItem>
                        </Select>
                    </li>

                    <li class="flex items-center gap-4">
                        <Avatar class="size-10">
                            <AvatarImage src="/avatar-3.png" />
                        </Avatar>

                        <div class="flex flex-col gap-1 mr-auto">
                            <p class="text-sm font-semibold leading-none">"Isabella Nguyen"</p>
                            <p class="text-sm text-(--muted-foreground)">"i@example.com"</p>
                        </div>

                        <Select default="viewer">
                            <SelectItem value="viewer">"Viewer"</SelectItem>
                            <SelectItem value="developer">"Developer"</SelectItem>
                            <SelectItem value="billing">"Billing"</SelectItem>
                            <SelectItem value="owner">"Owner"</SelectItem>
                        </Select>
                    </li>

                </ul>
            </CardContent>
        </Card>
    }
}
