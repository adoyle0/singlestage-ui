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
                            <AvatarImage src="/avatar-1.png" alt="Sofia Davis's Avatar" />
                            <AvatarFallback>"SD"</AvatarFallback>
                        </Avatar>

                        <div class="flex flex-col gap-1 mr-auto">
                            <p class="text-sm font-semibold leading-none">"Sofia Davis"</p>
                            <p class="text-sm text-(--muted-foreground)">"m@example.com"</p>
                        </div>

                        <Select class="w-fit" default="owner">
                            <SelectOption value="viewer">"Viewer"</SelectOption>
                            <SelectOption value="developer">"Developer"</SelectOption>
                            <SelectOption value="billing">"Billing"</SelectOption>
                            <SelectOption value="owner">"Owner"</SelectOption>
                        </Select>
                    </li>

                    <li class="flex items-center gap-4">
                        <Avatar class="size-10">
                            <AvatarImage src="/avatar-2.png" alt="Jackson Lee's Avatar" />
                            <AvatarFallback>"JL"</AvatarFallback>
                        </Avatar>

                        <div class="flex flex-col gap-1 mr-auto">
                            <p class="text-sm font-semibold leading-none">"Jackson Lee"</p>
                            <p class="text-sm text-(--muted-foreground)">"p@example.com"</p>
                        </div>

                        <Select class="w-fit">
                            <SelectOption value="empty">"Empty"</SelectOption>
                            <SelectOption value="viewer">"Viewer"</SelectOption>
                            <SelectOption value="developer">"Developer"</SelectOption>
                            <SelectOption value="billing">"Billing"</SelectOption>
                            <SelectOption value="owner">"Owner"</SelectOption>
                        </Select>
                    </li>

                    <li class="flex items-center gap-4">
                        <Avatar class="size-10">
                            <AvatarImage src="/avatar-3.png" alt="Isabella Nguyen's Avatar" />
                            <AvatarFallback>"IN"</AvatarFallback>
                        </Avatar>

                        <div class="flex flex-col gap-1 mr-auto">
                            <p class="text-sm font-semibold leading-none">"Isabella Nguyen"</p>
                            <p class="text-sm text-(--muted-foreground)">"i@example.com"</p>
                        </div>

                        <Select class="w-fit" default="viewer">
                            <SelectOption value="viewer">"Viewer"</SelectOption>
                            <SelectOption value="developer">"Developer"</SelectOption>
                            <SelectOption value="billing">"Billing"</SelectOption>
                            <SelectOption value="owner">"Owner"</SelectOption>
                        </Select>
                    </li>

                </ul>
            </CardContent>
        </Card>
    }
}
